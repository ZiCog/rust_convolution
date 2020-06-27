#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn re_re_conv(mut out: *mut libc::c_float,
                                    mut out_length: *mut libc::c_int,
                                    mut sample: *const libc::c_float,
                                    mut samplelen: libc::c_int,
                                    mut coeff: *const libc::c_float,
                                    mut coefflen: libc::c_int) {
    let mut outlen: libc::c_int = samplelen - coefflen + 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < outlen {
        let mut acc: libc::c_float = 0.0f64 as libc::c_float;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < coefflen {
            acc +=
                *sample.offset((i + j) as isize) * *coeff.offset(j as isize);
            j += 1
        }
        *out.offset(i as isize) = acc;
        i += 1
    }
    *out_length = outlen;
}
unsafe fn main_0() -> libc::c_int {
    let mut sample: *mut libc::c_float =
        malloc(((20 as libc::c_int * 1000 as libc::c_int *
                     1000 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                    as libc::c_ulong)) as
            *mut libc::c_float;
    let mut coeff: *mut libc::c_float =
        malloc((500 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                    as libc::c_ulong)) as
            *mut libc::c_float;
    let mut result_len: libc::c_int = 0;
    let mut result: *mut libc::c_float =
        malloc(((20 as libc::c_int * 1000 as libc::c_int *
                     1000 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                    as libc::c_ulong)) as
            *mut libc::c_float;
    re_re_conv(result, &mut result_len, sample,
               20 as libc::c_int * 1000 as libc::c_int * 1000 as libc::c_int,
               coeff, 500 as libc::c_int);
    printf(b"%f %f\x00" as *const u8 as *const libc::c_char,
           *result.offset(0 as libc::c_int as isize) as libc::c_double,
           *result.offset((20 as libc::c_int * 1000 as libc::c_int *
                               1000 as libc::c_int - 500 as libc::c_int) as
                              isize) as libc::c_double);
    return 0;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
