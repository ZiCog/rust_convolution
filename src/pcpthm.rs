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
