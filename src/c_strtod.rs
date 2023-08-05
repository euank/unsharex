use ::libc;
extern "C" {
    pub type __locale_data;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtod_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: locale_t,
    ) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: locale_t,
    ) -> locale_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
static mut c_locale: locale_t = 0 as *const __locale_struct as *mut __locale_struct;
unsafe extern "C" fn get_c_locale() -> locale_t {
    if c_locale.is_null() {
        ::core::ptr::write_volatile(
            &mut c_locale as *mut locale_t,
            newlocale(
                (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int
                    | (1 as libc::c_int) << 11 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int,
                b"C\0" as *const u8 as *const libc::c_char,
                0 as locale_t,
            ),
        );
    }
    return c_locale;
}
#[no_mangle]
pub unsafe extern "C" fn c_strtod(
    mut str: *const libc::c_char,
    mut end: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut res: libc::c_double = 0.;
    let mut errsv: libc::c_int = 0;
    let mut cl: locale_t = get_c_locale();
    if !cl.is_null() {
        return strtod_l(str, end, cl);
    }
    let mut org_locale: *mut libc::c_char = setlocale(
        1 as libc::c_int,
        0 as *const libc::c_char,
    );
    if !org_locale.is_null() {
        org_locale = strdup(org_locale);
        if org_locale.is_null() {
            return 0 as libc::c_int as libc::c_double;
        }
        setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    }
    res = strtod(str, end);
    errsv = *__errno_location();
    if !org_locale.is_null() {
        setlocale(1 as libc::c_int, org_locale);
        free(org_locale as *mut libc::c_void);
    }
    *__errno_location() = errsv;
    return res;
}
