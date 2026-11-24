use windows::Win32::Foundation::{COLORREF, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreateSolidBrush, DT_LEFT, DT_NOPREFIX, DT_SINGLELINE, DT_VCENTER, DeleteObject,
    DrawTextW, EndPaint, FW_NORMAL, FW_SEMIBOLD, FillRect, HDC, PAINTSTRUCT, SelectObject, SetBkMode, SetTextColor,
    TRANSPARENT,
};
use windows::core::w;

pub const BOX_W: i32 = 680;
pub const BOX_H: i32 = 46;
pub const TIP_H: i32 = 26;
const TIP_UP: i32 = 6;
const TIP_PX: i32 = 15;
const TIP_COLOR: u32 = 0x0080_8080;
const PAD: i32 = 16;
const FONT_PX: i32 = 19;
const BG: u32 = 0x0020_1C1C;

pub struct Lines<'a> {
    pub main: &'a str,
    pub color: u32,
    pub tip: &'a str,
    pub height: i32,
}

unsafe fn text_line(dc: HDC, line: &str, color: u32, (px, weight): (i32, i32), mut area: RECT) {
    unsafe {
        let font = CreateFontW(
            px,
            0,
            0,
            0,
            weight,
            0,
            0,
            0,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            0,
            w!("Segoe UI"),
        );
        let old = SelectObject(dc, font.into());
        SetTextColor(dc, COLORREF(color));
        let mut wide: Vec<u16> = line.encode_utf16().collect();
        DrawTextW(dc, &mut wide, &raw mut area, DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX);
        SelectObject(dc, old);
        let _ = DeleteObject(font.into());
    }
}

pub fn paint(hwnd: HWND, lines: &Lines) {
    unsafe {
        let mut ps = PAINTSTRUCT::default();
        let dc = BeginPaint(hwnd, &raw mut ps);
        let brush = CreateSolidBrush(COLORREF(BG));
        let full = RECT { left: 0, top: 0, right: BOX_W, bottom: lines.height };
        FillRect(dc, &raw const full, brush);
        SetBkMode(dc, TRANSPARENT);
        let first = RECT { left: PAD, top: 0, right: BOX_W - PAD, bottom: BOX_H };
        text_line(dc, lines.main, lines.color, (FONT_PX, FW_SEMIBOLD.0 as i32), first);
        if !lines.tip.is_empty() {
            let tip = RECT { left: PAD, top: BOX_H - TIP_UP, right: BOX_W - PAD, bottom: BOX_H + TIP_H - TIP_UP };
            text_line(dc, lines.tip, TIP_COLOR, (TIP_PX, FW_NORMAL.0 as i32), tip);
        }
        let _ = DeleteObject(brush.into());
        let _ = EndPaint(hwnd, &raw const ps);
    }
}
