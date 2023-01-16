use std::{ffi::{CStr, CString}, env};

extern "C" {
     fn readpassphrase(prompt: *const i8, buf: *mut i8, bufsize: usize, flags: i32)-> *mut i8;
}

pub fn get_username() -> Option<String> {
    let name = unsafe { CStr::from_ptr(libc::getpwuid(libc::getuid()).as_ref()?.pw_name) };
    name.to_str().ok().map(|x| x.to_string())
}

pub fn find_bin(exec: &str) -> Option<String> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|x| {
        let path = x.join(exec);
        match path.is_file() {
            true => Some(std::fs::canonicalize(path).ok()?.into_os_string().into_string().ok()?),
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
