use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicIsize, AtomicU8, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreateSolidBrush, DT_LEFT, DT_NOPREFIX, DT_SINGLELINE, DT_VCENTER, DeleteObject,
    DrawTextW, EndPaint, FW_SEMIBOLD, FillRect, InvalidateRect, PAINTSTRUCT, SelectObject, SetBkMode, SetTextColor,
    TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyState, GetKeyboardLayout, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, RegisterHotKey, ToUnicodeEx,
    VK_SPACE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetForegroundWindow, GetMessageW,
    GetSystemMetrics, GetWindowThreadProcessId, HHOOK, KBDLLHOOKSTRUCT, KillTimer, LWA_ALPHA, MSG, PostThreadMessageW,
    RegisterClassW, SM_CXSCREEN, SM_CYSCREEN, SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER,
    SetLayeredWindowAttributes, SetTimer, SetWindowPos, SetWindowsHookExW, ShowWindow, UnhookWindowsHookEx,
    WH_KEYBOARD_LL, WM_APP, WM_HOTKEY, WM_KEYDOWN, WM_PAINT, WM_SYSKEYDOWN, WM_TIMER, WNDCLASSW, WS_EX_LAYERED,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use windows::core::{PCWSTR, w};

use super::super::desk::clip_get;
use super::super::media::transcribe;
use super::assist::{self, VK_DOWN, VK_TAB, VK_UP};
use super::draw::{self, BOX_H, BOX_W, TIP_H};
use super::later::spawn_go;
use crate::voice::{ASR_RATE, AUDIO_RATE, AudioKind, clean, record, wav};

pub const NAME: &str = "Ctrl+Alt+Space";
const HOTKEY_ID: i32 = 0x60;
const NOTE: &str = "! ";
const RETRY: Duration = Duration::from_secs(5);
const HOLD: Duration = Duration::from_millis(350);
const HOLD_POLL: Duration = Duration::from_millis(15);
const MAX_TALK: Duration = Duration::from_secs(20);
const MIN_TALK_S: f32 = 0.3;
const VOICE_LANG: &str = "de";
const SHOW_MS: u32 = 4500;
const IDLE_MS: u32 = 60_000;
const T_CLOSE: usize = 1;
const T_IDLE: usize = 2;
const M_SUBMIT: u32 = WM_APP + 1;
const M_CANCEL: u32 = WM_APP + 2;
const M_DONE: u32 = WM_APP + 3;
const M_PASTE: u32 = WM_APP + 4;
const KEY_INJECTED: u32 = 0x10;
const BOX_TOP_DIV: i32 = 5;
const ALPHA: u8 = 240;
const SHOWN_CHARS: usize = 72;
const LINE_CHARS: usize = 160;
const FG: u32 = 0x00F2_F2F2;
const DIM: u32 = 0x0090_9090;
const RED: u32 = 0x0050_50F0;
const GREEN: u32 = 0x0070_D070;
const MODS: &[u32] = &[0x10, 0x11, 0x12, 0x14, 0x5B, 0x5C, 0x90, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5];
const VK_BACK: u32 = 0x08;
const VK_RETURN: u32 = 0x0D;
const VK_ESCAPE: u32 = 0x1B;
const VK_V: u32 = 0x56;
const OK_MARK: &str = "\u{2713}";
const BAD_MARK: &str = "\u{2717}";
const HINT: &str = "type a command \u{00B7} Enter runs it \u{00B7} Esc closes";
const LISTENING: &str = "\u{25CF} listening \u{2014} let go to send";

const CLOSED: u8 = 0;
const TYPING: u8 = 1;
const LISTEN: u8 = 2;
const BUSY: u8 = 3;
const SHOWN: u8 = 4;

static MODE: AtomicU8 = AtomicU8::new(CLOSED);
static WIN: AtomicIsize = AtomicIsize::new(0);
static HOOK: AtomicIsize = AtomicIsize::new(0);
static TID: AtomicU32 = AtomicU32::new(0);
static TEXT: Mutex<String> = Mutex::new(String::new());
static GOOD: AtomicU8 = AtomicU8::new(1);
static HOME: OnceLock<PathBuf> = OnceLock::new();

fn text() -> std::sync::MutexGuard<'static, String> {
    TEXT.lock().unwrap_or_else(PoisonError::into_inner)
}

