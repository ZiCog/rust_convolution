#![feature(test)]
#![feature(core_intrinsics)]

use rayon::prelude::*;

extern crate test;

pub mod zso {
    pub fn convolution(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let outlen = sample.len() - coeff.len() + 1;
        let mut out = Vec::with_capacity(outlen);
        for i in 0..outlen {
            let mut acc: f32 = 0.;
            for j in 0..coeff.len() {
                acc += sample[i + j] * coeff[j];
            }
            out.push(acc);
        }
        out
    }

    #[link(name = "conv", kind = "static")]
    extern "C" {
        fn convolution_c(
            output: *mut f32,
            outlen: *mut i32,
            sample: *const f32,
            samplelen: i32,
            coeff: *const f32,
            coefflen: i32,
        );
    }

    pub fn convolution_ffi(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let mut output: Vec<f32> = Vec::with_capacity(sample.len());
        let mut outlen: i32 = 0;
        unsafe {
            convolution_c(
                output.as_mut_ptr(),
                &mut outlen,
                sample.as_ptr(),
                sample.len() as i32,
                coeff.as_ptr(),
                coeff.len() as i32,
            );
            output.set_len(outlen as usize);
        }
        output
    }
}

pub mod bjorn3 {
    pub fn convolution(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let outlen = sample.len() - coeff.len() + 1;
        let mut out = Vec::with_capacity(outlen);
        for i in 0..outlen {
            let mut acc: f32 = 0.;
            for (x, chunk) in coeff.chunks(8).enumerate() {
                acc += chunk
                    .iter()
                    .enumerate()
                    .map(|(j, &c)| unsafe { sample.get_unchecked(i + x * 8 + j) } * c)
                    .sum::<f32>();
            }
            out.push(acc);
        }
        out
    }
}

pub mod dodomorandi {
    pub fn convolution(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        sample
            .windows(coeff.len())
            .map(|window| {
                window
                    .iter()
                    .zip(coeff.iter())
                    .map(|(sample, coeff)| sample * coeff)
                    .sum::<f32>()
            })
            .collect()
    }
}

pub mod pcpthm {
    use std::intrinsics::{fadd_fast, fmul_fast};

    pub fn convolution(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        sample
            .windows(coeff.len())
            .map(|window| {
                window
                    .iter()
                    .zip(coeff.iter())
                    .map(|(sample, coeff)| unsafe { fmul_fast(*sample, *coeff) })
                    .fold(0f32, |acc, value| unsafe { fadd_fast(acc, value) })
            })
            .collect()
    }
}

pub mod zicog {
    use std::intrinsics::{fadd_fast, fmul_fast};

    pub fn convolution(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let mut out: Vec<f32> = vec![0.0; sample.len() - coeff.len() + 1];

        for i in 0..out.len() {
            let mut acc: f32 = 0.0;
            let window = &sample[i..i + coeff.len()];
            for j in 0..window.len() {
                unsafe {
                    acc = fadd_fast(acc, fmul_fast(window[j], coeff[j]));
                }
            }
            out[i] = acc;
        }
        out
    }

    pub fn convolution_safe(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let mut out: Vec<f32> = vec![0.0; sample.len() - coeff.len() + 1];
        for i in 0..out.len() {
            let mut acc: f32 = 0.0;
            let window = &sample[i..i + coeff.len()];
            for j in 0..window.len() {
                acc += window[j] * coeff[j];
            }
            out[i] = acc;
        }
        out
    }

    use fast_floats::*;

    pub fn convolution_fast(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        let mut out: Vec<f32> = vec![0.0; sample.len() - coeff.len() + 1];
        for i in 0..out.len() {
            let mut acc: FF32 = Fast(0.0);
            let window = &sample[i..i + coeff.len()];
            for j in 0..window.len() {
                acc += (window[j] * coeff[j]);
            }
            out[i] = acc.get();
        }
        out
    }
}

pub mod alice {
    use fast_floats::*;

    pub fn dot_product(xs: &[f32], ys: &[f32]) -> f32 {
        xs.iter()
            .zip(ys)
            .fold(Fast(0.), |acc, (&x, &y)| acc + Fast(x) * Fast(y))
            .get()
    }

