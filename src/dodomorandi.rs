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
