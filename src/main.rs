use convolution::*;
use instant::*;

const SAMPLELEN: usize = 20_000_000;
const COEFFLEN: usize = 500;

use crate::pcg::*;

fn testfn(tfn: fn(&[f32], &[f32]) -> Vec<f32>, sample: &[f32], coeff: &[f32], comment: &str) {
    let now = Instant::now();
    let result = tfn(&sample, &coeff);
    println!(
        "{}:            Duration {}ms",
        comment,
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);
}

fn main() {
    let mut pcg32 = Pcg32::new();

    let mut sample: Vec<f32> = vec![0.0; SAMPLELEN];
    for s in &mut sample {
        *s = pcg32.frand();
    }

    let mut coeff: Vec<f32> = vec![0.0; COEFFLEN];
    for c in &mut coeff {
        *c = pcg32.frand();
    }

    testfn(zso::convolution,         &sample, &coeff, "zso::convolution");
    testfn(zso::convolution_ffi,     &sample, &coeff, "zso::convolution_ffi");

    testfn(bjorn3::convolution,      &sample, &coeff, "bjorn3::convolution");
    testfn(dodomorandi::convolution, &sample, &coeff, "dodomorandi::convolution");
    testfn(pcpthm::convolution,      &sample, &coeff, "pcpthm::convolution");

    testfn(zicog::convolution,       &sample, &coeff, "zicog::convolution");
    testfn(zicog::convolution_fast,  &sample, &coeff, "zicog::convolution_fast");
    testfn(zicog::convolution_safe,  &sample, &coeff, "zicog::convolution_safe");

    testfn(alice::convolution_serial,   &sample, &coeff, "alice::convolution_serial");
    testfn(alice::convolution_parallel, &sample, &coeff, "alice::convolution_parallel");
}
