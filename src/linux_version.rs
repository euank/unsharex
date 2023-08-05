use ::libc;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[no_mangle]
pub unsafe extern "C" fn get_linux_version() -> libc::c_int {
    static mut kver: libc::c_int = -(1 as libc::c_int);
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    if kver != -(1 as libc::c_int) {
        return kver;
    }
    if uname(&mut uts) != 0 {
        kver = 0 as libc::c_int;
        return kver;
    }
    n = sscanf(
        (uts.release).as_mut_ptr(),
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_int,
        &mut y as *mut libc::c_int,
        &mut z as *mut libc::c_int,
    );
    if n < 1 as libc::c_int || n > 3 as libc::c_int {
        kver = 0 as libc::c_int;
        return kver;
    }
    kver = (x << 16 as libc::c_int) + (y << 8 as libc::c_int)
        + (if z > 255 as libc::c_int { 255 as libc::c_int } else { z });
    return kver;
}
