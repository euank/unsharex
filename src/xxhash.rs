use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type XXH32_hash_t = uint32_t;
pub type xxh_u32 = XXH32_hash_t;
pub type XXH_alignment = libc::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type xxh_u8 = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub v: [XXH32_hash_t; 4],
    pub mem32: [XXH32_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}
pub type XXH32_state_t = XXH32_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [libc::c_uchar; 4],
}
pub type XXH64_hash_t = uint64_t;
pub type xxh_u64 = XXH64_hash_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [libc::c_uchar; 8],
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32(
    mut input: *const libc::c_void,
    mut len: size_t,
    mut seed: XXH32_hash_t,
) -> XXH32_hash_t {
    return XXH32_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
#[inline(always)]
unsafe extern "C" fn XXH32_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u32,
    mut align: XXH_alignment,
) -> xxh_u32 {
    let mut h32: xxh_u32 = 0;
    input.is_null();
    if len >= 16 as libc::c_int as libc::c_ulong {
        let bEnd: *const xxh_u8 = input.offset(len as isize);
        let limit: *const xxh_u8 = bEnd.offset(-(15 as libc::c_int as isize));
        let mut v1: xxh_u32 = seed
            .wrapping_add(0x9e3779b1 as libc::c_uint)
            .wrapping_add(0x85ebca77 as libc::c_uint);
        let mut v2: xxh_u32 = seed.wrapping_add(0x85ebca77 as libc::c_uint);
        let mut v3: xxh_u32 = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
        let mut v4: xxh_u32 = seed.wrapping_sub(0x9e3779b1 as libc::c_uint);
        loop {
            v1 = XXH32_round(
                v1,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v2 = XXH32_round(
                v2,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v3 = XXH32_round(
                v3,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            v4 = XXH32_round(
                v4,
                XXH_readLE32_align(input as *const libc::c_void, align),
            );
            input = input.offset(4 as libc::c_int as isize);
            if !(input < limit) {
                break;
            }
        }
        h32 = (::core::intrinsics::rotate_left(v1, 1 as libc::c_int as libc::c_uint))
            .wrapping_add(
                ::core::intrinsics::rotate_left(v2, 7 as libc::c_int as libc::c_uint),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v3, 12 as libc::c_int as libc::c_uint),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v4, 18 as libc::c_int as libc::c_uint),
            );
    } else {
        h32 = seed.wrapping_add(0x165667b1 as libc::c_uint);
    }
    h32 = (h32 as libc::c_uint).wrapping_add(len as xxh_u32) as xxh_u32 as xxh_u32;
    return XXH32_finalize(h32, input, len & 15 as libc::c_int as libc::c_ulong, align);
}
unsafe extern "C" fn XXH32_finalize(
    mut h32: xxh_u32,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u32 {
    ptr.is_null();
    if 0 as libc::c_int == 0 {
        len &= 15 as libc::c_int as libc::c_ulong;
        while len >= 4 as libc::c_int as libc::c_ulong {
            h32 = (h32 as libc::c_uint)
                .wrapping_add(
                    (XXH_readLE32_align(ptr as *const libc::c_void, align))
                        .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                ) as xxh_u32 as xxh_u32;
            ptr = ptr.offset(4 as libc::c_int as isize);
            h32 = (::core::intrinsics::rotate_left(
                h32,
                17 as libc::c_int as libc::c_uint,
            ))
                .wrapping_mul(0x27d4eb2f as libc::c_uint);
            len = (len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        while len > 0 as libc::c_int as libc::c_ulong {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            h32 = (h32 as libc::c_uint)
                .wrapping_add(
                    (*fresh0 as libc::c_uint).wrapping_mul(0x165667b1 as libc::c_uint),
                ) as xxh_u32 as xxh_u32;
            h32 = (::core::intrinsics::rotate_left(
                h32,
                11 as libc::c_int as libc::c_uint,
            ))
                .wrapping_mul(0x9e3779b1 as libc::c_uint);
            len = len.wrapping_sub(1);
            len;
        }
        return XXH32_avalanche(h32);
    } else {
        's_489: {
            let mut current_block_119: u64;
            match len & 15 as libc::c_int as libc::c_ulong {
                12 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 9611451750662464386;
                }
                8 => {
                    current_block_119 = 9611451750662464386;
                }
                4 => {
                    current_block_119 = 2450463518093978337;
                }
                13 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 15337429781555974987;
                }
                9 => {
                    current_block_119 = 15337429781555974987;
                }
                5 => {
                    current_block_119 = 14931764840333707944;
                }
                14 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 5169682950046638411;
                }
                10 => {
                    current_block_119 = 5169682950046638411;
                }
                6 => {
                    current_block_119 = 1724834618685525749;
                }
                15 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 1417868130854399360;
                }
                11 => {
                    current_block_119 = 1417868130854399360;
                }
                7 => {
                    current_block_119 = 9690184485344936443;
                }
                3 => {
                    current_block_119 = 1161999164417289389;
                }
                2 => {
                    current_block_119 = 419191944381118145;
                }
                1 => {
                    current_block_119 = 2792171972507379239;
                }
                0 => {
                    current_block_119 = 13517501757814995786;
                }
                _ => {
                    break 's_489;
                }
            }
            match current_block_119 {
                9611451750662464386 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 2450463518093978337;
                }
                15337429781555974987 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 14931764840333707944;
                }
                5169682950046638411 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 1724834618685525749;
                }
                1417868130854399360 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 9690184485344936443;
                }
                _ => {}
            }
            match current_block_119 {
                1724834618685525749 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    let fresh2 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh2 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    let fresh3 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh3 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                14931764840333707944 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    let fresh1 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh1 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                2450463518093978337 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    return XXH32_avalanche(h32);
                }
                9690184485344936443 => {
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (XXH_readLE32_align(ptr as *const libc::c_void, align))
                                .wrapping_mul(0xc2b2ae3d as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as libc::c_int as isize);
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        17 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x27d4eb2f as libc::c_uint);
                    current_block_119 = 1161999164417289389;
                }
                _ => {}
            }
            match current_block_119 {
                1161999164417289389 => {
                    let fresh4 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh4 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    current_block_119 = 419191944381118145;
                }
                _ => {}
            }
            match current_block_119 {
                419191944381118145 => {
                    let fresh5 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh5 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                    current_block_119 = 2792171972507379239;
                }
                _ => {}
            }
            match current_block_119 {
                2792171972507379239 => {
                    let fresh6 = ptr;
                    ptr = ptr.offset(1);
                    h32 = (h32 as libc::c_uint)
                        .wrapping_add(
                            (*fresh6 as libc::c_uint)
                                .wrapping_mul(0x165667b1 as libc::c_uint),
                        ) as xxh_u32 as xxh_u32;
                    h32 = (::core::intrinsics::rotate_left(
                        h32,
                        11 as libc::c_int as libc::c_uint,
                    ))
                        .wrapping_mul(0x9e3779b1 as libc::c_uint);
                }
                _ => {}
            }
            return XXH32_avalanche(h32);
        }
        return h32;
    };
}
unsafe extern "C" fn XXH32_avalanche(mut h32: xxh_u32) -> xxh_u32 {
    h32 ^= h32 >> 15 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(0x85ebca77 as libc::c_uint) as xxh_u32
        as xxh_u32;
    h32 ^= h32 >> 13 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(0xc2b2ae3d as libc::c_uint) as xxh_u32
        as xxh_u32;
    h32 ^= h32 >> 16 as libc::c_int;
    return h32;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32_align(
    mut ptr: *const libc::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return XXH_readLE32(ptr)
    } else {
        return if 1 as libc::c_int != 0 {
            *(ptr as *const xxh_u32)
        } else {
            XXH_swap32(*(ptr as *const xxh_u32))
        }
    };
}
unsafe extern "C" fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    return x << 24 as libc::c_int & 0xff000000 as libc::c_uint
        | x << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32(mut ptr: *const libc::c_void) -> xxh_u32 {
    return if 1 as libc::c_int != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    };
}
unsafe extern "C" fn XXH_read32(mut memPtr: *const libc::c_void) -> xxh_u32 {
    let mut val: xxh_u32 = 0;
    XXH_memcpy(
        &mut val as *mut xxh_u32 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<xxh_u32>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn XXH_memcpy(
    mut dest: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return memcpy(dest, src, size);
}
unsafe extern "C" fn XXH32_round(mut acc: xxh_u32, mut input: xxh_u32) -> xxh_u32 {
    acc = (acc as libc::c_uint)
        .wrapping_add(input.wrapping_mul(0x85ebca77 as libc::c_uint)) as xxh_u32
        as xxh_u32;
    acc = ::core::intrinsics::rotate_left(acc, 13 as libc::c_int as libc::c_uint);
    acc = (acc as libc::c_uint).wrapping_mul(0x9e3779b1 as libc::c_uint) as xxh_u32
        as xxh_u32;
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
        as *mut XXH32_state_t;
}
unsafe extern "C" fn XXH_malloc(mut s: size_t) -> *mut libc::c_void {
    return malloc(s);
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_freeState(
    mut statePtr: *mut XXH32_state_t,
) -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
unsafe extern "C" fn XXH_free(mut p: *mut libc::c_void) {
    free(p);
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    XXH_memcpy(
        dstState as *mut libc::c_void,
        srcState as *const libc::c_void,
        ::core::mem::size_of::<XXH32_state_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_reset(
    mut statePtr: *mut XXH32_state_t,
    mut seed: XXH32_hash_t,
) -> XXH_errorcode {
    let mut state: XXH32_state_t = XXH32_state_t {
        total_len_32: 0,
        large_len: 0,
        v: [0; 4],
        mem32: [0; 4],
        memsize: 0,
        reserved: 0,
    };
    memset(
        &mut state as *mut XXH32_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<XXH32_state_t>() as libc::c_ulong,
    );
    state
        .v[0 as libc::c_int
        as usize] = seed
        .wrapping_add(0x9e3779b1 as libc::c_uint)
        .wrapping_add(0x85ebca77 as libc::c_uint);
    state.v[1 as libc::c_int as usize] = seed.wrapping_add(0x85ebca77 as libc::c_uint);
    state
        .v[2 as libc::c_int
        as usize] = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
    state.v[3 as libc::c_int as usize] = seed.wrapping_sub(0x9e3779b1 as libc::c_uint);
    XXH_memcpy(
        statePtr as *mut libc::c_void,
        &mut state as *mut XXH32_state_t as *const libc::c_void,
        (::core::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<XXH32_hash_t>() as libc::c_ulong),
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_update(
    mut state: *mut XXH32_state_t,
    mut input: *const libc::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    let mut p: *const xxh_u8 = input as *const xxh_u8;
    let bEnd: *const xxh_u8 = p.offset(len as isize);
    (*state)
        .total_len_32 = ((*state).total_len_32 as libc::c_uint)
        .wrapping_add(len as XXH32_hash_t) as XXH32_hash_t as XXH32_hash_t;
    (*state).large_len
        |= ((len >= 16 as libc::c_int as libc::c_ulong) as libc::c_int
            | ((*state).total_len_32 >= 16 as libc::c_int as libc::c_uint)
                as libc::c_int) as XXH32_hash_t;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len)
        < 16 as libc::c_int as libc::c_ulong
    {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            len,
        );
        (*state)
            .memsize = ((*state).memsize as libc::c_uint)
            .wrapping_add(len as XXH32_hash_t) as XXH32_hash_t as XXH32_hash_t;
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            (16 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize) as size_t,
        );
        let mut p32: *const xxh_u32 = ((*state).mem32).as_mut_ptr();
        (*state)
            .v[0 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[0 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        p32;
        (*state)
            .v[1 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[1 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        p32;
        (*state)
            .v[2 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[2 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p32 = p32.offset(1);
        p32;
        (*state)
            .v[3 as libc::c_int
            as usize] = XXH32_round(
            (*state).v[3 as libc::c_int as usize],
            XXH_readLE32(p32 as *const libc::c_void),
        );
        p = p
            .offset(
                (16 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize)
                    as isize,
            );
        (*state).memsize = 0 as libc::c_int as XXH32_hash_t;
    }
    if p <= bEnd.offset(-(16 as libc::c_int as isize)) {
        let limit: *const xxh_u8 = bEnd.offset(-(16 as libc::c_int as isize));
        loop {
            (*state)
                .v[0 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[0 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[1 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[1 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[2 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[2 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            (*state)
                .v[3 as libc::c_int
                as usize] = XXH32_round(
                (*state).v[3 as libc::c_int as usize],
                XXH_readLE32(p as *const libc::c_void),
            );
            p = p.offset(4 as libc::c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
    }
    if p < bEnd {
        XXH_memcpy(
            ((*state).mem32).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            bEnd.offset_from(p) as libc::c_long as size_t,
        );
        (*state).memsize = bEnd.offset_from(p) as libc::c_long as libc::c_uint;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_digest(
    mut state: *const XXH32_state_t,
) -> XXH32_hash_t {
    let mut h32: xxh_u32 = 0;
    if (*state).large_len != 0 {
        h32 = (::core::intrinsics::rotate_left(
            (*state).v[0 as libc::c_int as usize],
            1 as libc::c_int as libc::c_uint,
        ))
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[1 as libc::c_int as usize],
                    7 as libc::c_int as libc::c_uint,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[2 as libc::c_int as usize],
                    12 as libc::c_int as libc::c_uint,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[3 as libc::c_int as usize],
                    18 as libc::c_int as libc::c_uint,
                ),
            );
    } else {
        h32 = ((*state).v[2 as libc::c_int as usize])
            .wrapping_add(0x165667b1 as libc::c_uint);
    }
    h32 = (h32 as libc::c_uint).wrapping_add((*state).total_len_32) as xxh_u32
        as xxh_u32;
    return XXH32_finalize(
        h32,
        ((*state).mem32).as_ptr() as *const xxh_u8,
        (*state).memsize as size_t,
        XXH_aligned,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_canonicalFromHash(
    mut dst: *mut XXH32_canonical_t,
    mut hash: XXH32_hash_t,
) {
    hash = XXH_swap32(hash);
    XXH_memcpy(
        dst as *mut libc::c_void,
        &mut hash as *mut XXH32_hash_t as *const libc::c_void,
        ::core::mem::size_of::<XXH32_canonical_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH32_hashFromCanonical(
    mut src: *const XXH32_canonical_t,
) -> XXH32_hash_t {
    return XXH_readBE32(src as *const libc::c_void);
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const libc::c_void) -> xxh_u32 {
    return if 1 as libc::c_int != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64(
    mut input: *const libc::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    return XXH64_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned);
}
#[inline(always)]
unsafe extern "C" fn XXH64_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u64,
    mut align: XXH_alignment,
) -> xxh_u64 {
    let mut h64: xxh_u64 = 0;
    input.is_null();
    if len >= 32 as libc::c_int as libc::c_ulong {
        let bEnd: *const xxh_u8 = input.offset(len as isize);
        let limit: *const xxh_u8 = bEnd.offset(-(31 as libc::c_int as isize));
        let mut v1: xxh_u64 = (seed as libc::c_ulonglong)
            .wrapping_add(0x9e3779b185ebca87 as libc::c_ulonglong)
            .wrapping_add(0xc2b2ae3d27d4eb4f as libc::c_ulonglong) as xxh_u64;
        let mut v2: xxh_u64 = (seed as libc::c_ulonglong)
            .wrapping_add(0xc2b2ae3d27d4eb4f as libc::c_ulonglong) as xxh_u64;
        let mut v3: xxh_u64 = seed.wrapping_add(0 as libc::c_int as libc::c_ulong);
        let mut v4: xxh_u64 = (seed as libc::c_ulonglong)
            .wrapping_sub(0x9e3779b185ebca87 as libc::c_ulonglong) as xxh_u64;
        loop {
            v1 = XXH64_round(
                v1,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v2 = XXH64_round(
                v2,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v3 = XXH64_round(
                v3,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            v4 = XXH64_round(
                v4,
                XXH_readLE64_align(input as *const libc::c_void, align),
            );
            input = input.offset(8 as libc::c_int as isize);
            if !(input < limit) {
                break;
            }
        }
        h64 = (::core::intrinsics::rotate_left(v1, 1 as libc::c_int as libc::c_ulong))
            .wrapping_add(
                ::core::intrinsics::rotate_left(v2, 7 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v3, 12 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(v4, 18 as libc::c_int as libc::c_ulong),
            );
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4);
    } else {
        h64 = (seed as libc::c_ulonglong)
            .wrapping_add(0x27d4eb2f165667c5 as libc::c_ulonglong) as xxh_u64;
    }
    h64 = (h64 as libc::c_ulong).wrapping_add(len) as xxh_u64 as xxh_u64;
    return XXH64_finalize(h64, input, len, align);
}
unsafe extern "C" fn XXH64_finalize(
    mut h64: xxh_u64,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u64 {
    ptr.is_null();
    len &= 31 as libc::c_int as libc::c_ulong;
    while len >= 8 as libc::c_int as libc::c_ulong {
        let k1: xxh_u64 = XXH64_round(
            0 as libc::c_int as xxh_u64,
            XXH_readLE64_align(ptr as *const libc::c_void, align),
        );
        ptr = ptr.offset(8 as libc::c_int as isize);
        h64 ^= k1;
        h64 = (::core::intrinsics::rotate_left(h64, 27 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(0x9e3779b185ebca87 as libc::c_ulonglong)
            .wrapping_add(0x85ebca77c2b2ae63 as libc::c_ulonglong) as xxh_u64;
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    if len >= 4 as libc::c_int as libc::c_ulong {
        h64 = (h64 as libc::c_ulonglong
            ^ (XXH_readLE32_align(ptr as *const libc::c_void, align) as xxh_u64
                as libc::c_ulonglong)
                .wrapping_mul(0x9e3779b185ebca87 as libc::c_ulonglong)) as xxh_u64;
        ptr = ptr.offset(4 as libc::c_int as isize);
        h64 = (::core::intrinsics::rotate_left(h64, 23 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(0xc2b2ae3d27d4eb4f as libc::c_ulonglong)
            .wrapping_add(0x165667b19e3779f9 as libc::c_ulonglong) as xxh_u64;
        len = (len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len > 0 as libc::c_int as libc::c_ulong {
        let fresh7 = ptr;
        ptr = ptr.offset(1);
        h64 = (h64 as libc::c_ulonglong
            ^ (*fresh7 as libc::c_ulonglong)
                .wrapping_mul(0x27d4eb2f165667c5 as libc::c_ulonglong)) as xxh_u64;
        h64 = (::core::intrinsics::rotate_left(h64, 11 as libc::c_int as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_mul(0x9e3779b185ebca87 as libc::c_ulonglong) as xxh_u64;
        len = len.wrapping_sub(1);
        len;
    }
    return XXH64_avalanche(h64);
}
unsafe extern "C" fn XXH64_avalanche(mut h64: xxh_u64) -> xxh_u64 {
    h64 ^= h64 >> 33 as libc::c_int;
    h64 = (h64 as libc::c_ulonglong)
        .wrapping_mul(0xc2b2ae3d27d4eb4f as libc::c_ulonglong) as xxh_u64 as xxh_u64;
    h64 ^= h64 >> 29 as libc::c_int;
    h64 = (h64 as libc::c_ulonglong)
        .wrapping_mul(0x165667b19e3779f9 as libc::c_ulonglong) as xxh_u64 as xxh_u64;
    h64 ^= h64 >> 32 as libc::c_int;
    return h64;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE64_align(
    mut ptr: *const libc::c_void,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return XXH_readLE64(ptr)
    } else {
        return if 1 as libc::c_int != 0 {
            *(ptr as *const xxh_u64)
        } else {
            XXH_swap64(*(ptr as *const xxh_u64))
        }
    };
}
unsafe extern "C" fn XXH_swap64(mut x: xxh_u64) -> xxh_u64 {
    return ((x << 56 as libc::c_int) as libc::c_ulonglong
        & 0xff00000000000000 as libc::c_ulonglong
        | (x << 40 as libc::c_int) as libc::c_ulonglong
            & 0xff000000000000 as libc::c_ulonglong
        | (x << 24 as libc::c_int) as libc::c_ulonglong
            & 0xff0000000000 as libc::c_ulonglong
        | (x << 8 as libc::c_int) as libc::c_ulonglong
            & 0xff00000000 as libc::c_ulonglong
        | (x >> 8 as libc::c_int) as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong
        | (x >> 24 as libc::c_int) as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong
        | (x >> 40 as libc::c_int) as libc::c_ulonglong & 0xff00 as libc::c_ulonglong
        | (x >> 56 as libc::c_int) as libc::c_ulonglong & 0xff as libc::c_ulonglong)
        as xxh_u64;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE64(mut ptr: *const libc::c_void) -> xxh_u64 {
    return if 1 as libc::c_int != 0 {
        XXH_read64(ptr)
    } else {
        XXH_swap64(XXH_read64(ptr))
    };
}
unsafe extern "C" fn XXH_read64(mut memPtr: *const libc::c_void) -> xxh_u64 {
    let mut val: xxh_u64 = 0;
    XXH_memcpy(
        &mut val as *mut xxh_u64 as *mut libc::c_void,
        memPtr,
        ::core::mem::size_of::<xxh_u64>() as libc::c_ulong,
    );
    return val;
}
unsafe extern "C" fn XXH64_round(mut acc: xxh_u64, mut input: xxh_u64) -> xxh_u64 {
    acc = (acc as libc::c_ulonglong)
        .wrapping_add(
            (input as libc::c_ulonglong)
                .wrapping_mul(0xc2b2ae3d27d4eb4f as libc::c_ulonglong),
        ) as xxh_u64 as xxh_u64;
    acc = ::core::intrinsics::rotate_left(acc, 31 as libc::c_int as libc::c_ulong);
    acc = (acc as libc::c_ulonglong)
        .wrapping_mul(0x9e3779b185ebca87 as libc::c_ulonglong) as xxh_u64 as xxh_u64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: xxh_u64, mut val: xxh_u64) -> xxh_u64 {
    val = XXH64_round(0 as libc::c_int as xxh_u64, val);
    acc ^= val;
    acc = (acc as libc::c_ulonglong)
        .wrapping_mul(0x9e3779b185ebca87 as libc::c_ulonglong)
        .wrapping_add(0x85ebca77c2b2ae63 as libc::c_ulonglong) as xxh_u64;
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_createState() -> *mut XXH64_state_t {
    return XXH_malloc(::core::mem::size_of::<XXH64_state_t>() as libc::c_ulong)
        as *mut XXH64_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_freeState(
    mut statePtr: *mut XXH64_state_t,
) -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_copyState(
    mut dstState: *mut XXH64_state_t,
    mut srcState: *const XXH64_state_t,
) {
    XXH_memcpy(
        dstState as *mut libc::c_void,
        srcState as *const libc::c_void,
        ::core::mem::size_of::<XXH64_state_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_reset(
    mut statePtr: *mut XXH64_state_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    let mut state: XXH64_state_t = XXH64_state_t {
        total_len: 0,
        v: [0; 4],
        mem64: [0; 4],
        memsize: 0,
        reserved32: 0,
        reserved64: 0,
    };
    memset(
        &mut state as *mut XXH64_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<XXH64_state_t>() as libc::c_ulong,
    );
    state
        .v[0 as libc::c_int
        as usize] = (seed as libc::c_ulonglong)
        .wrapping_add(0x9e3779b185ebca87 as libc::c_ulonglong)
        .wrapping_add(0xc2b2ae3d27d4eb4f as libc::c_ulonglong) as XXH64_hash_t;
    state
        .v[1 as libc::c_int
        as usize] = (seed as libc::c_ulonglong)
        .wrapping_add(0xc2b2ae3d27d4eb4f as libc::c_ulonglong) as XXH64_hash_t;
    state
        .v[2 as libc::c_int
        as usize] = seed.wrapping_add(0 as libc::c_int as libc::c_ulong);
    state
        .v[3 as libc::c_int
        as usize] = (seed as libc::c_ulonglong)
        .wrapping_sub(0x9e3779b185ebca87 as libc::c_ulonglong) as XXH64_hash_t;
    XXH_memcpy(
        statePtr as *mut libc::c_void,
        &mut state as *mut XXH64_state_t as *const libc::c_void,
        (::core::mem::size_of::<XXH64_state_t>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<XXH64_hash_t>() as libc::c_ulong),
    );
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_update(
    mut state: *mut XXH64_state_t,
    mut input: *const libc::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        return XXH_OK;
    }
    let mut p: *const xxh_u8 = input as *const xxh_u8;
    let bEnd: *const xxh_u8 = p.offset(len as isize);
    (*state)
        .total_len = ((*state).total_len as libc::c_ulong).wrapping_add(len)
        as XXH64_hash_t as XXH64_hash_t;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len)
        < 32 as libc::c_int as libc::c_ulong
    {
        XXH_memcpy(
            (((*state).mem64).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            len,
        );
        (*state)
            .memsize = ((*state).memsize as libc::c_uint).wrapping_add(len as xxh_u32)
            as XXH32_hash_t as XXH32_hash_t;
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (((*state).mem64).as_mut_ptr() as *mut xxh_u8)
                .offset((*state).memsize as isize) as *mut libc::c_void,
            input,
            (32 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize) as size_t,
        );
        (*state)
            .v[0 as libc::c_int
            as usize] = XXH64_round(
            (*state).v[0 as libc::c_int as usize],
            XXH_readLE64(
                ((*state).mem64).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
            ),
        );
        (*state)
            .v[1 as libc::c_int
            as usize] = XXH64_round(
            (*state).v[1 as libc::c_int as usize],
            XXH_readLE64(
                ((*state).mem64).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
            ),
        );
        (*state)
            .v[2 as libc::c_int
            as usize] = XXH64_round(
            (*state).v[2 as libc::c_int as usize],
            XXH_readLE64(
                ((*state).mem64).as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *const libc::c_void,
            ),
        );
        (*state)
            .v[3 as libc::c_int
            as usize] = XXH64_round(
            (*state).v[3 as libc::c_int as usize],
            XXH_readLE64(
                ((*state).mem64).as_mut_ptr().offset(3 as libc::c_int as isize)
                    as *const libc::c_void,
            ),
        );
        p = p
            .offset(
                (32 as libc::c_int as libc::c_uint).wrapping_sub((*state).memsize)
                    as isize,
            );
        (*state).memsize = 0 as libc::c_int as XXH32_hash_t;
    }
    if p.offset(32 as libc::c_int as isize) <= bEnd {
        let limit: *const xxh_u8 = bEnd.offset(-(32 as libc::c_int as isize));
        loop {
            (*state)
                .v[0 as libc::c_int
                as usize] = XXH64_round(
                (*state).v[0 as libc::c_int as usize],
                XXH_readLE64(p as *const libc::c_void),
            );
            p = p.offset(8 as libc::c_int as isize);
            (*state)
                .v[1 as libc::c_int
                as usize] = XXH64_round(
                (*state).v[1 as libc::c_int as usize],
                XXH_readLE64(p as *const libc::c_void),
            );
            p = p.offset(8 as libc::c_int as isize);
            (*state)
                .v[2 as libc::c_int
                as usize] = XXH64_round(
                (*state).v[2 as libc::c_int as usize],
                XXH_readLE64(p as *const libc::c_void),
            );
            p = p.offset(8 as libc::c_int as isize);
            (*state)
                .v[3 as libc::c_int
                as usize] = XXH64_round(
                (*state).v[3 as libc::c_int as usize],
                XXH_readLE64(p as *const libc::c_void),
            );
            p = p.offset(8 as libc::c_int as isize);
            if !(p <= limit) {
                break;
            }
        }
    }
    if p < bEnd {
        XXH_memcpy(
            ((*state).mem64).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            bEnd.offset_from(p) as libc::c_long as size_t,
        );
        (*state).memsize = bEnd.offset_from(p) as libc::c_long as libc::c_uint;
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_digest(
    mut state: *const XXH64_state_t,
) -> XXH64_hash_t {
    let mut h64: xxh_u64 = 0;
    if (*state).total_len >= 32 as libc::c_int as libc::c_ulong {
        h64 = (::core::intrinsics::rotate_left(
            (*state).v[0 as libc::c_int as usize],
            1 as libc::c_int as libc::c_ulong,
        ))
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[1 as libc::c_int as usize],
                    7 as libc::c_int as libc::c_ulong,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[2 as libc::c_int as usize],
                    12 as libc::c_int as libc::c_ulong,
                ),
            )
            .wrapping_add(
                ::core::intrinsics::rotate_left(
                    (*state).v[3 as libc::c_int as usize],
                    18 as libc::c_int as libc::c_ulong,
                ),
            );
        h64 = XXH64_mergeRound(h64, (*state).v[0 as libc::c_int as usize]);
        h64 = XXH64_mergeRound(h64, (*state).v[1 as libc::c_int as usize]);
        h64 = XXH64_mergeRound(h64, (*state).v[2 as libc::c_int as usize]);
        h64 = XXH64_mergeRound(h64, (*state).v[3 as libc::c_int as usize]);
    } else {
        h64 = ((*state).v[2 as libc::c_int as usize] as libc::c_ulonglong)
            .wrapping_add(0x27d4eb2f165667c5 as libc::c_ulonglong) as xxh_u64;
    }
    h64 = (h64 as libc::c_ulong).wrapping_add((*state).total_len) as xxh_u64 as xxh_u64;
    return XXH64_finalize(
        h64,
        ((*state).mem64).as_ptr() as *const xxh_u8,
        (*state).total_len,
        XXH_aligned,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_canonicalFromHash(
    mut dst: *mut XXH64_canonical_t,
    mut hash: XXH64_hash_t,
) {
    hash = XXH_swap64(hash);
    XXH_memcpy(
        dst as *mut libc::c_void,
        &mut hash as *mut XXH64_hash_t as *const libc::c_void,
        ::core::mem::size_of::<XXH64_canonical_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH64_hashFromCanonical(
    mut src: *const XXH64_canonical_t,
) -> XXH64_hash_t {
    return XXH_readBE64(src as *const libc::c_void);
}
unsafe extern "C" fn XXH_readBE64(mut ptr: *const libc::c_void) -> xxh_u64 {
    return if 1 as libc::c_int != 0 {
        XXH_swap64(XXH_read64(ptr))
    } else {
        XXH_read64(ptr)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_XXH_versionNumber() -> libc::c_uint {
    return (0 as libc::c_int * 100 as libc::c_int * 100 as libc::c_int
        + 8 as libc::c_int * 100 as libc::c_int + 1 as libc::c_int) as libc::c_uint;
}
