use std::ffi::CStr;
use std::fs::File;
use std::io::BufRead;
pub fn get_username() -> Option<String> {
    let name = unsafe { CStr::from_ptr(libc::getpwuid(libc::getuid()).as_ref()?.pw_name) };
    name.to_str().ok().map(|x| x.to_string())
}
