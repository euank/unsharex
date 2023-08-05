use ::c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __libc_current_sigrtmin() -> libc::c_int;
    fn __libc_current_sigrtmax() -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_signal_name {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {}
#[inline]
unsafe extern "C" fn c_tolower(c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn c_strncasecmp(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    while n > 0 as libc::c_int as libc::c_ulong {
        let x: libc::c_uint = *a as libc::c_uint;
        let y: libc::c_uint = *b as libc::c_uint;
        res = c_tolower(x as libc::c_int) - c_tolower(y as libc::c_int);
        if res != 0 {
            break;
        }
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
        n = n.wrapping_sub(1);
        n;
    }
    return res;
}
#[inline]
unsafe extern "C" fn c_strcasecmp(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    if a == b {
        return 0 as libc::c_int;
    }
    while *a as libc::c_int != '\0' as i32 {
        let x: libc::c_uint = *a as libc::c_uint;
        let y: libc::c_uint = *b as libc::c_uint;
        res = c_tolower(x as libc::c_int) - c_tolower(y as libc::c_int);
        if res != 0 {
            break;
        }
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
    }
    return res;
}
static mut ul_signames: [ul_signal_name; 34] = [
    {
        let init = ul_signal_name {
            name: b"HUP\0" as *const u8 as *const libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"INT\0" as *const u8 as *const libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"QUIT\0" as *const u8 as *const libc::c_char,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"ILL\0" as *const u8 as *const libc::c_char,
            val: 4 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"TRAP\0" as *const u8 as *const libc::c_char,
            val: 5 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"ABRT\0" as *const u8 as *const libc::c_char,
            val: 6 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"IOT\0" as *const u8 as *const libc::c_char,
            val: 6 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"BUS\0" as *const u8 as *const libc::c_char,
            val: 7 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"FPE\0" as *const u8 as *const libc::c_char,
            val: 8 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"KILL\0" as *const u8 as *const libc::c_char,
            val: 9 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"USR1\0" as *const u8 as *const libc::c_char,
            val: 10 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"SEGV\0" as *const u8 as *const libc::c_char,
            val: 11 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"USR2\0" as *const u8 as *const libc::c_char,
            val: 12 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"PIPE\0" as *const u8 as *const libc::c_char,
            val: 13 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"ALRM\0" as *const u8 as *const libc::c_char,
            val: 14 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"TERM\0" as *const u8 as *const libc::c_char,
            val: 15 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"STKFLT\0" as *const u8 as *const libc::c_char,
            val: 16 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"CHLD\0" as *const u8 as *const libc::c_char,
            val: 17 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"CLD\0" as *const u8 as *const libc::c_char,
            val: 17 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"CONT\0" as *const u8 as *const libc::c_char,
            val: 18 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"STOP\0" as *const u8 as *const libc::c_char,
            val: 19 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"TSTP\0" as *const u8 as *const libc::c_char,
            val: 20 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"TTIN\0" as *const u8 as *const libc::c_char,
            val: 21 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"TTOU\0" as *const u8 as *const libc::c_char,
            val: 22 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"URG\0" as *const u8 as *const libc::c_char,
            val: 23 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"XCPU\0" as *const u8 as *const libc::c_char,
            val: 24 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"XFSZ\0" as *const u8 as *const libc::c_char,
            val: 25 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"VTALRM\0" as *const u8 as *const libc::c_char,
            val: 26 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"PROF\0" as *const u8 as *const libc::c_char,
            val: 27 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"WINCH\0" as *const u8 as *const libc::c_char,
            val: 28 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"IO\0" as *const u8 as *const libc::c_char,
            val: 29 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"POLL\0" as *const u8 as *const libc::c_char,
            val: 29 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"PWR\0" as *const u8 as *const libc::c_char,
            val: 30 as libc::c_int,
        };
        init
    },
    {
        let init = ul_signal_name {
            name: b"SYS\0" as *const u8 as *const libc::c_char,
            val: 31 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn rtsig_to_signum(mut sig: *const libc::c_char) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut maxi: libc::c_int = 0 as libc::c_int;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    if c_strncasecmp(
        sig,
        b"min+\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    ) == 0 as libc::c_int
    {
        sig = sig.offset(4 as libc::c_int as isize);
    } else if c_strncasecmp(
        sig,
        b"max-\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    ) == 0 as libc::c_int
    {
        sig = sig.offset(4 as libc::c_int as isize);
        maxi = 1 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*sig as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    num = strtol(sig, &mut ep, 10 as libc::c_int) as libc::c_int;
    if ep.is_null()
        || sig == ep as *const libc::c_char
        || *__errno_location() != 0
        || num < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    num = if maxi != 0 {
        __libc_current_sigrtmax() - num
    } else {
        __libc_current_sigrtmin() + num
    };
    if num < __libc_current_sigrtmin() || __libc_current_sigrtmax() < num {
        return -(1 as libc::c_int);
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn signame_to_signum(mut sig: *const libc::c_char) -> libc::c_int {
    let mut n: size_t = 0;
    if c_strncasecmp(
        sig,
        b"sig\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) == 0
    {
        sig = sig.offset(3 as libc::c_int as isize);
    }
    if c_strncasecmp(
        sig,
        b"rt\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    ) == 0
    {
        return rtsig_to_signum(sig.offset(2 as libc::c_int as isize));
    }
    n = 0 as libc::c_int as size_t;
    while n
        < (::core::mem::size_of::<[ul_signal_name; 34]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<ul_signal_name>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if c_strcasecmp(ul_signames[n as usize].name, sig) == 0 {
            return ul_signames[n as usize].val;
        }
        n = n.wrapping_add(1);
        n;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn signum_to_signame(signum: libc::c_int) -> *const libc::c_char {
    let mut n: size_t = 0;
    n = 0 as libc::c_int as size_t;
    while n
        < (::core::mem::size_of::<[ul_signal_name; 34]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<ul_signal_name>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if ul_signames[n as usize].val == signum {
            return ul_signames[n as usize].name;
        }
        n = n.wrapping_add(1);
        n;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_signame_by_idx(
    idx: size_t,
    signame: *mut *const libc::c_char,
    signum: *mut libc::c_int,
) -> libc::c_int {
    if idx
        >= (::core::mem::size_of::<[ul_signal_name; 34]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<ul_signal_name>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    if !signame.is_null() {
        *signame = ul_signames[idx as usize].name;
    }
    if !signum.is_null() {
        *signum = ul_signames[idx as usize].val;
    }
    return 0 as libc::c_int;
}
