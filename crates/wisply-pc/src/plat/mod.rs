use crate::model::Sample;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(any(not(windows), test))]
pub mod parse;
pub mod shell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Levels {
    pub lowest: i32,
    pub low: i32,
    pub boost: i32,
}

pub trait Platform: Send {
    fn sample(&mut self) -> Sample;
    fn levels(&self) -> Levels;
    fn priority(&mut self, pid: u32) -> Option<i32>;
    fn set_priority(&mut self, pid: u32, level: i32) -> bool;
    fn suspend(&mut self, pid: u32) -> bool;
    fn resume(&mut self, pid: u32) -> bool;
    fn trim(&mut self, pid: u32) -> bool;
    fn kill(&mut self, pid: u32) -> bool;
    fn power_plan(&mut self) -> Option<String>;
    fn fast_power_plan(&mut self) -> Option<String>;
    fn set_power_plan(&mut self, id: &str) -> bool;
}

#[cfg(windows)]
pub fn system() -> Box<dyn Platform> {
    Box::new(windows::Windows::new())
}

#[cfg(target_os = "linux")]
pub fn system() -> Box<dyn Platform> {
    Box::new(linux::Linux::new())
}

#[cfg(target_os = "macos")]
pub fn system() -> Box<dyn Platform> {
    Box::new(mac::Mac::new())
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
pub fn system() -> Box<dyn Platform> {
    Box::new(crate::fake::Fake::default())
}
