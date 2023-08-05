use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UL_MD5Context {
    pub buf: [uint32_t; 4],
    pub bits: [uint32_t; 2],
    pub in_0: [libc::c_uchar; 64],
}
#[no_mangle]
pub unsafe extern "C" fn ul_MD5Init(mut ctx: *mut UL_MD5Context) {
    (*ctx).buf[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as uint32_t;
    (*ctx).buf[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*ctx).buf[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*ctx).buf[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as uint32_t;
    (*ctx).bits[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).bits[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ul_MD5Update(
    mut ctx: *mut UL_MD5Context,
    mut buf: *const libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut t: uint32_t = 0;
    t = (*ctx).bits[0 as libc::c_int as usize];
    (*ctx).bits[0 as libc::c_int as usize] = t.wrapping_add(len << 3 as libc::c_int);
    if (*ctx).bits[0 as libc::c_int as usize] < t {
        (*ctx)
            .bits[1 as libc::c_int
            as usize] = ((*ctx).bits[1 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).bits[1 as libc::c_int as usize];
    }
    (*ctx)
        .bits[1 as libc::c_int
        as usize] = ((*ctx).bits[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len >> 29 as libc::c_int) as uint32_t as uint32_t;
    t = t >> 3 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
    if t != 0 {
        let mut p: *mut libc::c_uchar = ((*ctx).in_0).as_mut_ptr().offset(t as isize);
        t = (64 as libc::c_int as libc::c_uint).wrapping_sub(t);
        if len < t {
            memcpy(
                p as *mut libc::c_void,
                buf as *const libc::c_void,
                len as libc::c_ulong,
            );
            return;
        }
        memcpy(p as *mut libc::c_void, buf as *const libc::c_void, t as libc::c_ulong);
        ul_MD5Transform(
            ((*ctx).buf).as_mut_ptr(),
            ((*ctx).in_0).as_mut_ptr() as *mut uint32_t as *const uint32_t,
        );
        buf = buf.offset(t as isize);
        len = len.wrapping_sub(t);
    }
    while len >= 64 as libc::c_int as libc::c_uint {
        memcpy(
            ((*ctx).in_0).as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        ul_MD5Transform(
            ((*ctx).buf).as_mut_ptr(),
            ((*ctx).in_0).as_mut_ptr() as *mut uint32_t as *const uint32_t,
        );
        buf = buf.offset(64 as libc::c_int as isize);
        len = len.wrapping_sub(64 as libc::c_int as libc::c_uint);
    }
    memcpy(
        ((*ctx).in_0).as_mut_ptr() as *mut libc::c_void,
        buf as *const libc::c_void,
        len as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_MD5Final(
    mut digest: *mut libc::c_uchar,
    mut ctx: *mut UL_MD5Context,
) {
    let mut count: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    count = (*ctx).bits[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint;
    p = ((*ctx).in_0).as_mut_ptr().offset(count as isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 0x80 as libc::c_int as libc::c_uchar;
    count = ((64 as libc::c_int - 1 as libc::c_int) as libc::c_uint).wrapping_sub(count);
    if count < 8 as libc::c_int as libc::c_uint {
        memset(p as *mut libc::c_void, 0 as libc::c_int, count as libc::c_ulong);
        ul_MD5Transform(
            ((*ctx).buf).as_mut_ptr(),
            ((*ctx).in_0).as_mut_ptr() as *mut uint32_t as *const uint32_t,
        );
        memset(
            ((*ctx).in_0).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            56 as libc::c_int as libc::c_ulong,
        );
    } else {
        memset(
            p as *mut libc::c_void,
            0 as libc::c_int,
            count.wrapping_sub(8 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
    }
    memcpy(
        &mut *((*ctx).in_0)
            .as_mut_ptr()
            .offset(
                (14 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_uchar as *mut libc::c_void,
        &mut *((*ctx).bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        &mut *((*ctx).in_0)
            .as_mut_ptr()
            .offset(
                (15 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_uchar as *mut libc::c_void,
        &mut *((*ctx).bits).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    ul_MD5Transform(
        ((*ctx).buf).as_mut_ptr(),
        ((*ctx).in_0).as_mut_ptr() as *mut uint32_t as *const uint32_t,
    );
    memcpy(
        digest as *mut libc::c_void,
        ((*ctx).buf).as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<UL_MD5Context>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_MD5Transform(
    mut buf: *mut uint32_t,
    mut in_0: *const uint32_t,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    a = *buf.offset(0 as libc::c_int as isize);
    b = *buf.offset(1 as libc::c_int as isize);
    c = *buf.offset(2 as libc::c_int as isize);
    d = *buf.offset(3 as libc::c_int as isize);
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
                .wrapping_add(0xd76aa478 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
                .wrapping_add(0xe8c7b756 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
                .wrapping_add(0x242070db as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
                .wrapping_add(0xc1bdceee as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
                .wrapping_add(0xf57c0faf as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
                .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
                .wrapping_add(0xa8304613 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
                .wrapping_add(0xfd469501 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
                .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
                .wrapping_add(0x8b44f7af as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
                .wrapping_add(0xffff5bb1 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
                .wrapping_add(0x895cd7be as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
                .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
                .wrapping_add(0xfd987193 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
                .wrapping_add(0xa679438e as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
                .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
                .wrapping_add(0xf61e2562 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
                .wrapping_add(0xc040b340 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
                .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
                .wrapping_add(0xe9b6c7aa as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
                .wrapping_add(0xd62f105d as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
                .wrapping_add(0x2441453 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
                .wrapping_add(0xd8a1e681 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
                .wrapping_add(0xe7d3fbc8 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
                .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
                .wrapping_add(0xc33707d6 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
                .wrapping_add(0xf4d50d87 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
                .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
                .wrapping_add(0xa9e3e905 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
                .wrapping_add(0xfcefa3f8 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
                .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
                .wrapping_add(0x8d2a4c8a as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
                .wrapping_add(0xfffa3942 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
                .wrapping_add(0x8771f681 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
                .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
                .wrapping_add(0xfde5380c as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
                .wrapping_add(0xa4beea44 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
                .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
                .wrapping_add(0xf6bb4b60 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
                .wrapping_add(0xbebfbc70 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
                .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
                .wrapping_add(0xeaa127fa as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
                .wrapping_add(0xd4ef3085 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
                .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
                .wrapping_add(0xd9d4d039 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
                .wrapping_add(0xe6db99e5 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
                .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
                .wrapping_add(0xc4ac5665 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
                .wrapping_add(0xf4292244 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
                .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
                .wrapping_add(0xab9423a7 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
                .wrapping_add(0xfc93a039 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
                .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
                .wrapping_add(0x8f0ccc92 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
                .wrapping_add(0xffeff47d as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
                .wrapping_add(0x85845dd1 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
                .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
                .wrapping_add(0xfe2ce6e0 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
                .wrapping_add(0xa3014314 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
                .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
                .wrapping_add(0xf7537e82 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
                .wrapping_add(0xbd3af235 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
                .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
                .wrapping_add(0xeb86d391 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh1 = *buf.offset(0 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *buf.offset(1 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *buf.offset(2 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *buf.offset(3 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
}