    use rayon::prelude::*;

    pub fn convolution_serial(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        sample
            // Get sequence of windows that "slide" over the sample data
            .windows(coeff.len())
            // Form the dot product of every window in the sequence
            .map(|window| {
                window
                    .iter()
                    .zip(coeff)
                    .fold(Fast(0.), |acc, (&x, &y)| acc + Fast(x) * Fast(y))
                    .get()
            })
            // Map produces an iterator so we have to assemble a vector from that.
            .collect()
    }

    pub fn convolution_parallel(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
        sample
            // Get sequence of windows that "slide" over the sample data
            .par_windows(coeff.len())
            // Form the dot product of every window in the sequence
            .map(|window| dot_product(window, coeff))
            // Map produces an iterator so we have to assemble a vector from that.
            .collect()
    }
}

pub mod rusty_ron {
    use simdeez::avx2::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;

    simd_compiletime_generate!(
        pub fn re_re_conv_f32(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
            let len = sample.len() - coeff.len() + 1;
            let mut result: Vec<f32> = Vec::with_capacity(len);
            for i in 0..len {
                let mut acc = S::set1_ps(0.0);
                if coeff.len() % S::VF32_WIDTH == 0 {
                    for j in (0..coeff.len()).step_by(S::VF32_WIDTH) {
                        let s = S::loadu_ps(&sample[i + j]);
                        let c = S::loadu_ps(&coeff[j]);
                        acc = S::fmadd_ps(s, c, acc);
                    }
                    let sum = S::horizontal_add_ps(acc);
                    result.push(sum);
                } else {
                    for j in (coeff.len() + 1)..coeff.len() % S::VF32_WIDTH {
                        let sum = (0..coeff.len()).map(|j| sample[i + j] * coeff[j]).sum();
                        result.push(sum);
                    }
                }
            }
            result
        }
    );
}

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

    #[test]
    fn test_re_re_conv_zso() {
        assert_eq!(zso::convolution(&SAMPLE, &COEFF), EXPECTED);
    }
    #[test]
    fn test_re_re_conv_zso_ffi() {
        assert_eq!(zso::convolution_ffi(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_bjorn3() {
        assert_eq!(bjorn3::convolution(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_dodomorandi() {
        assert_eq!(dodomorandi::convolution(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_pcpthm() {
        assert_eq!(pcpthm::convolution(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_zicog() {
        assert_eq!(zicog::convolution(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_zicog_safe() {
        assert_eq!(zicog::convolution_safe(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_zicog_fast() {
        assert_eq!(zicog::convolution_fast(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_alice_serial() {
        assert_eq!(alice::convolution_serial(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_alice_parallel() {
        assert_eq!(alice::convolution_parallel(&SAMPLE, &COEFF), EXPECTED);
    }

    #[test]
    fn test_re_re_conv_rusty_ron() {
        let result = rusty_ron::re_re_conv_f32_compiletime(&SAMPLE, &COEFF);
        println!("{:?}", result);
        assert_eq!(result, EXPECTED);
    }

    #[bench]
    fn bench_re_re_conv_zso(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| zso::convolution(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_zso_ffi(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| zso::convolution_ffi(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_bjorn3(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| bjorn3::convolution(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_dodomorandi(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| dodomorandi::convolution(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_pcpthm(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| pcpthm::convolution(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_zicog(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| zicog::convolution(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_zicog_safe(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| zicog::convolution_safe(&sample, &coeff))
    }
    #[bench]
    fn bench_re_re_conv_zicog_fast(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| zicog::convolution_fast(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_alice_serial(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| alice::convolution_serial(&sample, &coeff))
    }
    #[bench]
    fn bench_re_re_conv_alice_parallel(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| alice::convolution_parallel(&sample, &coeff))
    }

    #[bench]
    fn bench_re_re_conv_rusty_ron(b: &mut Bencher) {
        let sample: Vec<f32> = vec![0.33333; 1024];
        let coeff: Vec<f32> = vec![0.33333; 16];
        b.iter(|| rusty_ron::re_re_conv_f32_compiletime(&sample, &coeff))
    }
}
