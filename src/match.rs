use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn match_fstype(
    mut type_0: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut no: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if pattern.is_null() && type_0.is_null() {
        return 1 as libc::c_int;
    }
    if pattern.is_null() {
        return 0 as libc::c_int;
    }
    if strncmp(
        pattern,
        b"no\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        no = 1 as libc::c_int;
        pattern = pattern.offset(2 as libc::c_int as isize);
    }
    len = strlen(type_0) as libc::c_int;
    p = pattern;
    loop {
        if strncmp(
            p,
            b"no\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
            && strncasecmp(
                p.offset(2 as libc::c_int as isize),
                type_0,
                len as libc::c_ulong,
            ) == 0
            && (*p.offset((len + 2 as libc::c_int) as isize) as libc::c_int
                == 0 as libc::c_int
                || *p.offset((len + 2 as libc::c_int) as isize) as libc::c_int
                    == ',' as i32)
        {
            return 0 as libc::c_int;
        }
        if strncasecmp(p, type_0, len as libc::c_ulong) == 0 as libc::c_int
            && (*p.offset(len as isize) as libc::c_int == 0 as libc::c_int
                || *p.offset(len as isize) as libc::c_int == ',' as i32)
        {
            return (no == 0) as libc::c_int;
        }
        p = strchr(p, ',' as i32);
        if p.is_null() {
            break;
        }
        p = p.offset(1);
        p;
    }
    return no;
}
