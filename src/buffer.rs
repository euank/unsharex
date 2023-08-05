use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn mbs_safe_nwidth(
        buf: *const libc::c_char,
        bufsz: size_t,
        sz: *mut size_t,
    ) -> size_t;
    fn mbs_width(s: *const libc::c_char) -> size_t;
    fn mbs_safe_encode_to_buffer(
        s: *const libc::c_char,
        width: *mut size_t,
        buf: *mut libc::c_char,
        safechars: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbs_safe_encode_size(bytes: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_buffer {
    pub begin: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub sz: size_t,
    pub chunksize: size_t,
    pub encoded: *mut libc::c_char,
    pub encoded_sz: size_t,
    pub ptrs: *mut *mut libc::c_char,
    pub nptrs: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_reset_data(mut buf: *mut ul_buffer) {
    if !((*buf).begin).is_null() {
        *((*buf).begin).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    (*buf).end = (*buf).begin;
    if !((*buf).ptrs).is_null() && (*buf).nptrs != 0 {
        memset(
            (*buf).ptrs as *mut libc::c_void,
            0 as libc::c_int,
            ((*buf).nptrs)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_free_data(mut buf: *mut ul_buffer) {
    if !buf.is_null() {} else {
        __assert_fail(
            b"buf\0" as *const u8 as *const libc::c_char,
            b"lib/buffer.c\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void ul_buffer_free_data(struct ul_buffer *)\0"))
                .as_ptr(),
        );
    }
    'c_4500: {
        if !buf.is_null() {} else {
            __assert_fail(
                b"buf\0" as *const u8 as *const libc::c_char,
                b"lib/buffer.c\0" as *const u8 as *const libc::c_char,
                23 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void ul_buffer_free_data(struct ul_buffer *)\0"))
                    .as_ptr(),
            );
        }
    };
    free((*buf).begin as *mut libc::c_void);
    (*buf).begin = 0 as *mut libc::c_char;
    (*buf).end = 0 as *mut libc::c_char;
    (*buf).sz = 0 as libc::c_int as size_t;
    free((*buf).ptrs as *mut libc::c_void);
    (*buf).ptrs = 0 as *mut *mut libc::c_char;
    (*buf).nptrs = 0 as libc::c_int as size_t;
    free((*buf).encoded as *mut libc::c_void);
    (*buf).encoded = 0 as *mut libc::c_char;
    (*buf).encoded_sz = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_set_chunksize(
    mut buf: *mut ul_buffer,
    mut sz: size_t,
) {
    (*buf).chunksize = sz;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_is_empty(mut buf: *mut ul_buffer) -> libc::c_int {
    return ((*buf).begin == (*buf).end) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_save_pointer(
    mut buf: *mut ul_buffer,
    mut ptr_idx: libc::c_ushort,
) -> libc::c_int {
    if ptr_idx as libc::c_ulong >= (*buf).nptrs {
        let mut tmp: *mut *mut libc::c_char = realloc(
            (*buf).ptrs as *mut libc::c_void,
            ((ptr_idx as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        if tmp.is_null() {
            return -(22 as libc::c_int);
        }
        (*buf).ptrs = tmp;
        (*buf).nptrs = (ptr_idx as libc::c_int + 1 as libc::c_int) as size_t;
    }
    let ref mut fresh0 = *((*buf).ptrs).offset(ptr_idx as isize);
    *fresh0 = (*buf).end;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_pointer(
    mut buf: *mut ul_buffer,
    mut ptr_idx: libc::c_ushort,
) -> *mut libc::c_char {
    if (ptr_idx as libc::c_ulong) < (*buf).nptrs {
        return *((*buf).ptrs).offset(ptr_idx as isize);
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_pointer_length(
    mut buf: *mut ul_buffer,
    mut ptr_idx: libc::c_ushort,
) -> size_t {
    let mut ptr: *mut libc::c_char = ul_buffer_get_pointer(buf, ptr_idx);
    if !ptr.is_null() && ptr > (*buf).begin {
        return ptr.offset_from((*buf).begin) as libc::c_long as size_t;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_safe_pointer_width(
    mut buf: *mut ul_buffer,
    mut ptr_idx: libc::c_ushort,
) -> size_t {
    let mut len: size_t = ul_buffer_get_pointer_length(buf, ptr_idx);
    if len == 0 {
        return 0 as libc::c_int as size_t;
    }
    return mbs_safe_nwidth((*buf).begin, len, 0 as *mut size_t);
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_refer_string(
    mut buf: *mut ul_buffer,
    mut str: *mut libc::c_char,
) {
    if (*buf).sz != 0 {
        ul_buffer_free_data(buf);
    }
    (*buf).begin = str;
    (*buf)
        .sz = if !str.is_null() {
        strlen(str)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    (*buf)
        .end = if !((*buf).begin).is_null() {
        ((*buf).begin).offset((*buf).sz as isize)
    } else {
        (*buf).begin
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_alloc_data(
    mut buf: *mut ul_buffer,
    mut sz: size_t,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !buf.is_null() {} else {
        __assert_fail(
            b"buf\0" as *const u8 as *const libc::c_char,
            b"lib/buffer.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int ul_buffer_alloc_data(struct ul_buffer *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_4767: {
        if !buf.is_null() {} else {
            __assert_fail(
                b"buf\0" as *const u8 as *const libc::c_char,
                b"lib/buffer.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"int ul_buffer_alloc_data(struct ul_buffer *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if sz <= (*buf).sz {
        return 0 as libc::c_int;
    }
    if !((*buf).end).is_null() && !((*buf).begin).is_null() {
        len = ((*buf).end).offset_from((*buf).begin) as libc::c_long as size_t;
    }
    if (*buf).chunksize != 0 {
        sz = sz
            .wrapping_add((*buf).chunksize)
            .wrapping_div((*buf).chunksize)
            .wrapping_mul((*buf).chunksize)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    tmp = realloc((*buf).begin as *mut libc::c_void, sz) as *mut libc::c_char;
    if tmp.is_null() {
        return -(12 as libc::c_int);
    }
    (*buf).begin = tmp;
    (*buf).end = ((*buf).begin).offset(len as isize);
    (*buf).sz = sz;
    memset((*buf).end as *mut libc::c_void, '\0' as i32, sz.wrapping_sub(len));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_append_data(
    mut buf: *mut ul_buffer,
    mut data: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut maxsz: size_t = 0 as libc::c_int as size_t;
    if buf.is_null() {
        return -(22 as libc::c_int);
    }
    if data.is_null() || *data == 0 {
        return 0 as libc::c_int;
    }
    if !((*buf).begin).is_null() && !((*buf).end).is_null() {
        maxsz = ((*buf).sz)
            .wrapping_sub(
                ((*buf).end).offset_from((*buf).begin) as libc::c_long as libc::c_ulong,
            );
    }
    if maxsz <= sz.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut rc: libc::c_int = ul_buffer_alloc_data(
            buf,
            ((*buf).sz).wrapping_add(sz).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if rc != 0 {
            return rc;
        }
    }
    if ((*buf).end).is_null() {
        return -(22 as libc::c_int);
    }
    (*buf)
        .end = mempcpy((*buf).end as *mut libc::c_void, data as *const libc::c_void, sz)
        as *mut libc::c_char;
    *(*buf).end = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_append_string(
    mut buf: *mut ul_buffer,
    mut str: *const libc::c_char,
) -> libc::c_int {
    if str.is_null() {
        return 0 as libc::c_int;
    }
    return ul_buffer_append_data(buf, str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_append_ntimes(
    mut buf: *mut ul_buffer,
    mut n: size_t,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut len: size_t = strlen(str);
    i = 0 as libc::c_int as size_t;
    while len != 0 && i < n {
        let mut rc: libc::c_int = ul_buffer_append_data(buf, str, len);
        if rc != 0 {
            return rc;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_set_data(
    mut buf: *mut ul_buffer,
    mut data: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    ul_buffer_reset_data(buf);
    return ul_buffer_append_data(buf, data, sz);
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_data(
    mut buf: *mut ul_buffer,
    mut sz: *mut size_t,
    mut width: *mut size_t,
) -> *mut libc::c_char {
    if !sz.is_null() {
        *sz = ((*buf).end).offset_from((*buf).begin) as libc::c_long as size_t;
    }
    if !width.is_null() {
        *width = if !((*buf).begin).is_null() && *(*buf).begin as libc::c_int != 0 {
            mbs_width((*buf).begin)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    }
    return (*buf).begin;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_bufsiz(mut buf: *mut ul_buffer) -> size_t {
    return (*buf).sz;
}
#[no_mangle]
pub unsafe extern "C" fn ul_buffer_get_safe_data(
    mut buf: *mut ul_buffer,
    mut sz: *mut size_t,
    mut width: *mut size_t,
    mut safechars: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut data: *mut libc::c_char = ul_buffer_get_data(
        buf,
        0 as *mut size_t,
        0 as *mut size_t,
    );
    let mut encsz: size_t = 0;
    let mut wsz: size_t = 0 as libc::c_int as size_t;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if !data.is_null() {
        encsz = (mbs_safe_encode_size((*buf).sz))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if encsz > (*buf).encoded_sz {
            let mut tmp: *mut libc::c_char = realloc(
                (*buf).encoded as *mut libc::c_void,
                encsz,
            ) as *mut libc::c_char;
            if tmp.is_null() {
                current_block = 7874665414274482590;
            } else {
                (*buf).encoded = tmp;
                (*buf).encoded_sz = encsz;
                current_block = 2473556513754201174;
            }
        } else {
            current_block = 2473556513754201174;
        }
        match current_block {
            7874665414274482590 => {}
            _ => {
                res = mbs_safe_encode_to_buffer(
                    data,
                    &mut wsz,
                    (*buf).encoded,
                    safechars,
                );
                if !(res.is_null() || wsz == 0 || wsz == -(1 as libc::c_int) as size_t) {
                    if !width.is_null() {
                        *width = wsz;
                    }
                    if !sz.is_null() {
                        *sz = strlen(res);
                    }
                    return res;
                }
            }
        }
    }
    if !width.is_null() {
        *width = 0 as libc::c_int as size_t;
    }
    if !sz.is_null() {
        *sz = 0 as libc::c_int as size_t;
    }
    return 0 as *mut libc::c_char;
}
