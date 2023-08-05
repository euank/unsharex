use ::libc;
extern "C" {
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct identry {
    pub id: libc::c_ulong,
    pub name: *mut libc::c_char,
    pub next: *mut identry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idcache {
    pub ent: *mut identry,
    pub width: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn get_id(ic: *mut idcache, id: libc::c_ulong) -> *mut identry {
    let mut ent: *mut identry = 0 as *mut identry;
    if ic.is_null() {
        return 0 as *mut identry;
    }
    ent = (*ic).ent;
    while !ent.is_null() {
        if (*ent).id == id {
            return ent;
        }
        ent = (*ent).next;
    }
    return 0 as *mut identry;
}
#[no_mangle]
pub unsafe extern "C" fn new_idcache() -> *mut idcache {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<idcache>() as libc::c_ulong,
    ) as *mut idcache;
}
#[no_mangle]
pub unsafe extern "C" fn free_idcache(ic: *mut idcache) {
    let mut ent: *mut identry = (*ic).ent;
    while !ent.is_null() {
        let next: *mut identry = (*ent).next;
        free((*ent).name as *mut libc::c_void);
        free(ent as *mut libc::c_void);
        ent = next;
    }
    free(ic as *mut libc::c_void);
}
unsafe extern "C" fn add_id(ic: *mut idcache, name: *mut libc::c_char, id: libc::c_ulong) {
    let mut ent: *mut identry = 0 as *mut identry;
    let mut x: *mut identry = 0 as *mut identry;
    let mut w: libc::c_int = 0 as libc::c_int;
    ent = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<identry>() as libc::c_ulong,
    ) as *mut identry;
    if ent.is_null() {
        return;
    }
    (*ent).id = id;
    if !name.is_null() {
        let mut wc: [wchar_t; 257] = [0; 257];
        if mbstowcs(wc.as_mut_ptr(), name, 256 as libc::c_int as size_t)
            > 0 as libc::c_int as libc::c_ulong
        {
            wc[256 as libc::c_int as usize] = '\0' as i32;
            w = wcswidth(wc.as_mut_ptr(), 256 as libc::c_int as size_t);
        } else {
            w = strlen(name) as libc::c_int;
        }
    }
    if w > 0 as libc::c_int {
        (*ent).name = strdup(name);
        if ((*ent).name).is_null() {
            free(ent as *mut libc::c_void);
            return;
        }
    } else if asprintf(
        &mut (*ent).name as *mut *mut libc::c_char,
        b"%lu\0" as *const u8 as *const libc::c_char,
        id,
    ) < 0 as libc::c_int
    {
        free(ent as *mut libc::c_void);
        return;
    }
    x = (*ic).ent;
    while !x.is_null() && !((*x).next).is_null() {
        x = (*x).next;
    }
    if !x.is_null() {
        (*x).next = ent;
    } else {
        (*ic).ent = ent;
    }
    if w <= 0 as libc::c_int {
        w = (if !((*ent).name).is_null() {
            strlen((*ent).name)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
    }
    (*ic).width = if (*ic).width < w { w } else { (*ic).width };
}
#[no_mangle]
pub unsafe extern "C" fn add_uid(cache: *mut idcache, id: libc::c_ulong) {
    let ent: *mut identry = get_id(cache, id);
    if ent.is_null() {
        let pw: *mut passwd = getpwuid(id as uid_t);
        add_id(
            cache,
            if !pw.is_null() {
                (*pw).pw_name
            } else {
                0 as *mut libc::c_char
            },
            id,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_gid(cache: *mut idcache, id: libc::c_ulong) {
    let ent: *mut identry = get_id(cache, id);
    if ent.is_null() {
        let gr: *mut group = getgrgid(id as gid_t);
        add_id(
            cache,
            if !gr.is_null() {
                (*gr).gr_name
            } else {
                0 as *mut libc::c_char
            },
            id,
        );
    }
}
