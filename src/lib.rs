use std::{
    env,
    ffi::{CStr, CString},
};

extern "C" {
    fn readpassphrase(prompt: *const i8, buf: *mut i8, bufsize: usize, flags: i32) -> *mut i8;
    fn crypt(phrase: *const i8, setting: *const i8) -> *mut i8;
}

pub fn get_username() -> Option<String> {
    unsafe { CStr::from_ptr(libc::getlogin()) }
        .to_str()
        .ok()
        .map(|x| x.to_string())
}

pub fn find_bin(exec: &str) -> Option<String> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|x| {
        let path = x.join(exec);
        match path.is_file() {
            true => Some(path.display().to_string()),
            false => None,
        }
    })
}

pub fn read_password(prompt: &str) -> Option<String> {
    unsafe {
        let buf: *mut i8 = libc::malloc(72).cast();
        readpassphrase(CString::new(prompt).ok()?.as_ptr(), buf, 72, 0);
        let ret = CStr::from_ptr(buf).to_str().ok().map(|x| x.to_string());
        libc::free(buf.cast());
        ret
    }
}

pub fn check_password(username: &str, passwd: &str) -> Option<bool> {
    let pass = unsafe { libc::getspnam(CString::new(username).ok()?.as_ptr()).as_ref()? };
    Some(unsafe {
        CStr::from_ptr(pass.sp_pwdp)
            == CStr::from_ptr(crypt(CString::new(passwd).ok()?.as_ptr(), pass.sp_pwdp))
    })
}

pub fn is_root() -> bool {
    unsafe {libc::geteuid() == 0}
}
