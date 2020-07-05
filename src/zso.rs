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

    fn convolution_c_vreg(
        output: *mut f32,
        outlen: *mut i32,
        sample: *const f32,
        samplelen: i32,
        coeff: *const f32,
        coefflen: i32,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn convolution_c_avx(
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

pub fn convolution_ffi_vreg(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut output: Vec<f32> = Vec::with_capacity(sample.len());
    let mut outlen: i32 = 0;
    unsafe {
        convolution_c_vreg(
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

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn convolution_ffi_avx(sample: &[f32], coeff: &[f32]) -> Vec<f32> {
    let mut output: Vec<f32> = Vec::with_capacity(sample.len());
    let mut outlen: i32 = 0;
    unsafe {
        convolution_c_avx(
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
