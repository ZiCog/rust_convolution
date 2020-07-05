#![feature(test)]
#![feature(core_intrinsics)]
#![feature(concat_idents)]

pub mod alice;
pub mod bjorn3;
pub mod dodomorandi;
pub mod pcg;
pub mod pcpthm;
pub mod zicog;
pub mod zso;

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const SAMPLE: &[f32] = &[
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    const COEFF: &[f32] = &[0.0, -1.0, 1.0, 0.0];
    const EXPECTED: &[f32] = &[
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    // Macro to generate simple test functions
    macro_rules! make_test {
        ($mod_name:ident, $fn_name:ident) => {
            paste::item! {
                #[test]
                fn [< test_ $mod_name _ $fn_name >] () {
                    assert_eq!($mod_name::$fn_name(&SAMPLE, &COEFF), EXPECTED);
                }
            }
        };
    }

    make_test!(zso, convolution);
    make_test!(zso, convolution_ffi);
    make_test!(bjorn3, convolution);
    make_test!(dodomorandi, convolution);
    make_test!(pcpthm, convolution);
    make_test!(zicog, convolution);
    make_test!(zicog, convolution_safe);
    make_test!(zicog, convolution_fast);
    make_test!(alice, convolution_serial);
    make_test!(alice, convolution_parallel);

    // Macro to generate benche mark.
    macro_rules! make_bench {
        // Arguments are module name and function name of function to test bench
        ($test_name:ident, $mod_name:ident, $fn_name:ident) => {
            // The macro will expand into the contents of this block.
            paste::item! {
                #[bench]
                fn [< bench_ $mod_name _ $fn_name >] (b: &mut Bencher) {
                    let sample: Vec<f32> = vec![0.33333; 1024];
                    let coeff: Vec<f32> = vec![0.33333; 16];
                    b.iter(|| $mod_name::$fn_name(&sample, &coeff))
                }
            }
        };
    }

    make_bench!(zso_convolution, zso, convolution);
    make_bench!(zso_convolution_ffi, zso, convolution_ffi);
    make_bench!(bjorn3_convolution, bjorn3, convolution);
    make_bench!(dodomorandi_convolution, dodomorandi, convolution);
    make_bench!(pcpthm_convolution, pcpthm, convolution);
    make_bench!(zicog_convolution, zicog, convolution);
    make_bench!(zicog_convolution_safe, zicog, convolution_safe);
    make_bench!(zicog_convolution_fast, zicog, convolution_fast);
    make_bench!(alice_convolution_serial, alice, convolution_serial);
    make_bench!(alice_convolution_parallel, alice, convolution_parallel);
}
