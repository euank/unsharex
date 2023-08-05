use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn capget(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;
    fn capset(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;
    fn fd_is_procfs(fd: libc::c_int) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __u32 = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: libc::c_int,
}
pub type cap_user_header_t = *mut __user_cap_header_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
pub type cap_user_data_t = *mut __user_cap_data_struct;
unsafe extern "C" fn test_cap(cap: libc::c_uint) -> libc::c_int {
    return (prctl(
        23 as libc::c_int,
        cap,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn cap_last_by_bsearch(ret: *mut libc::c_int) -> libc::c_int {
    let mut cap: libc::c_int = 2147483647 as libc::c_int;
    let mut cap0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cap1: libc::c_uint = 2147483647 as libc::c_int as libc::c_uint;
    while (cap0 as libc::c_int) < cap {
        if test_cap(cap as libc::c_uint) != 0 {
            cap0 = cap as libc::c_uint;
        } else {
            cap1 = cap as libc::c_uint;
        }
        cap = cap0.wrapping_add(cap1).wrapping_div(2 as libc::c_uint) as libc::c_int;
    }
    *ret = cap;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cap_last_by_procfs(ret: *mut libc::c_int) -> libc::c_int {
    let f: *mut FILE = fopen(
        b"/proc/sys/kernel/cap_last_cap\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut rc: libc::c_int = -(22 as libc::c_int);
    *ret = 0 as libc::c_int;
    if !f.is_null() && fd_is_procfs(fileno(f)) != 0 {
        let mut cap: libc::c_int = 0;
        if fscanf(
            f,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut cap as *mut libc::c_int,
        ) == 1 as libc::c_int
            && cap < 2147483647 as libc::c_int
            && test_cap((cap + 1 as libc::c_int) as libc::c_uint) == 0
        {
            *ret = cap;
            rc = 0 as libc::c_int;
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn cap_last_cap() -> libc::c_int {
    static mut cap: libc::c_int = -(1 as libc::c_int);
    if cap != -(1 as libc::c_int) {
        return cap;
    }
    if cap_last_by_procfs(&mut cap) < 0 as libc::c_int {
        cap_last_by_bsearch(&mut cap);
    }
    return cap;
}
#[no_mangle]
pub unsafe extern "C" fn cap_permitted_to_ambient() {
    let mut header: __user_cap_header_struct = {
        let init = __user_cap_header_struct {
            version: 0x20080522 as libc::c_int as __u32,
            pid: 0 as libc::c_int,
        };
        init
    };
    let mut payload: [__user_cap_data_struct; 2] = [
        {
            let init = __user_cap_data_struct {
                effective: 0 as libc::c_int as __u32,
                permitted: 0,
                inheritable: 0,
            };
            init
        },
        __user_cap_data_struct {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        },
    ];
    let mut effective: uint64_t = 0;
    let mut cap: uint64_t = 0;
    if capget(&mut header, payload.as_mut_ptr()) < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"capget failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    payload[0 as libc::c_int as usize].inheritable = payload[0 as libc::c_int as usize].permitted;
    payload[1 as libc::c_int as usize].inheritable = payload[1 as libc::c_int as usize].permitted;
    if capset(&mut header, payload.as_mut_ptr()) < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"capset failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    effective = (payload[1 as libc::c_int as usize].effective as uint64_t) << 32 as libc::c_int
        | payload[0 as libc::c_int as usize].effective as uint64_t;
    cap = 0 as libc::c_int as uint64_t;
    while cap
        < (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        if !(cap > cap_last_cap() as uint64_t) {
            if effective as libc::c_ulonglong & (1 as libc::c_ulonglong) << cap != 0
                && prctl(
                    47 as libc::c_int,
                    2 as libc::c_int,
                    cap,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) < 0 as libc::c_int
            {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"prctl(PR_CAP_AMBIENT) failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        cap = cap.wrapping_add(1);
        cap;
    }
}
