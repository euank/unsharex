use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn split(
        state: *mut *const libc::c_char,
        l: *mut size_t,
        separator: *const libc::c_char,
        quoted: libc::c_int,
    ) -> *const libc::c_char;
    fn strconcat(
        s: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn strv_clear(mut l: *mut *mut libc::c_char) {
    let mut k: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if l.is_null() {
        return;
    }
    k = l;
    while !(*k).is_null() {
        free(*k as *mut libc::c_void);
        k = k.offset(1);
        k;
    }
    *l = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn strv_free(
    mut l: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    strv_clear(l);
    free(l as *mut libc::c_void);
    return 0 as *mut *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn strv_copy(
    mut l: *const *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut r: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut k: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    r = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                (strv_length(l)).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    k = r;
    if r.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    if !l.is_null() {
        while !(*l).is_null() {
            *k = strdup(*l);
            if (*k).is_null() {
                strv_free(r);
                return 0 as *mut *mut libc::c_char;
            }
            k = k.offset(1);
            k;
            l = l.offset(1);
            l;
        }
    }
    *k = 0 as *mut libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_length(mut l: *const *mut libc::c_char) -> libc::c_uint {
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if l.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    while !(*l).is_null() {
        n = n.wrapping_add(1);
        n;
        l = l.offset(1);
        l;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn strv_new_ap(
    mut x: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut *mut libc::c_char {
    let mut current_block: u64;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut a: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut aq: ::core::ffi::VaListImpl;
    if !x.is_null() {
        n = (if x == -(1 as libc::c_int) as *const libc::c_char {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_uint;
        aq = ap.clone();
        loop {
            s = aq.arg::<*const libc::c_char>();
            if s.is_null() {
                break;
            }
            if s == -(1 as libc::c_int) as *const libc::c_char {
                continue;
            }
            n = n.wrapping_add(1);
            n;
        }
    }
    a = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                n.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    if a.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    if !x.is_null() {
        if x != -(1 as libc::c_int) as *const libc::c_char {
            let ref mut fresh0 = *a.offset(i as isize);
            *fresh0 = strdup(x);
            if (*a.offset(i as isize)).is_null() {
                current_block = 8463583645481103270;
            } else {
                i = i.wrapping_add(1);
                i;
                current_block = 8236137900636309791;
            }
        } else {
            current_block = 8236137900636309791;
        }
        loop {
            match current_block {
                8463583645481103270 => {
                    strv_free(a);
                    return 0 as *mut *mut libc::c_char;
                }
                _ => {
                    s = ap.arg::<*const libc::c_char>();
                    if s.is_null() {
                        break;
                    }
                    if s == -(1 as libc::c_int) as *const libc::c_char {
                        current_block = 8236137900636309791;
                        continue;
                    }
                    let ref mut fresh1 = *a.offset(i as isize);
                    *fresh1 = strdup(s);
                    if (*a.offset(i as isize)).is_null() {
                        current_block = 8463583645481103270;
                        continue;
                    }
                    i = i.wrapping_add(1);
                    i;
                    current_block = 8236137900636309791;
                }
            }
        }
    }
    let ref mut fresh2 = *a.offset(i as isize);
    *fresh2 = 0 as *mut libc::c_char;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn strv_new(
    mut x: *const libc::c_char,
    mut args: ...
) -> *mut *mut libc::c_char {
    let mut r: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    r = strv_new_ap(x, ap.as_va_list());
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_extend_strv(
    mut a: *mut *mut *mut libc::c_char,
    mut b: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    s = b;
    while !s.is_null() && !(*s).is_null() {
        r = strv_extend(a, *s);
        if r < 0 as libc::c_int {
            return r;
        }
        s = s.offset(1);
        s;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strv_extend_strv_concat(
    mut a: *mut *mut *mut libc::c_char,
    mut b: *mut *mut libc::c_char,
    mut suffix: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    s = b;
    while !s.is_null() && !(*s).is_null() {
        let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
        v = strconcat(*s, suffix);
        if v.is_null() {
            return -(12 as libc::c_int);
        }
        r = strv_push(a, v);
        if r < 0 as libc::c_int {
            free(v as *mut libc::c_void);
            return r;
        }
        s = s.offset(1);
        s;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strv_split(
    mut s: *const libc::c_char,
    mut separator: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut word: *const libc::c_char = 0 as *const libc::c_char;
    let mut state: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: size_t = 0;
    let mut n: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut r: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !s.is_null() {} else {
        __assert_fail(
            b"s\0" as *const u8 as *const libc::c_char,
            b"lib/strv.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"char **strv_split(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_7174: {
        if !s.is_null() {} else {
            __assert_fail(
                b"s\0" as *const u8 as *const libc::c_char,
                b"lib/strv.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"char **strv_split(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    n = 0 as libc::c_int as libc::c_uint;
    state = s;
    word = split(&mut state, &mut l, separator, 0 as libc::c_int);
    while !word.is_null() {
        n = n.wrapping_add(1);
        n;
        word = split(&mut state, &mut l, separator, 0 as libc::c_int);
    }
    r = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                n.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    if r.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    i = 0 as libc::c_int as libc::c_uint;
    state = s;
    word = split(&mut state, &mut l, separator, 0 as libc::c_int);
    while !word.is_null() {
        let ref mut fresh3 = *r.offset(i as isize);
        *fresh3 = strndup(word, l);
        if (*r.offset(i as isize)).is_null() {
            strv_free(r);
            return 0 as *mut *mut libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
        word = split(&mut state, &mut l, separator, 0 as libc::c_int);
    }
    let ref mut fresh4 = *r.offset(i as isize);
    *fresh4 = 0 as *mut libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_join(
    mut l: *mut *mut libc::c_char,
    mut separator: *const libc::c_char,
) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: size_t = 0;
    let mut k: size_t = 0;
    if separator.is_null() {
        separator = b" \0" as *const u8 as *const libc::c_char;
    }
    k = strlen(separator);
    n = 0 as libc::c_int as size_t;
    s = l;
    while !s.is_null() && !(*s).is_null() {
        if n != 0 as libc::c_int as libc::c_ulong {
            n = (n as libc::c_ulong).wrapping_add(k) as size_t as size_t;
        }
        n = (n as libc::c_ulong).wrapping_add(strlen(*s)) as size_t as size_t;
        s = s.offset(1);
        s;
    }
    r = malloc(n.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if r.is_null() {
        return 0 as *mut libc::c_char;
    }
    e = r;
    s = l;
    while !s.is_null() && !(*s).is_null() {
        if e != r {
            e = stpcpy(e, separator);
        }
        e = stpcpy(e, *s);
        s = s.offset(1);
        s;
    }
    *e = 0 as libc::c_int as libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_push(
    mut l: *mut *mut *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut c: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    if value.is_null() {
        return 0 as libc::c_int;
    }
    n = strv_length(*l);
    m = n.wrapping_add(2 as libc::c_int as libc::c_uint);
    if m < n {
        return -(12 as libc::c_int);
    }
    c = realloc(
        *l as *mut libc::c_void,
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(m as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if c.is_null() {
        return -(12 as libc::c_int);
    }
    let ref mut fresh5 = *c.offset(n as isize);
    *fresh5 = value;
    let ref mut fresh6 = *c
        .offset(n.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
    *fresh6 = 0 as *mut libc::c_char;
    *l = c;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strv_push_prepend(
    mut l: *mut *mut *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut c: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if value.is_null() {
        return 0 as libc::c_int;
    }
    n = strv_length(*l);
    m = n.wrapping_add(2 as libc::c_int as libc::c_uint);
    if m < n {
        return -(12 as libc::c_int);
    }
    c = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(m as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if c.is_null() {
        return -(12 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        let ref mut fresh7 = *c
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        *fresh7 = *(*l).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let ref mut fresh8 = *c.offset(0 as libc::c_int as isize);
    *fresh8 = value;
    let ref mut fresh9 = *c
        .offset(n.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
    *fresh9 = 0 as *mut libc::c_char;
    free(*l as *mut libc::c_void);
    *l = c;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strv_consume(
    mut l: *mut *mut *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = strv_push(l, value);
    if r < 0 as libc::c_int {
        free(value as *mut libc::c_void);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_consume_prepend(
    mut l: *mut *mut *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = strv_push_prepend(l, value);
    if r < 0 as libc::c_int {
        free(value as *mut libc::c_void);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strv_extend(
    mut l: *mut *mut *mut libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if value.is_null() {
        return 0 as libc::c_int;
    }
    v = strdup(value);
    if v.is_null() {
        return -(12 as libc::c_int);
    }
    return strv_consume(l, v);
}
#[no_mangle]
pub unsafe extern "C" fn strv_remove(
    mut l: *mut *mut libc::c_char,
    mut s: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut f: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut t: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if l.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    if !s.is_null() {} else {
        __assert_fail(
            b"s\0" as *const u8 as *const libc::c_char,
            b"lib/strv.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"char **strv_remove(char **, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6668: {
        if !s.is_null() {} else {
            __assert_fail(
                b"s\0" as *const u8 as *const libc::c_char,
                b"lib/strv.c\0" as *const u8 as *const libc::c_char,
                348 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"char **strv_remove(char **, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    t = l;
    f = t;
    while !(*f).is_null() {
        if strcmp(*f, s) == 0 as libc::c_int {
            free(*f as *mut libc::c_void);
        } else {
            let fresh10 = t;
            t = t.offset(1);
            *fresh10 = *f;
        }
        f = f.offset(1);
        f;
    }
    *t = 0 as *mut libc::c_char;
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn strv_extendf(
    mut l: *mut *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    ap = args.clone();
    r = vasprintf(&mut x, format, ap.as_va_list());
    if r < 0 as libc::c_int {
        return -(12 as libc::c_int);
    }
    return strv_consume(l, x);
}
#[no_mangle]
pub unsafe extern "C" fn strv_extendv(
    mut l: *mut *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    r = vasprintf(&mut x, format, ap.as_va_list());
    if r < 0 as libc::c_int {
        return -(12 as libc::c_int);
    }
    return strv_consume(l, x);
}
#[no_mangle]
pub unsafe extern "C" fn strv_reverse(
    mut l: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut n: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    n = strv_length(l);
    if n <= 1 as libc::c_int as libc::c_uint {
        return l;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < n.wrapping_div(2 as libc::c_int as libc::c_uint) {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        t = *l.offset(i as isize);
        let ref mut fresh11 = *l.offset(i as isize);
        *fresh11 = *l
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i) as isize,
            );
        let ref mut fresh12 = *l
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i) as isize,
            );
        *fresh12 = t;
        i = i.wrapping_add(1);
        i;
    }
    return l;
}
