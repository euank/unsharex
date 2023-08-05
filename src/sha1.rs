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
pub struct UL_SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CHAR64LONG16 {
    pub c: [libc::c_uchar; 64],
    pub l: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA1Transform(
    mut state: *mut uint32_t,
    mut buffer: *const libc::c_uchar,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut block: [CHAR64LONG16; 1] = [CHAR64LONG16 { c: [0; 64] }; 1];
    memcpy(
        block.as_mut_ptr() as *mut libc::c_void,
        buffer as *const libc::c_void,
        64 as libc::c_int as libc::c_ulong,
    );
    a = *state.offset(0 as libc::c_int as isize);
    b = *state.offset(1 as libc::c_int as isize);
    c = *state.offset(2 as libc::c_int as isize);
    d = *state.offset(3 as libc::c_int as isize);
    e = *state.offset(4 as libc::c_int as isize);
    let ref mut fresh0 = (*block.as_mut_ptr()).l[0 as libc::c_int as usize];
    *fresh0 = ((*block.as_mut_ptr()).l[0 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[0 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[0 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[0 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(*fresh0)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh1 = (*block.as_mut_ptr()).l[1 as libc::c_int as usize];
    *fresh1 = ((*block.as_mut_ptr()).l[1 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[1 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[1 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[1 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(*fresh1)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh2 = (*block.as_mut_ptr()).l[2 as libc::c_int as usize];
    *fresh2 = ((*block.as_mut_ptr()).l[2 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[2 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[2 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[2 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(*fresh2)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh3 = (*block.as_mut_ptr()).l[3 as libc::c_int as usize];
    *fresh3 = ((*block.as_mut_ptr()).l[3 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[3 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[3 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[3 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(*fresh3)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh4 = (*block.as_mut_ptr()).l[4 as libc::c_int as usize];
    *fresh4 = ((*block.as_mut_ptr()).l[4 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[4 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[4 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[4 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(*fresh4)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh5 = (*block.as_mut_ptr()).l[5 as libc::c_int as usize];
    *fresh5 = ((*block.as_mut_ptr()).l[5 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[5 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[5 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[5 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(*fresh5)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh6 = (*block.as_mut_ptr()).l[6 as libc::c_int as usize];
    *fresh6 = ((*block.as_mut_ptr()).l[6 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[6 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[6 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[6 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(*fresh6)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh7 = (*block.as_mut_ptr()).l[7 as libc::c_int as usize];
    *fresh7 = ((*block.as_mut_ptr()).l[7 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[7 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[7 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[7 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(*fresh7)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh8 = (*block.as_mut_ptr()).l[8 as libc::c_int as usize];
    *fresh8 = ((*block.as_mut_ptr()).l[8 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[8 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[8 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[8 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(*fresh8)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh9 = (*block.as_mut_ptr()).l[9 as libc::c_int as usize];
    *fresh9 = ((*block.as_mut_ptr()).l[9 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[9 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[9 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[9 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(*fresh9)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh10 = (*block.as_mut_ptr()).l[10 as libc::c_int as usize];
    *fresh10 = ((*block.as_mut_ptr()).l[10 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[10 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[10 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[10 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(*fresh10)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh11 = (*block.as_mut_ptr()).l[11 as libc::c_int as usize];
    *fresh11 = ((*block.as_mut_ptr()).l[11 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[11 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[11 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[11 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(*fresh11)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh12 = (*block.as_mut_ptr()).l[12 as libc::c_int as usize];
    *fresh12 = ((*block.as_mut_ptr()).l[12 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[12 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[12 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[12 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(*fresh12)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh13 = (*block.as_mut_ptr()).l[13 as libc::c_int as usize];
    *fresh13 = ((*block.as_mut_ptr()).l[13 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[13 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[13 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[13 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(*fresh13)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh14 = (*block.as_mut_ptr()).l[14 as libc::c_int as usize];
    *fresh14 = ((*block.as_mut_ptr()).l[14 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[14 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[14 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[14 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(*fresh14)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh15 = (*block.as_mut_ptr()).l[15 as libc::c_int as usize];
    *fresh15 = ((*block.as_mut_ptr()).l[15 as libc::c_int as usize] << 24 as libc::c_int
        | (*block.as_mut_ptr()).l[15 as libc::c_int as usize]
            >> 32 as libc::c_int - 24 as libc::c_int) & 0xff00ff00 as libc::c_uint
        | ((*block.as_mut_ptr()).l[15 as libc::c_int as usize] << 8 as libc::c_int
            | (*block.as_mut_ptr()).l[15 as libc::c_int as usize]
                >> 32 as libc::c_int - 8 as libc::c_int)
            & 0xff00ff as libc::c_int as libc::c_uint;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(*fresh15)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh16 = (*block.as_mut_ptr())
        .l[(16 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh16 = ((*block.as_mut_ptr())
        .l[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(16 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(16 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(*fresh16)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh17 = (*block.as_mut_ptr())
        .l[(17 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh17 = ((*block.as_mut_ptr())
        .l[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(17 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(17 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(*fresh17)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh18 = (*block.as_mut_ptr())
        .l[(18 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh18 = ((*block.as_mut_ptr())
        .l[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(18 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(18 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(*fresh18)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh19 = (*block.as_mut_ptr())
        .l[(19 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh19 = ((*block.as_mut_ptr())
        .l[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(19 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(19 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(*fresh19)
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh20 = (*block.as_mut_ptr())
        .l[(20 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh20 = ((*block.as_mut_ptr())
        .l[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(20 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(20 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh20)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh21 = (*block.as_mut_ptr())
        .l[(21 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh21 = ((*block.as_mut_ptr())
        .l[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(21 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(21 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh21)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh22 = (*block.as_mut_ptr())
        .l[(22 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh22 = ((*block.as_mut_ptr())
        .l[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(22 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(22 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh22)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh23 = (*block.as_mut_ptr())
        .l[(23 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh23 = ((*block.as_mut_ptr())
        .l[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(23 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(23 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh23)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh24 = (*block.as_mut_ptr())
        .l[(24 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh24 = ((*block.as_mut_ptr())
        .l[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(24 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(24 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh24)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh25 = (*block.as_mut_ptr())
        .l[(25 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh25 = ((*block.as_mut_ptr())
        .l[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(25 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(25 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh25)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh26 = (*block.as_mut_ptr())
        .l[(26 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh26 = ((*block.as_mut_ptr())
        .l[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(26 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(26 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh26)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh27 = (*block.as_mut_ptr())
        .l[(27 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh27 = ((*block.as_mut_ptr())
        .l[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(27 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(27 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh27)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh28 = (*block.as_mut_ptr())
        .l[(28 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh28 = ((*block.as_mut_ptr())
        .l[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(28 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(28 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh28)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh29 = (*block.as_mut_ptr())
        .l[(29 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh29 = ((*block.as_mut_ptr())
        .l[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(29 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(29 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh29)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh30 = (*block.as_mut_ptr())
        .l[(30 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh30 = ((*block.as_mut_ptr())
        .l[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(30 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(30 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh30)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh31 = (*block.as_mut_ptr())
        .l[(31 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh31 = ((*block.as_mut_ptr())
        .l[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(31 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(31 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh31)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh32 = (*block.as_mut_ptr())
        .l[(32 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh32 = ((*block.as_mut_ptr())
        .l[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(32 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(32 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh32)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh33 = (*block.as_mut_ptr())
        .l[(33 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh33 = ((*block.as_mut_ptr())
        .l[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(33 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(33 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh33)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh34 = (*block.as_mut_ptr())
        .l[(34 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh34 = ((*block.as_mut_ptr())
        .l[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(34 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(34 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh34)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh35 = (*block.as_mut_ptr())
        .l[(35 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh35 = ((*block.as_mut_ptr())
        .l[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(35 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(35 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh35)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh36 = (*block.as_mut_ptr())
        .l[(36 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh36 = ((*block.as_mut_ptr())
        .l[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(36 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(36 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh36)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh37 = (*block.as_mut_ptr())
        .l[(37 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh37 = ((*block.as_mut_ptr())
        .l[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(37 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(37 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh37)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh38 = (*block.as_mut_ptr())
        .l[(38 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh38 = ((*block.as_mut_ptr())
        .l[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(38 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(38 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh38)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh39 = (*block.as_mut_ptr())
        .l[(39 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh39 = ((*block.as_mut_ptr())
        .l[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(39 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(39 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh39)
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh40 = (*block.as_mut_ptr())
        .l[(40 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh40 = ((*block.as_mut_ptr())
        .l[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(40 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(40 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(*fresh40)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh41 = (*block.as_mut_ptr())
        .l[(41 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh41 = ((*block.as_mut_ptr())
        .l[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(41 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(41 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(*fresh41)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh42 = (*block.as_mut_ptr())
        .l[(42 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh42 = ((*block.as_mut_ptr())
        .l[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(42 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(42 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(*fresh42)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh43 = (*block.as_mut_ptr())
        .l[(43 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh43 = ((*block.as_mut_ptr())
        .l[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(43 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(43 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(*fresh43)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh44 = (*block.as_mut_ptr())
        .l[(44 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh44 = ((*block.as_mut_ptr())
        .l[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(44 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(44 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(*fresh44)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh45 = (*block.as_mut_ptr())
        .l[(45 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh45 = ((*block.as_mut_ptr())
        .l[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(45 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(45 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(*fresh45)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh46 = (*block.as_mut_ptr())
        .l[(46 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh46 = ((*block.as_mut_ptr())
        .l[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(46 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(46 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(*fresh46)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh47 = (*block.as_mut_ptr())
        .l[(47 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh47 = ((*block.as_mut_ptr())
        .l[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(47 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(47 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(*fresh47)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh48 = (*block.as_mut_ptr())
        .l[(48 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh48 = ((*block.as_mut_ptr())
        .l[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(48 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(48 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(*fresh48)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh49 = (*block.as_mut_ptr())
        .l[(49 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh49 = ((*block.as_mut_ptr())
        .l[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(49 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(49 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(*fresh49)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh50 = (*block.as_mut_ptr())
        .l[(50 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh50 = ((*block.as_mut_ptr())
        .l[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(50 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(50 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(*fresh50)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh51 = (*block.as_mut_ptr())
        .l[(51 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh51 = ((*block.as_mut_ptr())
        .l[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(51 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(51 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(*fresh51)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh52 = (*block.as_mut_ptr())
        .l[(52 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh52 = ((*block.as_mut_ptr())
        .l[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(52 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(52 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(*fresh52)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh53 = (*block.as_mut_ptr())
        .l[(53 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh53 = ((*block.as_mut_ptr())
        .l[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(53 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(53 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(*fresh53)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh54 = (*block.as_mut_ptr())
        .l[(54 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh54 = ((*block.as_mut_ptr())
        .l[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(54 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(54 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(*fresh54)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh55 = (*block.as_mut_ptr())
        .l[(55 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh55 = ((*block.as_mut_ptr())
        .l[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(55 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(55 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(*fresh55)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh56 = (*block.as_mut_ptr())
        .l[(56 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh56 = ((*block.as_mut_ptr())
        .l[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(56 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(56 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(*fresh56)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh57 = (*block.as_mut_ptr())
        .l[(57 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh57 = ((*block.as_mut_ptr())
        .l[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(57 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(57 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(*fresh57)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh58 = (*block.as_mut_ptr())
        .l[(58 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh58 = ((*block.as_mut_ptr())
        .l[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(58 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(58 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(*fresh58)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh59 = (*block.as_mut_ptr())
        .l[(59 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh59 = ((*block.as_mut_ptr())
        .l[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(59 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(59 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(*fresh59)
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh60 = (*block.as_mut_ptr())
        .l[(60 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh60 = ((*block.as_mut_ptr())
        .l[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(60 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(60 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh60)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh61 = (*block.as_mut_ptr())
        .l[(61 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh61 = ((*block.as_mut_ptr())
        .l[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(61 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(61 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh61)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh62 = (*block.as_mut_ptr())
        .l[(62 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh62 = ((*block.as_mut_ptr())
        .l[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(62 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(62 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh62)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh63 = (*block.as_mut_ptr())
        .l[(63 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh63 = ((*block.as_mut_ptr())
        .l[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(63 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(63 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh63)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh64 = (*block.as_mut_ptr())
        .l[(64 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh64 = ((*block.as_mut_ptr())
        .l[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(64 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(64 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh64)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh65 = (*block.as_mut_ptr())
        .l[(65 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh65 = ((*block.as_mut_ptr())
        .l[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(65 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(65 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh65)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh66 = (*block.as_mut_ptr())
        .l[(66 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh66 = ((*block.as_mut_ptr())
        .l[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(66 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(66 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh66)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh67 = (*block.as_mut_ptr())
        .l[(67 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh67 = ((*block.as_mut_ptr())
        .l[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(67 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(67 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh67)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh68 = (*block.as_mut_ptr())
        .l[(68 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh68 = ((*block.as_mut_ptr())
        .l[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(68 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(68 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh68)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh69 = (*block.as_mut_ptr())
        .l[(69 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh69 = ((*block.as_mut_ptr())
        .l[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(69 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(69 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh69)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh70 = (*block.as_mut_ptr())
        .l[(70 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh70 = ((*block.as_mut_ptr())
        .l[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(70 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(70 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh70)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh71 = (*block.as_mut_ptr())
        .l[(71 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh71 = ((*block.as_mut_ptr())
        .l[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(71 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(71 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh71)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh72 = (*block.as_mut_ptr())
        .l[(72 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh72 = ((*block.as_mut_ptr())
        .l[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(72 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(72 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh72)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh73 = (*block.as_mut_ptr())
        .l[(73 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh73 = ((*block.as_mut_ptr())
        .l[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(73 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(73 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh73)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh74 = (*block.as_mut_ptr())
        .l[(74 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh74 = ((*block.as_mut_ptr())
        .l[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(74 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(74 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh74)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh75 = (*block.as_mut_ptr())
        .l[(75 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh75 = ((*block.as_mut_ptr())
        .l[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(75 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(75 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*fresh75)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    b = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh76 = (*block.as_mut_ptr())
        .l[(76 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh76 = ((*block.as_mut_ptr())
        .l[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(76 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(76 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*fresh76)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    e << 5 as libc::c_int | e >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    a = a << 30 as libc::c_int | a >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh77 = (*block.as_mut_ptr())
        .l[(77 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh77 = ((*block.as_mut_ptr())
        .l[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(77 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(77 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(*fresh77)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    e = e << 30 as libc::c_int | e >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh78 = (*block.as_mut_ptr())
        .l[(78 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh78 = ((*block.as_mut_ptr())
        .l[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(78 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(78 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(*fresh78)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    c << 5 as libc::c_int | c >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    d = d << 30 as libc::c_int | d >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh79 = (*block.as_mut_ptr())
        .l[(79 as libc::c_int & 15 as libc::c_int) as usize];
    *fresh79 = ((*block.as_mut_ptr())
        .l[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr())
            .l[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ (*block.as_mut_ptr()).l[(79 as libc::c_int & 15 as libc::c_int) as usize])
        << 1 as libc::c_int
        | ((*block.as_mut_ptr())
            .l[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr())
                .l[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ (*block.as_mut_ptr()).l[(79 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(*fresh79)
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    b << 5 as libc::c_int | b >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        ) as uint32_t as uint32_t;
    c = c << 30 as libc::c_int | c >> 32 as libc::c_int - 30 as libc::c_int;
    let ref mut fresh80 = *state.offset(0 as libc::c_int as isize);
    *fresh80 = (*fresh80 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh81 = *state.offset(1 as libc::c_int as isize);
    *fresh81 = (*fresh81 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh82 = *state.offset(2 as libc::c_int as isize);
    *fresh82 = (*fresh82 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh83 = *state.offset(3 as libc::c_int as isize);
    *fresh83 = (*fresh83 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    let ref mut fresh84 = *state.offset(4 as libc::c_int as isize);
    *fresh84 = (*fresh84 as libc::c_uint).wrapping_add(e) as uint32_t as uint32_t;
    e = 0 as libc::c_int as uint32_t;
    d = e;
    c = d;
    b = c;
    a = b;
    memset(
        block.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<[CHAR64LONG16; 1]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA1Init(mut context: *mut UL_SHA1_CTX) {
    (*context).state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as uint32_t;
    (*context).state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*context).state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*context).state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as uint32_t;
    (*context).state[4 as libc::c_int as usize] = 0xc3d2e1f0 as libc::c_uint;
    (*context).count[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*context)
        .count[0 as libc::c_int as usize] = (*context).count[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA1Update(
    mut context: *mut UL_SHA1_CTX,
    mut data: *const libc::c_uchar,
    mut len: uint32_t,
) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    j = (*context).count[0 as libc::c_int as usize];
    (*context)
        .count[0 as libc::c_int
        as usize] = ((*context).count[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len << 3 as libc::c_int) as uint32_t as uint32_t;
    if (*context).count[0 as libc::c_int as usize] < j {
        (*context)
            .count[1 as libc::c_int
            as usize] = ((*context).count[1 as libc::c_int as usize]).wrapping_add(1);
        (*context).count[1 as libc::c_int as usize];
    }
    (*context)
        .count[1 as libc::c_int
        as usize] = ((*context).count[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len >> 29 as libc::c_int) as uint32_t as uint32_t;
    j = j >> 3 as libc::c_int & 63 as libc::c_int as libc::c_uint;
    if j.wrapping_add(len) > 63 as libc::c_int as libc::c_uint {
        i = (64 as libc::c_int as libc::c_uint).wrapping_sub(j);
        memcpy(
            &mut *((*context).buffer).as_mut_ptr().offset(j as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            data as *const libc::c_void,
            i as libc::c_ulong,
        );
        ul_SHA1Transform(
            ((*context).state).as_mut_ptr(),
            ((*context).buffer).as_mut_ptr() as *const libc::c_uchar,
        );
        while i.wrapping_add(63 as libc::c_int as libc::c_uint) < len {
            ul_SHA1Transform(((*context).state).as_mut_ptr(), &*data.offset(i as isize));
            i = (i as libc::c_uint).wrapping_add(64 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        }
        j = 0 as libc::c_int as uint32_t;
    } else {
        i = 0 as libc::c_int as uint32_t;
    }
    memcpy(
        &mut *((*context).buffer).as_mut_ptr().offset(j as isize) as *mut libc::c_uchar
            as *mut libc::c_void,
        &*data.offset(i as isize) as *const libc::c_uchar as *const libc::c_void,
        len.wrapping_sub(i) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA1Final(
    mut digest: *mut libc::c_uchar,
    mut context: *mut UL_SHA1_CTX,
) {
    let mut i: libc::c_uint = 0;
    let mut finalcount: [libc::c_uchar; 8] = [0; 8];
    let mut c: libc::c_uchar = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        finalcount[i
            as usize] = ((*context)
            .count[(if i >= 4 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as usize]
            >> (3 as libc::c_int as libc::c_uint)
                .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    c = 0o200 as libc::c_int as libc::c_uchar;
    ul_SHA1Update(context, &mut c, 1 as libc::c_int as uint32_t);
    while (*context).count[0 as libc::c_int as usize]
        & 504 as libc::c_int as libc::c_uint != 448 as libc::c_int as libc::c_uint
    {
        c = 0 as libc::c_int as libc::c_uchar;
        ul_SHA1Update(context, &mut c, 1 as libc::c_int as uint32_t);
    }
    ul_SHA1Update(context, finalcount.as_mut_ptr(), 8 as libc::c_int as uint32_t);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 20 as libc::c_int as libc::c_uint {
        *digest
            .offset(
                i as isize,
            ) = ((*context).state[(i >> 2 as libc::c_int) as usize]
            >> (3 as libc::c_int as libc::c_uint)
                .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    memset(
        context as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<UL_SHA1_CTX>() as libc::c_ulong,
    );
    memset(
        &mut finalcount as *mut [libc::c_uchar; 8] as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA1(
    mut hash_out: *mut libc::c_char,
    mut str: *const libc::c_char,
    mut len: libc::c_uint,
) {
    let mut ctx: UL_SHA1_CTX = UL_SHA1_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut ii: libc::c_uint = 0;
    ul_SHA1Init(&mut ctx);
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < len {
        ul_SHA1Update(
            &mut ctx,
            (str as *const libc::c_uchar).offset(ii as isize),
            1 as libc::c_int as uint32_t,
        );
        ii = ii.wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    ul_SHA1Final(hash_out as *mut libc::c_uchar, &mut ctx);
    *hash_out.offset(20 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