fn set_text(s: &str) {
    *text() = s.to_string();
    redraw();
}

fn win() -> Option<HWND> {
    let v = WIN.load(Ordering::SeqCst);
    (v != 0).then_some(HWND(v as *mut _))
}

fn redraw() {
    if let Some(h) = win() {
        let _ = unsafe { InvalidateRect(Some(h), None, true) };
    }
}

fn post(msg: u32) {
    let _ = unsafe { PostThreadMessageW(TID.load(Ordering::SeqCst), msg, WPARAM(0), LPARAM(0)) };
}

fn down(vk: i32) -> bool {
    unsafe { GetAsyncKeyState(vk) as u16 & 0x8000 != 0 }
}

fn typed(vk: u32, scan: u32) -> String {
    let mut state = [0u8; 256];
    for key in [0x10usize, 0x11, 0x12] {
        if down(key as i32) {
            state[key] = 0x80;
        }
    }
    if unsafe { GetKeyState(0x14) } & 1 != 0 {
        state[0x14] = 0x01;
    }
    let hkl = unsafe { GetKeyboardLayout(GetWindowThreadProcessId(GetForegroundWindow(), None)) };
    let mut buf = [0u16; 8];
    let n = unsafe { ToUnicodeEx(vk, scan, &state, &mut buf, 0x4, Some(hkl)) };
    if n > 0 { String::from_utf16_lossy(&buf[..n as usize]) } else { String::new() }
}

fn box_h() -> i32 {
    if MODE.load(Ordering::SeqCst) == TYPING { BOX_H + TIP_H } else { BOX_H }
}

fn ask() {
    if let Some(home) = HOME.get() {
        assist::ask(home, &text(), redraw);
    }
}

fn shrink() {
    if let Some(h) = win() {
        let _ = unsafe { SetWindowPos(h, None, 0, 0, BOX_W, box_h(), SWP_NOMOVE | SWP_NOACTIVATE | SWP_NOZORDER) };
    }
}

fn idle_again() {
    if let Some(h) = win() {
        unsafe { SetTimer(Some(h), T_IDLE, IDLE_MS, None) };
    }
}

fn key(vk: u32, scan: u32) {
    let ctrl = down(0x11);
    let alt = down(0x12);
    match vk {
        VK_RETURN => return post(M_SUBMIT),
        VK_ESCAPE => return post(M_CANCEL),
        VK_V if ctrl && !alt => return post(M_PASTE),
        VK_BACK => {
            let mut t = text();
            if ctrl {
                let kept = t.trim_end().rfind(' ').map_or(0, |i| i + 1);
                t.truncate(kept);
            } else {
                t.pop();
            }
            assist::typed();
        }
        VK_UP | VK_DOWN | VK_TAB => assist::nav(vk, &mut text()),
        _ => {
            let add: String = typed(vk, scan).chars().filter(|c| !c.is_control()).collect();
            text().push_str(&add);
            assist::typed();
        }
    }
    idle_again();
    redraw();
    ask();
}

unsafe extern "system" fn on_key(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if code == 0 {
        let info = unsafe { &*(lp.0 as *const KBDLLHOOKSTRUCT) };
        let pressed = matches!(wp.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN);
        if info.flags.0 & KEY_INJECTED == 0 && !MODS.contains(&info.vkCode) {
            match MODE.load(Ordering::SeqCst) {
                TYPING => {
                    if pressed {
                        key(info.vkCode, info.scanCode);
                    }
                    return LRESULT(1);
                }
                LISTEN if info.vkCode == u32::from(VK_SPACE.0) => return LRESULT(1),
                _ => {}
            }
        }
    }
    unsafe { CallNextHookEx(None, code, wp, lp) }
}

fn shown() -> (String, u32) {
    let t = text().clone();
    match MODE.load(Ordering::SeqCst) {
        TYPING if t.is_empty() => (HINT.to_string(), DIM),
        TYPING => {
            let n = t.chars().count();
            let tail: String = t.chars().skip(n.saturating_sub(SHOWN_CHARS)).collect();
            (format!("\u{203A} {}{tail}\u{258F}", if n > SHOWN_CHARS { "\u{2026}" } else { "" }), FG)
        }
        LISTEN => (LISTENING.to_string(), RED),
        BUSY => (format!("\u{2026} {t}"), DIM),
        _ if GOOD.load(Ordering::SeqCst) == 1 => (t, GREEN),
        _ => (t, RED),
    }
}

