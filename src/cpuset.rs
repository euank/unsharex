use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn __sched_cpufree(__set: *mut cpu_set_t);
    fn __sched_cpualloc(__count: size_t) -> *mut cpu_set_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn val_to_char(mut v: libc::c_int) -> libc::c_int {
    if v >= 0 as libc::c_int && v < 10 as libc::c_int {
        return '0' as i32 + v;
    }
    if v >= 10 as libc::c_int && v < 16 as libc::c_int {
        return 'a' as i32 - 10 as libc::c_int + v;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn char_to_val(mut c: libc::c_int) -> libc::c_int {
    let mut cl: libc::c_int = 0;
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    cl = ({
        let mut __res: libc::c_int = 0;
        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = c;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(c);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(c as isize);
        }
        __res
    });
    if cl >= 'a' as i32 && cl <= 'f' as i32 {
        return cl + (10 as libc::c_int - 'a' as i32);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn nexttoken(
    mut q: *const libc::c_char,
    mut sep: libc::c_int,
) -> *const libc::c_char {
    if !q.is_null() {
        q = strchr(q, sep);
    }
    if !q.is_null() {
        q = q.offset(1);
        q;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn get_max_number_of_cpus() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut cpus: libc::c_int = 2048 as libc::c_int;
    let mut setsize: size_t = 0;
    let mut set: *mut cpu_set_t = cpuset_alloc(cpus, &mut setsize, 0 as *mut size_t);
    if set.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        libc::memset(set as *mut libc::c_void, '\0' as i32, setsize as libc::size_t);
        n = syscall(204 as libc::c_int as libc::c_long, 0 as libc::c_int, setsize, set)
            as libc::c_int;
        if n < 0 as libc::c_int && *__errno_location() == 22 as libc::c_int
            && cpus < 1024 as libc::c_int * 1024 as libc::c_int
        {
            cpuset_free(set);
            cpus *= 2 as libc::c_int;
            set = cpuset_alloc(cpus, &mut setsize, 0 as *mut size_t);
            if set.is_null() {
                return -(1 as libc::c_int);
            }
        } else {
            cpuset_free(set);
            return n * 8 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cpuset_alloc(
    mut ncpus: libc::c_int,
    mut setsize: *mut size_t,
    mut nbits: *mut size_t,
) -> *mut cpu_set_t {
    let mut set: *mut cpu_set_t = __sched_cpualloc(ncpus as size_t);
    if set.is_null() {
        return 0 as *mut cpu_set_t;
    }
    if !setsize.is_null() {
        *setsize = (ncpus as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<__cpu_mask>() as libc::c_ulong),
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<__cpu_mask>() as libc::c_ulong),
            )
            .wrapping_mul(::core::mem::size_of::<__cpu_mask>() as libc::c_ulong);
    }
    if !nbits.is_null() {
        *nbits = (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (ncpus as libc::c_ulong)
                    .wrapping_add(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_mul(::core::mem::size_of::<__cpu_mask>() as libc::c_ulong),
            );
    }
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn cpuset_free(mut set: *mut cpu_set_t) {
    __sched_cpufree(set);
}
#[no_mangle]
pub unsafe extern "C" fn cpulist_create(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut set: *mut cpu_set_t,
    mut setsize: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut ptr: *mut libc::c_char = str;
    let mut entry_made: libc::c_int = 0 as libc::c_int;
    let mut max: size_t = (8 as libc::c_int as libc::c_ulong).wrapping_mul(setsize);
    i = 0 as libc::c_int as size_t;
    while i < max {
        if ({
            let mut __cpu: size_t = i;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    & (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
            } else {
                0 as libc::c_int
            }
        }) != 0
        {
            let mut rlen: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut run: size_t = 0 as libc::c_int as size_t;
            entry_made = 1 as libc::c_int;
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < max {
                if !(({
                    let mut __cpu: size_t = j;
                    if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                        (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                            .offset(
                                __cpu
                                    .wrapping_div(
                                        (8 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                            ),
                                    ) as isize,
                            )
                            & (1 as libc::c_int as __cpu_mask)
                                << __cpu
                                    .wrapping_rem(
                                        (8 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                            ),
                                    ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                    } else {
                        0 as libc::c_int
                    }
                }) != 0)
                {
                    break;
                }
                run = run.wrapping_add(1);
                run;
                j = j.wrapping_add(1);
                j;
            }
            if run == 0 {
                rlen = snprintf(
                    ptr,
                    len,
                    b"%zu,\0" as *const u8 as *const libc::c_char,
                    i,
                );
            } else if run == 1 as libc::c_int as libc::c_ulong {
                rlen = snprintf(
                    ptr,
                    len,
                    b"%zu,%zu,\0" as *const u8 as *const libc::c_char,
                    i,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                i = i.wrapping_add(1);
                i;
            } else {
                rlen = snprintf(
                    ptr,
                    len,
                    b"%zu-%zu,\0" as *const u8 as *const libc::c_char,
                    i,
                    i.wrapping_add(run),
                );
                i = (i as libc::c_ulong).wrapping_add(run) as size_t as size_t;
            }
            if rlen < 0 as libc::c_int || rlen as size_t >= len {
                return 0 as *mut libc::c_char;
            }
            ptr = ptr.offset(rlen as isize);
            len = (len as libc::c_ulong).wrapping_sub(rlen as libc::c_ulong) as size_t
                as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    ptr = ptr.offset(-(entry_made as isize));
    *ptr = '\0' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn cpumask_create(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut set: *mut cpu_set_t,
    mut setsize: size_t,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = str;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cpu: libc::c_int = 0;
    cpu = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(setsize)
        .wrapping_sub(4 as libc::c_int as libc::c_ulong) as libc::c_int;
    while cpu >= 0 as libc::c_int {
        let mut val: libc::c_char = 0 as libc::c_int as libc::c_char;
        if len == ptr.offset_from(str) as libc::c_long as size_t {
            break;
        }
        if ({
            let mut __cpu: size_t = cpu as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    & (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
            } else {
                0 as libc::c_int
            }
        }) != 0
        {
            val = (val as libc::c_int | 1 as libc::c_int) as libc::c_char;
        }
        if ({
            let mut __cpu: size_t = (cpu + 1 as libc::c_int) as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    & (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
            } else {
                0 as libc::c_int
            }
        }) != 0
        {
            val = (val as libc::c_int | 2 as libc::c_int) as libc::c_char;
        }
        if ({
            let mut __cpu: size_t = (cpu + 2 as libc::c_int) as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    & (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
            } else {
                0 as libc::c_int
            }
        }) != 0
        {
            val = (val as libc::c_int | 4 as libc::c_int) as libc::c_char;
        }
        if ({
            let mut __cpu: size_t = (cpu + 3 as libc::c_int) as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                (*(((*set).__bits).as_mut_ptr() as *const __cpu_mask)
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    & (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) != 0 as libc::c_int as libc::c_ulong) as libc::c_int
            } else {
                0 as libc::c_int
            }
        }) != 0
        {
            val = (val as libc::c_int | 8 as libc::c_int) as libc::c_char;
        }
        if ret.is_null() && val as libc::c_int != 0 {
            ret = ptr;
        }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = val_to_char(val as libc::c_int) as libc::c_char;
        cpu -= 4 as libc::c_int;
    }
    *ptr = '\0' as i32 as libc::c_char;
    return if !ret.is_null() { ret } else { ptr.offset(-(1 as libc::c_int as isize)) };
}
#[no_mangle]
pub unsafe extern "C" fn cpumask_parse(
    mut str: *const libc::c_char,
    mut set: *mut cpu_set_t,
    mut setsize: size_t,
) -> libc::c_int {
    let mut len: libc::c_int = strlen(str) as libc::c_int;
    let mut ptr: *const libc::c_char = str
        .offset(len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut cpu: libc::c_int = 0 as libc::c_int;
    if len > 1 as libc::c_int
        && memcmp(
            str as *const libc::c_void,
            b"0x\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_long as libc::c_ulong,
        ) == 0
    {
        str = str.offset(2 as libc::c_int as isize);
    }
    libc::memset(set as *mut libc::c_void, '\0' as i32, setsize as libc::size_t);
    while ptr >= str {
        let mut val: libc::c_char = 0;
        if *ptr as libc::c_int == ',' as i32 {
            ptr = ptr.offset(-1);
            ptr;
        }
        val = char_to_val(*ptr as libc::c_int) as libc::c_char;
        if val as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            return -(1 as libc::c_int);
        }
        if val as libc::c_int & 1 as libc::c_int != 0 {
            let mut __cpu: size_t = cpu as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                let ref mut fresh1 = *((*set).__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh1
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
        }
        if val as libc::c_int & 2 as libc::c_int != 0 {
            let mut __cpu_0: size_t = (cpu + 1 as libc::c_int) as size_t;
            if __cpu_0.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                let ref mut fresh2 = *((*set).__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu_0
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh2
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu_0
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
        }
        if val as libc::c_int & 4 as libc::c_int != 0 {
            let mut __cpu_1: size_t = (cpu + 2 as libc::c_int) as size_t;
            if __cpu_1.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                let ref mut fresh3 = *((*set).__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu_1
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh3
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu_1
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
        }
        if val as libc::c_int & 8 as libc::c_int != 0 {
            let mut __cpu_2: size_t = (cpu + 3 as libc::c_int) as size_t;
            if __cpu_2.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                let ref mut fresh4 = *((*set).__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu_2
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh4
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu_2
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
        }
        ptr = ptr.offset(-1);
        ptr;
        cpu += 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn nextnumber(
    mut str: *const libc::c_char,
    mut end: *mut *mut libc::c_char,
    mut result: *mut libc::c_uint,
) -> libc::c_int {
    *__errno_location() = 0 as libc::c_int;
    if str.is_null() || *str as libc::c_int == '\0' as i32
        || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(22 as libc::c_int);
    }
    *result = strtoul(str, end, 10 as libc::c_int) as libc::c_uint;
    if *__errno_location() != 0 {
        return -*__errno_location();
    }
    if str == *end as *const libc::c_char {
        return -(22 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cpulist_parse(
    mut str: *const libc::c_char,
    mut set: *mut cpu_set_t,
    mut setsize: size_t,
    mut fail: libc::c_int,
) -> libc::c_int {
    let mut max: size_t = (8 as libc::c_int as libc::c_ulong).wrapping_mul(setsize);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    q = str;
    libc::memset(set as *mut libc::c_void, '\0' as i32, setsize as libc::size_t);
    loop {
        p = q;
        q = nexttoken(q, ',' as i32);
        if p.is_null() {
            break;
        }
        let mut a: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut s: libc::c_uint = 0;
        let mut c1: *const libc::c_char = 0 as *const libc::c_char;
        let mut c2: *const libc::c_char = 0 as *const libc::c_char;
        if nextnumber(p, &mut end, &mut a) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        b = a;
        s = 1 as libc::c_int as libc::c_uint;
        p = end;
        c1 = nexttoken(p, '-' as i32);
        c2 = nexttoken(p, ',' as i32);
        if !c1.is_null() && (c2.is_null() || c1 < c2) {
            if nextnumber(c1, &mut end, &mut b) != 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            c1 = if !end.is_null() && *end as libc::c_int != 0 {
                nexttoken(end, ':' as i32)
            } else {
                0 as *const libc::c_char
            };
            if !c1.is_null() && (c2.is_null() || c1 < c2) {
                if nextnumber(c1, &mut end, &mut s) != 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                if s == 0 as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int;
                }
            }
        }
        if !(a <= b) {
            return 1 as libc::c_int;
        }
        while a <= b {
            if fail != 0 && a as libc::c_ulong >= max {
                return 2 as libc::c_int;
            }
            let mut __cpu: size_t = a as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) < setsize {
                let ref mut fresh5 = *((*set).__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh5
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
            a = a.wrapping_add(s);
        }
    }
    if !end.is_null() && *end as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
