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
            acc += window[j] * coeff[j];
        }
        out[i] = acc.get();
    }
    out
}

pub fn convolution_slow(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut out: Vec<f32> = vec![0.0; sample.len() - coeff.len() + 1];
    for i in 0..out.len() {
        let mut acc: FF32 = Fast(0.0);
        for j in 0..coeff.len() {
            acc += sample[i + j] * coeff[j];
        }
        out[i] = acc.get();
    }
    out
}
