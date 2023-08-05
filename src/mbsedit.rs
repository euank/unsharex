use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mbs_safe_width(s: *const libc::c_char) -> size_t;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbs_editor {
    pub buf: *mut libc::c_char,
    pub max_bytes: size_t,
    pub max_cells: size_t,
    pub cur_cells: size_t,
    pub cur_bytes: size_t,
    pub cursor: size_t,
    pub cursor_cells: size_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MBS_EDIT_HOME: C2RustUnnamed_0 = 3;
pub const MBS_EDIT_END: C2RustUnnamed_0 = 2;
pub const MBS_EDIT_RIGHT: C2RustUnnamed_0 = 1;
pub const MBS_EDIT_LEFT: C2RustUnnamed_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn mbs_new_edit(
    buf: *mut libc::c_char,
    bufsz: size_t,
    ncells: size_t,
) -> *mut mbs_editor {
    let edit: *mut mbs_editor = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mbs_editor>() as libc::c_ulong,
    ) as *mut mbs_editor;
    if !edit.is_null() {
        (*edit).buf = buf;
        (*edit).max_bytes = bufsz;
        (*edit).max_cells = ncells;
        (*edit).cur_cells = mbs_safe_width(buf);
        (*edit).cur_bytes = strlen(buf);
    }
    return edit;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_free_edit(edit: *mut mbs_editor) -> *mut libc::c_char {
    let ret: *mut libc::c_char = if !edit.is_null() {
        (*edit).buf
    } else {
        0 as *mut libc::c_char
    };
    free(edit as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn mbs_next(
    str: *const libc::c_char,
    ncells: *mut size_t,
) -> size_t {
    let mut wc: wchar_t = 0;
    let mut n: size_t = 0 as libc::c_int as size_t;
    if str.is_null() || *str == 0 {
        return 0 as libc::c_int as size_t;
    }
    n = mbrtowc(&mut wc, str, __ctype_get_mb_cur_max(), 0 as *mut mbstate_t);
    *ncells = wcwidth(wc) as size_t;
    return n;
}
unsafe extern "C" fn mbs_prev(
    start: *const libc::c_char,
    end: *const libc::c_char,
    ncells: *mut size_t,
) -> size_t {
    let mut wc: wchar_t = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut prev: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    if start.is_null() || end.is_null() || start == end || *start == 0 {
        return 0 as libc::c_int as size_t;
    }
    p = start;
    prev = p;
    while p < end {
        n = mbrtowc(&mut wc, p, __ctype_get_mb_cur_max(), 0 as *mut mbstate_t);
        prev = p;
        if n == -(1 as libc::c_int) as size_t || n == -(2 as libc::c_int) as size_t {
            p = p.offset(1);
            p;
        } else {
            p = p.offset(n as isize);
        }
    }
    if prev == end {
        return 0 as libc::c_int as size_t;
    }
    *ncells = wcwidth(wc) as size_t;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_edit_goto(
    edit: *mut mbs_editor,
    where_0: libc::c_int,
) -> libc::c_int {
    match where_0 {
        0 => {
            if (*edit).cursor == 0 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int
            } else {
                let mut n: size_t = 0;
                let mut cells: size_t = 0;
                n = mbs_prev(
                    (*edit).buf,
                    ((*edit).buf).offset((*edit).cursor as isize),
                    &mut cells,
                );
                if n != 0 {
                    (*edit)
                        .cursor = ((*edit).cursor as libc::c_ulong).wrapping_sub(n)
                        as size_t as size_t;
                    (*edit)
                        .cursor_cells = ((*edit).cursor_cells as libc::c_ulong)
                        .wrapping_sub(cells) as size_t as size_t;
                }
            }
        }
        1 => {
            if (*edit).cursor_cells >= (*edit).cur_cells {
                return 1 as libc::c_int
            } else {
                let mut n_0: size_t = 0;
                let mut cells_0: size_t = 0;
                n_0 = mbs_next(
                    ((*edit).buf).offset((*edit).cursor as isize),
                    &mut cells_0,
                );
                if n_0 != 0 {
                    (*edit)
                        .cursor = ((*edit).cursor as libc::c_ulong).wrapping_add(n_0)
                        as size_t as size_t;
                    (*edit)
                        .cursor_cells = ((*edit).cursor_cells as libc::c_ulong)
                        .wrapping_add(cells_0) as size_t as size_t;
                }
            }
        }
        3 => {
            (*edit).cursor = 0 as libc::c_int as size_t;
            (*edit).cursor_cells = 0 as libc::c_int as size_t;
        }
        2 => {
            (*edit).cursor = (*edit).cur_bytes;
            (*edit).cursor_cells = (*edit).cur_cells;
        }
        _ => return -(22 as libc::c_int),
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn remove_next(
    str: *mut libc::c_char,
    ncells: *mut size_t,
) -> size_t {
    let mut bytes: size_t = 0;
    let mut move_bytes: size_t = 0;
    let mut n: size_t = 0;
    n = mbs_next(str, ncells);
    bytes = strlen(str);
    move_bytes = bytes.wrapping_sub(n);
    memmove(
        str as *mut libc::c_void,
        str.offset(n as isize) as *const libc::c_void,
        move_bytes,
    );
    *str.offset(bytes.wrapping_sub(n) as isize) = '\0' as i32 as libc::c_char;
    return n;
}
unsafe extern "C" fn mbs_insert(
    str: *mut libc::c_char,
    c: wint_t,
    ncells: *mut size_t,
) -> size_t {
    let mut n: size_t = 1 as libc::c_int as size_t;
    let mut bytes: size_t = 0;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let wc: wchar_t = c as wchar_t;
    let vla = __ctype_get_mb_cur_max() as usize;
    let mut in_buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    n = wctomb(in_buf.as_mut_ptr(), wc) as size_t;
    if n == -(1 as libc::c_int) as size_t {
        return n;
    }
    *ncells = wcwidth(wc) as size_t;
    in_0 = in_buf.as_mut_ptr();
    bytes = strlen(str);
    memmove(
        str.offset(n as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        bytes,
    );
    memcpy(str as *mut libc::c_void, in_0 as *const libc::c_void, n);
    *str.offset(bytes.wrapping_add(n) as isize) = '\0' as i32 as libc::c_char;
    return n;
}
unsafe extern "C" fn mbs_edit_remove(edit: *mut mbs_editor) -> libc::c_int {
    let mut n: size_t = 0;
    let mut ncells: size_t = 0;
    if (*edit).cur_cells == 0 as libc::c_int as libc::c_ulong
        || (*edit).cursor >= (*edit).cur_bytes
    {
        return 1 as libc::c_int;
    }
    n = remove_next(((*edit).buf).offset((*edit).cursor as isize), &mut ncells);
    if n == -(1 as libc::c_int) as size_t {
        return 1 as libc::c_int;
    }
    (*edit)
        .cur_bytes = ((*edit).cur_bytes as libc::c_ulong).wrapping_sub(n) as size_t
        as size_t;
    (*edit).cur_cells = mbs_safe_width((*edit).buf);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_edit_delete(edit: *mut mbs_editor) -> libc::c_int {
    if (*edit).cursor >= (*edit).cur_bytes
        && mbs_edit_goto(edit, MBS_EDIT_LEFT as libc::c_int) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return mbs_edit_remove(edit);
}
#[no_mangle]
pub unsafe extern "C" fn mbs_edit_backspace(edit: *mut mbs_editor) -> libc::c_int {
    if mbs_edit_goto(edit, MBS_EDIT_LEFT as libc::c_int) == 0 as libc::c_int {
        return mbs_edit_remove(edit);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_edit_insert(
    edit: *mut mbs_editor,
    c: wint_t,
) -> libc::c_int {
    let mut n: size_t = 0;
    let mut ncells: size_t = 0;
    if ((*edit).cur_bytes).wrapping_add(__ctype_get_mb_cur_max()) > (*edit).max_bytes {
        return 1 as libc::c_int;
    }
    n = mbs_insert(((*edit).buf).offset((*edit).cursor as isize), c, &mut ncells);
    if n == -(1 as libc::c_int) as size_t {
        return 1 as libc::c_int;
    }
    (*edit)
        .cursor = ((*edit).cursor as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    (*edit)
        .cursor_cells = ((*edit).cursor_cells as libc::c_ulong).wrapping_add(ncells)
        as size_t as size_t;
    (*edit)
        .cur_bytes = ((*edit).cur_bytes as libc::c_ulong).wrapping_add(n) as size_t
        as size_t;
    (*edit).cur_cells = mbs_safe_width((*edit).buf);
    return 0 as libc::c_int;
}
