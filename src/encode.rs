use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub const UL_ENCODE_UTF16LE: C2RustUnnamed = 1;
pub const UL_ENCODE_UTF16BE: C2RustUnnamed = 0;
pub const UL_ENCODE_LATIN1: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn ul_encode_to_utf8(
    mut enc: libc::c_int,
    mut dest: *mut libc::c_uchar,
    mut len: size_t,
    mut src: *const libc::c_uchar,
    mut count: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut c: uint32_t = 0;
    let mut c2: uint16_t = 0;
    i = 0 as libc::c_int as size_t;
    j = i;
    while i < count {
        if enc == UL_ENCODE_UTF16LE as libc::c_int {
            if i.wrapping_add(2 as libc::c_int as libc::c_ulong) > count {
                break;
            }
            c = ((*src.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int) << 8 as libc::c_int
                | *src.offset(i as isize) as libc::c_int) as uint32_t;
            i = i.wrapping_add(1);
            i;
        } else if enc == UL_ENCODE_UTF16BE as libc::c_int {
            if i.wrapping_add(2 as libc::c_int as libc::c_ulong) > count {
                break;
            }
            c = ((*src.offset(i as isize) as libc::c_int) << 8 as libc::c_int
                | *src.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int) as uint32_t;
            i = i.wrapping_add(1);
            i;
        } else if enc == UL_ENCODE_LATIN1 as libc::c_int {
            c = *src.offset(i as isize) as uint32_t;
        } else {
            return 0 as libc::c_int as size_t
        }
        if (enc == UL_ENCODE_UTF16LE as libc::c_int
            || enc == UL_ENCODE_UTF16BE as libc::c_int)
            && c >= 0xd800 as libc::c_int as libc::c_uint
            && c <= 0xdbff as libc::c_int as libc::c_uint
            && i.wrapping_add(2 as libc::c_int as libc::c_ulong) < count
        {
            if enc == UL_ENCODE_UTF16LE as libc::c_int {
                c2 = ((*src
                    .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *src
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int) as uint16_t;
            } else {
                c2 = ((*src
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *src
                        .offset(
                            i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int) as uint16_t;
            }
            if c2 as libc::c_int >= 0xdc00 as libc::c_int
                && c2 as libc::c_int <= 0xdfff as libc::c_int
            {
                c = (0x10000 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        c.wrapping_sub(0xd800 as libc::c_int as libc::c_uint)
                            << 10 as libc::c_int,
                    )
                    .wrapping_add(
                        (c2 as libc::c_int - 0xdc00 as libc::c_int) as libc::c_uint,
                    );
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        if c == 0 as libc::c_int as libc::c_uint {
            *dest.offset(j as isize) = '\0' as i32 as libc::c_uchar;
            break;
        } else {
            if c < 0x80 as libc::c_int as libc::c_uint {
                if j.wrapping_add(1 as libc::c_int as libc::c_ulong) >= len {
                    break;
                }
                let fresh0 = j;
                j = j.wrapping_add(1);
                *dest.offset(fresh0 as isize) = c as uint8_t;
            } else if c < 0x800 as libc::c_int as libc::c_uint {
                if j.wrapping_add(2 as libc::c_int as libc::c_ulong) >= len {
                    break;
                }
                let fresh1 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh1 as isize,
                    ) = (0xc0 as libc::c_int as libc::c_uint | c >> 6 as libc::c_int)
                    as uint8_t;
                let fresh2 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh2 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            } else if c < 0x10000 as libc::c_int as libc::c_uint {
                if j.wrapping_add(3 as libc::c_int as libc::c_ulong) >= len {
                    break;
                }
                let fresh3 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh3 as isize,
                    ) = (0xe0 as libc::c_int as libc::c_uint | c >> 12 as libc::c_int)
                    as uint8_t;
                let fresh4 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh4 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as uint8_t;
                let fresh5 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh5 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            } else {
                if j.wrapping_add(4 as libc::c_int as libc::c_ulong) >= len {
                    break;
                }
                let fresh6 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh6 as isize,
                    ) = (0xf0 as libc::c_int as libc::c_uint | c >> 18 as libc::c_int)
                    as uint8_t;
                let fresh7 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh7 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as uint8_t;
                let fresh8 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh8 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    as uint8_t;
                let fresh9 = j;
                j = j.wrapping_add(1);
                *dest
                    .offset(
                        fresh9 as isize,
                    ) = (0x80 as libc::c_int as libc::c_uint
                    | c & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    *dest.offset(j as isize) = '\0' as i32 as libc::c_uchar;
    return j;
}
