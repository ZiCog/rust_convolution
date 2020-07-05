use fast_floats::*;

pub fn dot_product(xs: &[f32], ys: &[f32]) -> f32 {
    xs.iter()
        .zip(ys)
        .fold(Fast(0.), |acc, (&x, &y)| acc + Fast(x) * Fast(y))
        .get()
}

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

use rayon::prelude::*;

pub fn convolution_parallel(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    sample
        // Get sequence of windows that "slide" over the sample data
        .par_windows(coeff.len())
        // Form the dot product of every window in the sequence
        .map(|window| dot_product(window, coeff))
        // Map produces an iterator so we have to assemble a vector from that.
        .collect()
}
