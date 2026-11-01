use std::sync::atomic::{AtomicIsize, Ordering};

static OWNER: AtomicIsize = AtomicIsize::new(0);

pub const THEMES: &[&str] = &["light", "dark", "glass"];

pub fn set_owner(hwnd: isize) {
    OWNER.store(hwnd, Ordering::Relaxed);
}

pub fn owner() -> isize {
    OWNER.load(Ordering::Relaxed)
}

pub fn theme(name: Option<&str>) -> (i32, String) {
    match name.filter(|n| THEMES.contains(n)) {
        Some(n) => {
            paint(n);
            (0, n.to_string())
        }
        None => (2, format!("err: theme must be one of {}", THEMES.join(", "))),
    }
}

#[cfg(windows)]
fn paint(name: &str) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Dwm::{
        DWMSBT_NONE, DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE,
        DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
    };
    use windows::Win32::UI::Controls::MARGINS;
    use windows::core::BOOL;

    let raw = owner();
    if raw == 0 {
        return;
    }
    let hwnd = HWND(raw as *mut _);
    let dark = BOOL::from(name != "light");
    let glass = name == "glass";
    let backdrop = if glass { DWMSBT_TRANSIENTWINDOW } else { DWMSBT_NONE };
    let edge = if glass { -1 } else { 0 };
    let margins = MARGINS { cxLeftWidth: edge, cxRightWidth: edge, cyTopHeight: edge, cyBottomHeight: edge };
    unsafe {
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            (&raw const dark).cast(),
            size_of::<BOOL>() as u32,
        );
        let _ = DwmExtendFrameIntoClientArea(hwnd, &raw const margins);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            (&raw const backdrop).cast(),
            size_of_val(&backdrop) as u32,
        );
    }
}

#[cfg(not(windows))]
fn paint(_name: &str) {}

#[cfg(test)]
mod tests {
    #[test]
    fn only_known_themes_are_applied() {
        assert_eq!(super::theme(Some("glass")), (0, "glass".into()));
        assert_eq!(super::theme(Some("neon")).0, 2);
        assert_eq!(super::theme(None).0, 2);
    }
}
