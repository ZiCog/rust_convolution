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