unsafe extern "system" fn paint(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg != WM_PAINT {
        return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
    }
    let (main, color) = shown();
    let tip = if MODE.load(Ordering::SeqCst) == TYPING { assist::line_for(&text()) } else { String::new() };
    draw::paint(hwnd, &draw::Lines { main: &main, color, tip: &tip, height: box_h() });
    LRESULT(0)
}

fn register_class() {
    unsafe {
        let Ok(inst) = GetModuleHandleW(None) else { return };
        let wc = WNDCLASSW {
            lpfnWndProc: Some(paint),
            hInstance: inst.into(),
            lpszClassName: w!("WisplyGoBox"),
            ..Default::default()
        };
        RegisterClassW(&raw const wc);
    }
}

fn open_box() -> bool {
    unsafe {
        let Ok(inst) = GetModuleHandleW(None) else { return false };
        let x = (GetSystemMetrics(SM_CXSCREEN) - BOX_W) / 2;
        let y = GetSystemMetrics(SM_CYSCREEN) / BOX_TOP_DIV;
        let ex = WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED;
        let Ok(h) = CreateWindowExW(
            ex,
            w!("WisplyGoBox"),
            PCWSTR::null(),
            WS_POPUP,
            x,
            y,
            BOX_W,
            box_h(),
            None,
            None,
            Some(inst.into()),
            None,
        ) else {
            return false;
        };
        let _ = SetLayeredWindowAttributes(h, COLORREF(0), ALPHA, LWA_ALPHA);
        WIN.store(h.0 as isize, Ordering::SeqCst);
        let _ = ShowWindow(h, SW_SHOWNOACTIVATE);
        let inst = GetModuleHandleW(None).ok();
        if let Ok(hook) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(on_key), inst.map(Into::into), 0) {
            HOOK.store(hook.0 as isize, Ordering::SeqCst);
        }
    }
    true
}

fn unhook() {
    let h = HOOK.swap(0, Ordering::SeqCst);
    if h != 0 {
        let _ = unsafe { UnhookWindowsHookEx(HHOOK(h as *mut _)) };
    }
}

fn close() {
    unhook();
    MODE.store(CLOSED, Ordering::SeqCst);
    text().clear();
    assist::clear();
    let h = WIN.swap(0, Ordering::SeqCst);
    if h != 0 {
        let h = HWND(h as *mut _);
        unsafe {
            let _ = KillTimer(Some(h), T_CLOSE);
            let _ = KillTimer(Some(h), T_IDLE);
            let _ = DestroyWindow(h);
        }
    }
}

fn finish(ok: bool, line: &str) {
    GOOD.store(u8::from(ok), Ordering::SeqCst);
    let mark = if ok { OK_MARK } else { BAD_MARK };
    let line: String = line.chars().take(LINE_CHARS).collect();
    *text() = format!("{mark} {line}");
    post(M_DONE);
}

fn run_go(home: &Path, said: &str) {
    let out = spawn_go(home, said, &[]).and_then(|c| {
        c.wait_with_output().map_err(|e| wisply_core::Fail::new(format!("wisply go did not finish ({e})")))
    });
    match out {
        Ok(o) => {
            let ok = o.status.success();
            let raw =
                String::from_utf8_lossy(if ok || o.stderr.is_empty() { &o.stdout } else { &o.stderr }).to_string();
            let line = raw
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with(NOTE))
                .take(2)
                .collect::<Vec<_>>()
                .join(" \u{00B7} ");
            finish(ok, &line);
        }
        Err(f) => finish(false, &f.text()),
    }
}

fn submit(home: &Path) {
    unhook();
    if let Some(h) = win() {
        let _ = unsafe { KillTimer(Some(h), T_IDLE) };
    }
    let said = text().trim().to_string();
    if said.is_empty() {
        return close();
    }
    MODE.store(BUSY, Ordering::SeqCst);
    assist::remember(home, &said);
    assist::clear();
    shrink();
    set_text(&said);
    let home = home.to_path_buf();
    let _ = thread::Builder::new().name("go-box".into()).spawn(move || run_go(&home, &said));
}

