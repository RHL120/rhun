use std::{
    env,
    ffi::{CStr, CString},
    fs::File,
    mem::MaybeUninit,
    os::unix::prelude::AsRawFd,
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

pub fn update_pass_time(username: &str) -> Option<()> {
    std::fs::File::create(format!("/tmp/runas_timestamp_{}", username))
        .ok()
        .map(|_| ())
}

pub fn check_pass_time(username: &str) -> Option<bool> {
    let file = match std::fs::File::open(format!("/tmp/runas_timestamp_{}", username)) {
        Ok(x) => x,
        Err(x) => {
            return if x.kind() == std::io::ErrorKind::NotFound {
                Some(false)
            } else {
                None
            }
        }
    };
    let md = file.metadata().ok()?;
    Some(
        owner_is_root(&file)?
            && md.modified().ok()?.elapsed().ok()? < std::time::Duration::from_secs(300),
    )
}

pub fn check_password(username: &str, passwd: &str) -> Option<bool> {
    let pass = unsafe { libc::getspnam(CString::new(username).ok()?.as_ptr()).as_ref()? };
    Some(unsafe {
        CStr::from_ptr(pass.sp_pwdp)
            == CStr::from_ptr(crypt(CString::new(passwd).ok()?.as_ptr(), pass.sp_pwdp))
    })
}

pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn owner_is_root(f: &File) -> Option<bool> {
    let fd = f.as_raw_fd();
    unsafe {
        let mut stat: MaybeUninit<libc::stat> = std::mem::MaybeUninit::uninit();
        if libc::fstat(fd, stat.as_mut_ptr()) != 0 {
            None
        } else {
            let stat = stat.assume_init_ref();
            Some(stat.st_uid == 0 && stat.st_gid == 0)
        }
    }
}
