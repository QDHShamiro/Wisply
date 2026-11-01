#[cfg(windows)]
const TITLE: &str = "Choose a folder";
#[cfg(windows)]
const OK_LABEL: &str = "Select folder";

pub fn pick(start: Option<&str>) -> (i32, String) {
    match choose(start.filter(|s| !s.trim().is_empty())) {
        Some(path) => (0, path),
        None => (1, String::new()),
    }
}

#[cfg(windows)]
fn choose(start: Option<&str>) -> Option<String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize,
    };
    use windows::Win32::UI::Shell::{
        FOS_FORCEFILESYSTEM, FOS_PICKFOLDERS, FileOpenDialog, IFileOpenDialog, IShellItem, SHCreateItemFromParsingName,
        SIGDN_FILESYSPATH,
    };
    use windows::core::HSTRING;

    unsafe {
        let init = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let out = (|| {
            let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER).ok()?;
            let options = dialog.GetOptions().ok()?;
            dialog.SetOptions(options | FOS_PICKFOLDERS | FOS_FORCEFILESYSTEM).ok()?;
            let _ = dialog.SetTitle(&HSTRING::from(TITLE));
            let _ = dialog.SetOkButtonLabel(&HSTRING::from(OK_LABEL));
            if let Some(dir) = start
                && let Ok(item) = SHCreateItemFromParsingName::<_, _, IShellItem>(&HSTRING::from(dir), None)
            {
                let _ = dialog.SetFolder(&item);
            }
            let owner = super::win::owner();
            dialog.Show((owner != 0).then_some(HWND(owner as *mut _))).ok()?;
            let item = dialog.GetResult().ok()?;
            let name = item.GetDisplayName(SIGDN_FILESYSPATH).ok()?;
            let path = name.to_string().ok();
            CoTaskMemFree(Some(name.0 as *const _));
            path
        })();
        if init.is_ok() {
            CoUninitialize();
        }
        out
    }
}

#[cfg(not(windows))]
fn choose(_start: Option<&str>) -> Option<String> {
    None
}