fn heard(home: &Path) -> Result<String, String> {
    let mut mono: Vec<f32> = Vec::new();
    let t0 = Instant::now();
    record(&AudioKind::Mic, "", 2, |block| {
        mono.extend(block.as_chunks::<2>().0.iter().map(|[l, r]| (l + r) / 2.0));
        down(i32::from(VK_SPACE.0)) && t0.elapsed() < MAX_TALK
    })?;
    if (mono.len() as f32) < AUDIO_RATE as f32 * MIN_TALK_S {
        return Err("hold the keys while you talk".into());
    }
    let step = (AUDIO_RATE / ASR_RATE).max(1) as usize;
    let pcm: Vec<u8> = mono
        .chunks(step)
        .flat_map(|c| {
            let v = c.iter().sum::<f32>() / c.len() as f32;
            ((v.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16).to_le_bytes()
        })
        .collect();
    let path = env::temp_dir().join(format!("wisply-go-{}.wav", std::process::id()));
    fs::write(&path, wav(&pcm)).map_err(|e| format!("cannot write the recording ({e})"))?;
    let result = transcribe(home, &path, Some(VOICE_LANG), false);
    let _ = fs::remove_file(&path);
    let said = result.map_err(|f| f.text())?.segs.iter().map(|(_, t)| t.trim()).collect::<Vec<_>>().join(" ");
    let said = clean(said.trim(), "");
    if said.is_empty() { Err("heard nothing".into()) } else { Ok(said) }
}

fn listen(home: &Path) {
    MODE.store(LISTEN, Ordering::SeqCst);
    redraw();
    let home = home.to_path_buf();
    let _ = thread::Builder::new().name("go-voice".into()).spawn(move || match heard(&home) {
        Ok(said) => {
            MODE.store(BUSY, Ordering::SeqCst);
            set_text(&said);
            run_go(&home, &said);
        }
        Err(why) => finish(false, &why),
    });
}

fn on_hotkey(home: &Path) {
    match MODE.load(Ordering::SeqCst) {
        TYPING => return close(),
        SHOWN => close(),
        CLOSED => {}
        _ => return,
    }
    let t0 = Instant::now();
    let held = loop {
        if !down(i32::from(VK_SPACE.0)) {
            break false;
        }
        if t0.elapsed() >= HOLD {
            break true;
        }
        thread::sleep(HOLD_POLL);
    };
    text().clear();
    if !held {
        assist::open(home);
    }
    MODE.store(if held { LISTEN } else { TYPING }, Ordering::SeqCst);
    if !open_box() {
        MODE.store(CLOSED, Ordering::SeqCst);
        return;
    }
    if held {
        listen(home);
    } else {
        idle_again();
    }
}

fn paste() {
    if MODE.load(Ordering::SeqCst) != TYPING {
        return;
    }
    if let Some(clip) = clip_get() {
        let one: String = clip.split_whitespace().collect::<Vec<_>>().join(" ");
        text().push_str(&one);
        assist::typed();
        idle_again();
        redraw();
        ask();
    }
}

fn shown_done() {
    MODE.store(SHOWN, Ordering::SeqCst);
    redraw();
    if let Some(h) = win() {
        unsafe { SetTimer(Some(h), T_CLOSE, SHOW_MS, None) };
    }
}

fn run(home: &Path) {
    while unsafe { RegisterHotKey(None, HOTKEY_ID, MOD_CONTROL | MOD_ALT | MOD_NOREPEAT, u32::from(VK_SPACE.0)) }
        .is_err()
    {
        thread::sleep(RETRY);
    }
    TID.store(unsafe { GetCurrentThreadId() }, Ordering::SeqCst);
    register_class();
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&raw mut msg, None, 0, 0) }.0 > 0 {
        match msg.message {
            WM_HOTKEY if msg.wParam.0 == HOTKEY_ID as usize => on_hotkey(home),
            M_SUBMIT => submit(home),
            M_CANCEL => close(),
            M_DONE => shown_done(),
            M_PASTE => paste(),
            WM_TIMER if msg.wParam.0 == T_CLOSE || msg.wParam.0 == T_IDLE => close(),
            _ => unsafe {
                DispatchMessageW(&raw const msg);
            },
        }
    }
}

pub fn start(home: PathBuf) {
    if HOME.set(home).is_err() {
        return;
    }
    let _ = thread::Builder::new().name("go-input".into()).spawn(|| {
        if let Some(home) = HOME.get() {
            run(home);
        }
    });
}
