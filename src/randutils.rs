use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn jrand48(__xsubi: *mut libc::c_ushort) -> libc::c_long;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn getrandom(
        __buffer: *mut libc::c_void,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[inline]
unsafe extern "C" fn xusleep(mut usec: useconds_t) -> libc::c_int {
    let mut waittime: timespec = {
        let mut init = timespec {
            tv_sec: usec as libc::c_long / 1000000 as libc::c_long,
            tv_nsec: usec as libc::c_long % 1000000 as libc::c_long
                * 1000 as libc::c_int as libc::c_long,
        };
        init
    };
    return nanosleep(&mut waittime, 0 as *mut timespec);
}
#[thread_local]
static mut ul_jrand_seed: [libc::c_ushort; 3] = [0; 3];
#[no_mangle]
pub unsafe extern "C" fn rand_get_number(
    mut low_n: libc::c_int,
    mut high_n: libc::c_int,
) -> libc::c_int {
    return (random() % (high_n - low_n + 1 as libc::c_int) as libc::c_long
        + low_n as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn crank_random() {
    let mut i: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut n_pid: libc::c_uint = 0;
    let mut n_uid: libc::c_uint = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    n_pid = getpid() as libc::c_uint;
    n_uid = getuid();
    srandom(
        ((n_pid << 16 as libc::c_int ^ n_uid) as libc::c_long ^ tv.tv_sec ^ tv.tv_usec)
            as libc::c_uint,
    );
    ul_jrand_seed[0 as libc::c_int
        as usize] = (getpid() as libc::c_long
        ^ tv.tv_sec & 0xffff as libc::c_int as libc::c_long) as libc::c_ushort;
    ul_jrand_seed[1 as libc::c_int
        as usize] = (getppid() as libc::c_long
        ^ tv.tv_usec & 0xffff as libc::c_int as libc::c_long) as libc::c_ushort;
    ul_jrand_seed[2 as libc::c_int
        as usize] = ((tv.tv_sec ^ tv.tv_usec) >> 16 as libc::c_int) as libc::c_ushort;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    i = ((tv.tv_sec ^ tv.tv_usec) & 0x1f as libc::c_int as libc::c_long) as libc::c_int;
    while i > 0 as libc::c_int {
        random();
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn random_get_fd() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = open(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        fd = open(
            b"/dev/random\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int | 0o4000 as libc::c_int | 0o2000000 as libc::c_int,
        );
    }
    crank_random();
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn ul_random_get_bytes(
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
) -> libc::c_int {
    let mut cp: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut i: size_t = 0;
    let mut n: size_t = nbytes;
    let mut lose_counter: libc::c_int = 0 as libc::c_int;
    while n > 0 as libc::c_int as libc::c_ulong {
        let mut x: libc::c_int = 0;
        *__errno_location() = 0 as libc::c_int;
        x = getrandom(cp as *mut libc::c_void, n, 0x1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        if x > 0 as libc::c_int {
            n = (n as libc::c_ulong).wrapping_sub(x as libc::c_ulong) as size_t
                as size_t;
            cp = cp.offset(x as isize);
            lose_counter = 0 as libc::c_int;
            *__errno_location() = 0 as libc::c_int;
        } else {
            if *__errno_location() == 38 as libc::c_int {
                break;
            }
            if !(*__errno_location() == 11 as libc::c_int
                && lose_counter < 8 as libc::c_int)
            {
                break;
            }
            xusleep(125000 as libc::c_int as useconds_t);
            lose_counter += 1;
            lose_counter;
        }
    }
    if *__errno_location() == 38 as libc::c_int {
        let mut fd: libc::c_int = random_get_fd();
        lose_counter = 0 as libc::c_int;
        if fd >= 0 as libc::c_int {
            while n > 0 as libc::c_int as libc::c_ulong {
                let mut x_0: ssize_t = read(fd, cp as *mut libc::c_void, n);
                if x_0 <= 0 as libc::c_int as libc::c_long {
                    let fresh0 = lose_counter;
                    lose_counter = lose_counter + 1;
                    if fresh0 > 8 as libc::c_int {
                        break;
                    }
                    xusleep(125000 as libc::c_int as useconds_t);
                } else {
                    n = (n as libc::c_ulong).wrapping_sub(x_0 as libc::c_ulong) as size_t
                        as size_t;
                    cp = cp.offset(x_0 as isize);
                    lose_counter = 0 as libc::c_int;
                }
            }
            close(fd);
        }
    }
    crank_random();
    cp = buf as *mut libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < nbytes {
        let fresh1 = cp;
        cp = cp.offset(1);
        *fresh1 = (*fresh1 as libc::c_long
            ^ random() >> 7 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    let mut tmp_seed: [libc::c_ushort; 3] = [0; 3];
    memcpy(
        tmp_seed.as_mut_ptr() as *mut libc::c_void,
        ul_jrand_seed.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_ushort; 3]>() as libc::c_ulong,
    );
    ul_jrand_seed[2 as libc::c_int
        as usize] = (ul_jrand_seed[2 as libc::c_int as usize] as libc::c_long
        ^ syscall(186 as libc::c_int as libc::c_long)) as libc::c_ushort;
    cp = buf as *mut libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < nbytes {
        let fresh2 = cp;
        cp = cp.offset(1);
        *fresh2 = (*fresh2 as libc::c_long
            ^ jrand48(tmp_seed.as_mut_ptr()) >> 7 as libc::c_int
                & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    memcpy(
        ul_jrand_seed.as_mut_ptr() as *mut libc::c_void,
        tmp_seed.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[libc::c_ushort; 3]>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    );
    return (n != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn random_tell_source() -> *const libc::c_char {
    return dcgettext(
        0 as *const libc::c_char,
        b"getrandom() function\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
}
