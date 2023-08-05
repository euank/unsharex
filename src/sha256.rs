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
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256 {
    pub len: uint64_t,
    pub h: [uint32_t; 8],
    pub buf: [uint8_t; 64],
}
unsafe extern "C" fn ror(mut n: uint32_t, mut k: libc::c_int) -> uint32_t {
    return n >> k | n << 32 as libc::c_int - k;
}
static mut K: [uint32_t; 64] = [
    0x428a2f98 as libc::c_int as uint32_t,
    0x71374491 as libc::c_int as uint32_t,
    0xb5c0fbcf as libc::c_uint,
    0xe9b5dba5 as libc::c_uint,
    0x3956c25b as libc::c_int as uint32_t,
    0x59f111f1 as libc::c_int as uint32_t,
    0x923f82a4 as libc::c_uint,
    0xab1c5ed5 as libc::c_uint,
    0xd807aa98 as libc::c_uint,
    0x12835b01 as libc::c_int as uint32_t,
    0x243185be as libc::c_int as uint32_t,
    0x550c7dc3 as libc::c_int as uint32_t,
    0x72be5d74 as libc::c_int as uint32_t,
    0x80deb1fe as libc::c_uint,
    0x9bdc06a7 as libc::c_uint,
    0xc19bf174 as libc::c_uint,
    0xe49b69c1 as libc::c_uint,
    0xefbe4786 as libc::c_uint,
    0xfc19dc6 as libc::c_int as uint32_t,
    0x240ca1cc as libc::c_int as uint32_t,
    0x2de92c6f as libc::c_int as uint32_t,
    0x4a7484aa as libc::c_int as uint32_t,
    0x5cb0a9dc as libc::c_int as uint32_t,
    0x76f988da as libc::c_int as uint32_t,
    0x983e5152 as libc::c_uint,
    0xa831c66d as libc::c_uint,
    0xb00327c8 as libc::c_uint,
    0xbf597fc7 as libc::c_uint,
    0xc6e00bf3 as libc::c_uint,
    0xd5a79147 as libc::c_uint,
    0x6ca6351 as libc::c_int as uint32_t,
    0x14292967 as libc::c_int as uint32_t,
    0x27b70a85 as libc::c_int as uint32_t,
    0x2e1b2138 as libc::c_int as uint32_t,
    0x4d2c6dfc as libc::c_int as uint32_t,
    0x53380d13 as libc::c_int as uint32_t,
    0x650a7354 as libc::c_int as uint32_t,
    0x766a0abb as libc::c_int as uint32_t,
    0x81c2c92e as libc::c_uint,
    0x92722c85 as libc::c_uint,
    0xa2bfe8a1 as libc::c_uint,
    0xa81a664b as libc::c_uint,
    0xc24b8b70 as libc::c_uint,
    0xc76c51a3 as libc::c_uint,
    0xd192e819 as libc::c_uint,
    0xd6990624 as libc::c_uint,
    0xf40e3585 as libc::c_uint,
    0x106aa070 as libc::c_int as uint32_t,
    0x19a4c116 as libc::c_int as uint32_t,
    0x1e376c08 as libc::c_int as uint32_t,
    0x2748774c as libc::c_int as uint32_t,
    0x34b0bcb5 as libc::c_int as uint32_t,
    0x391c0cb3 as libc::c_int as uint32_t,
    0x4ed8aa4a as libc::c_int as uint32_t,
    0x5b9cca4f as libc::c_int as uint32_t,
    0x682e6ff3 as libc::c_int as uint32_t,
    0x748f82ee as libc::c_int as uint32_t,
    0x78a5636f as libc::c_int as uint32_t,
    0x84c87814 as libc::c_uint,
    0x8cc70208 as libc::c_uint,
    0x90befffa as libc::c_uint,
    0xa4506ceb as libc::c_uint,
    0xbef9a3f7 as libc::c_uint,
    0xc67178f2 as libc::c_uint,
];
unsafe extern "C" fn processblock(mut s: *mut sha256, mut buf: *const uint8_t) {
    let mut W: [uint32_t; 64] = [0; 64];
    let mut t1: uint32_t = 0;
    let mut t2: uint32_t = 0;
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        W[i
            as usize] = (*buf.offset((4 as libc::c_int * i) as isize) as uint32_t)
            << 24 as libc::c_int;
        W[i as usize]
            |= (*buf.offset((4 as libc::c_int * i + 1 as libc::c_int) as isize)
                as uint32_t) << 16 as libc::c_int;
        W[i as usize]
            |= (*buf.offset((4 as libc::c_int * i + 2 as libc::c_int) as isize)
                as uint32_t) << 8 as libc::c_int;
        W[i as usize]
            |= *buf.offset((4 as libc::c_int * i + 3 as libc::c_int) as isize)
                as libc::c_uint;
        i += 1;
        i;
    }
    while i < 64 as libc::c_int {
        W[i
            as usize] = (ror(W[(i - 2 as libc::c_int) as usize], 17 as libc::c_int)
            ^ ror(W[(i - 2 as libc::c_int) as usize], 19 as libc::c_int)
            ^ W[(i - 2 as libc::c_int) as usize] >> 10 as libc::c_int)
            .wrapping_add(W[(i - 7 as libc::c_int) as usize])
            .wrapping_add(
                ror(W[(i - 15 as libc::c_int) as usize], 7 as libc::c_int)
                    ^ ror(W[(i - 15 as libc::c_int) as usize], 18 as libc::c_int)
                    ^ W[(i - 15 as libc::c_int) as usize] >> 3 as libc::c_int,
            )
            .wrapping_add(W[(i - 16 as libc::c_int) as usize]);
        i += 1;
        i;
    }
    a = (*s).h[0 as libc::c_int as usize];
    b = (*s).h[1 as libc::c_int as usize];
    c = (*s).h[2 as libc::c_int as usize];
    d = (*s).h[3 as libc::c_int as usize];
    e = (*s).h[4 as libc::c_int as usize];
    f = (*s).h[5 as libc::c_int as usize];
    g = (*s).h[6 as libc::c_int as usize];
    h = (*s).h[7 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        t1 = h
            .wrapping_add(
                ror(e, 6 as libc::c_int) ^ ror(e, 11 as libc::c_int)
                    ^ ror(e, 25 as libc::c_int),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(K[i as usize])
            .wrapping_add(W[i as usize]);
        t2 = (ror(a, 2 as libc::c_int) ^ ror(a, 13 as libc::c_int)
            ^ ror(a, 22 as libc::c_int))
            .wrapping_add(a & b | c & (a | b));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        i += 1;
        i;
    }
    (*s)
        .h[0 as libc::c_int
        as usize] = ((*s).h[0 as libc::c_int as usize] as libc::c_uint).wrapping_add(a)
        as uint32_t as uint32_t;
    (*s)
        .h[1 as libc::c_int
        as usize] = ((*s).h[1 as libc::c_int as usize] as libc::c_uint).wrapping_add(b)
        as uint32_t as uint32_t;
    (*s)
        .h[2 as libc::c_int
        as usize] = ((*s).h[2 as libc::c_int as usize] as libc::c_uint).wrapping_add(c)
        as uint32_t as uint32_t;
    (*s)
        .h[3 as libc::c_int
        as usize] = ((*s).h[3 as libc::c_int as usize] as libc::c_uint).wrapping_add(d)
        as uint32_t as uint32_t;
    (*s)
        .h[4 as libc::c_int
        as usize] = ((*s).h[4 as libc::c_int as usize] as libc::c_uint).wrapping_add(e)
        as uint32_t as uint32_t;
    (*s)
        .h[5 as libc::c_int
        as usize] = ((*s).h[5 as libc::c_int as usize] as libc::c_uint).wrapping_add(f)
        as uint32_t as uint32_t;
    (*s)
        .h[6 as libc::c_int
        as usize] = ((*s).h[6 as libc::c_int as usize] as libc::c_uint).wrapping_add(g)
        as uint32_t as uint32_t;
    (*s)
        .h[7 as libc::c_int
        as usize] = ((*s).h[7 as libc::c_int as usize] as libc::c_uint).wrapping_add(h)
        as uint32_t as uint32_t;
}
unsafe extern "C" fn pad(mut s: *mut sha256) {
    let mut r: libc::c_uint = ((*s).len).wrapping_rem(64 as libc::c_int as libc::c_ulong)
        as libc::c_uint;
    let fresh0 = r;
    r = r.wrapping_add(1);
    (*s).buf[fresh0 as usize] = 0x80 as libc::c_int as uint8_t;
    if r > 56 as libc::c_int as libc::c_uint {
        memset(
            ((*s).buf).as_mut_ptr().offset(r as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (64 as libc::c_int as libc::c_uint).wrapping_sub(r) as libc::c_ulong,
        );
        r = 0 as libc::c_int as libc::c_uint;
        processblock(s, ((*s).buf).as_mut_ptr());
    }
    memset(
        ((*s).buf).as_mut_ptr().offset(r as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (56 as libc::c_int as libc::c_uint).wrapping_sub(r) as libc::c_ulong,
    );
    (*s)
        .len = ((*s).len as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
    (*s).buf[56 as libc::c_int as usize] = ((*s).len >> 56 as libc::c_int) as uint8_t;
    (*s).buf[57 as libc::c_int as usize] = ((*s).len >> 48 as libc::c_int) as uint8_t;
    (*s).buf[58 as libc::c_int as usize] = ((*s).len >> 40 as libc::c_int) as uint8_t;
    (*s).buf[59 as libc::c_int as usize] = ((*s).len >> 32 as libc::c_int) as uint8_t;
    (*s).buf[60 as libc::c_int as usize] = ((*s).len >> 24 as libc::c_int) as uint8_t;
    (*s).buf[61 as libc::c_int as usize] = ((*s).len >> 16 as libc::c_int) as uint8_t;
    (*s).buf[62 as libc::c_int as usize] = ((*s).len >> 8 as libc::c_int) as uint8_t;
    (*s).buf[63 as libc::c_int as usize] = (*s).len as uint8_t;
    processblock(s, ((*s).buf).as_mut_ptr());
}
unsafe extern "C" fn sha256_init(mut s: *mut sha256) {
    (*s).len = 0 as libc::c_int as uint64_t;
    (*s).h[0 as libc::c_int as usize] = 0x6a09e667 as libc::c_int as uint32_t;
    (*s).h[1 as libc::c_int as usize] = 0xbb67ae85 as libc::c_uint;
    (*s).h[2 as libc::c_int as usize] = 0x3c6ef372 as libc::c_int as uint32_t;
    (*s).h[3 as libc::c_int as usize] = 0xa54ff53a as libc::c_uint;
    (*s).h[4 as libc::c_int as usize] = 0x510e527f as libc::c_int as uint32_t;
    (*s).h[5 as libc::c_int as usize] = 0x9b05688c as libc::c_uint;
    (*s).h[6 as libc::c_int as usize] = 0x1f83d9ab as libc::c_int as uint32_t;
    (*s).h[7 as libc::c_int as usize] = 0x5be0cd19 as libc::c_int as uint32_t;
}
unsafe extern "C" fn sha256_sum(mut s: *mut sha256, mut md: *mut uint8_t) {
    let mut i: libc::c_int = 0;
    pad(s);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *md
            .offset(
                (4 as libc::c_int * i) as isize,
            ) = ((*s).h[i as usize] >> 24 as libc::c_int) as uint8_t;
        *md
            .offset(
                (4 as libc::c_int * i + 1 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] >> 16 as libc::c_int) as uint8_t;
        *md
            .offset(
                (4 as libc::c_int * i + 2 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] >> 8 as libc::c_int) as uint8_t;
        *md
            .offset(
                (4 as libc::c_int * i + 3 as libc::c_int) as isize,
            ) = (*s).h[i as usize] as uint8_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn sha256_update(
    mut s: *mut sha256,
    mut m: *const libc::c_void,
    mut len: libc::c_ulong,
) {
    let mut p: *const uint8_t = m as *const uint8_t;
    let mut r: libc::c_uint = ((*s).len).wrapping_rem(64 as libc::c_int as libc::c_ulong)
        as libc::c_uint;
    (*s).len = ((*s).len as libc::c_ulong).wrapping_add(len) as uint64_t as uint64_t;
    if r != 0 {
        if len < (64 as libc::c_int as libc::c_uint).wrapping_sub(r) as libc::c_ulong {
            memcpy(
                ((*s).buf).as_mut_ptr().offset(r as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                len,
            );
            return;
        }
        memcpy(
            ((*s).buf).as_mut_ptr().offset(r as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            (64 as libc::c_int as libc::c_uint).wrapping_sub(r) as libc::c_ulong,
        );
        len = len
            .wrapping_sub(
                (64 as libc::c_int as libc::c_uint).wrapping_sub(r) as libc::c_ulong,
            );
        p = p.offset((64 as libc::c_int as libc::c_uint).wrapping_sub(r) as isize);
        processblock(s, ((*s).buf).as_mut_ptr());
    }
    while len >= 64 as libc::c_int as libc::c_ulong {
        processblock(s, p);
        len = len.wrapping_sub(64 as libc::c_int as libc::c_ulong);
        p = p.offset(64 as libc::c_int as isize);
    }
    memcpy(((*s).buf).as_mut_ptr() as *mut libc::c_void, p as *const libc::c_void, len);
}
#[no_mangle]
pub unsafe extern "C" fn ul_SHA256(
    mut hash_out: *mut libc::c_uchar,
    mut str: *const libc::c_uchar,
    mut len: size_t,
) {
    let mut state: sha256 = {
        let mut init = sha256 {
            len: 0,
            h: [0; 8],
            buf: [0; 64],
        };
        init
    };
    sha256_init(&mut state);
    sha256_update(&mut state, str as *const libc::c_void, len);
    sha256_sum(&mut state, hash_out);
}
