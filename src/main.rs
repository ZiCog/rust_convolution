use convolution::*;
use instant::*;

const SAMPLELEN: usize = 20_000_000 + 4; // avx test N*6
const COEFFLEN: usize = 500 + 4; // avx test N*8

use crate::pcg::*;
use clap::{App, Arg};

fn args<'life>() -> clap::ArgMatches<'life> {
    // Command line argument parsing
    App::new("rust_convolution")
        .version("0.1.0")
        .author("Michael <michael@conveqs.com>")
        .about("A benchmark assessing various ways to do a large 2D convolution in Rust.")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("threads")
                .help("Maximum number of threads to use.")
                .required(true),
        )
        .get_matches()
}

fn testfn(tfn: fn(&[f32], &[f32]) -> Vec<f32>, sample: &[f32], coeff: &[f32], comment: &str) {
    let now = Instant::now();
    let result = tfn(&sample, &coeff);
    println!("{:<30}Duration {}ms", comment, now.elapsed().as_millis());
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);
}

fn main() {
    // Command line argument parsing
    let args = args();
    let threads = args.value_of("threads").unwrap_or("1");
    let threads = threads.parse::<usize>().unwrap();
    println!("Using a maximum of {} threads", threads);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let mut pcg32 = Pcg32::new();

    let mut sample: Vec<f32> = vec![0.0; SAMPLELEN];
    for s in &mut sample {
        *s = pcg32.frand();
    }

    let mut coeff: Vec<f32> = vec![0.0; COEFFLEN];
    for c in &mut coeff {
        *c = pcg32.frand();
    }

    testfn(zso::convolution, &sample, &coeff, "zso::convolution:");
    testfn(
        zso::convolution_ffi,
        &sample,
        &coeff,
        "zso::convolution_ffi:",
    );
    testfn(
        zso::convolution_ffi_vreg,
        &sample,
        &coeff,
        "zso::convolution_ffi_vreg:",
    );
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx") {
        testfn(
            zso::convolution_ffi_avx,
            &sample,
            &coeff,
            "zso::convolution_ffi_avx:",
        );
    }
    testfn(bjorn3::convolution, &sample, &coeff, "bjorn3::convolution:");
    testfn(
        dodomorandi::convolution,
        &sample,
        &coeff,
        "dodomorandi::convolution:",
    );
    testfn(pcpthm::convolution, &sample, &coeff, "pcpthm::convolution:");

    testfn(zicog::convolution, &sample, &coeff, "zicog::convolution:");
    testfn(
        zicog::convolution_fast,
        &sample,
        &coeff,
        "zicog::convolution_fast:",
    );
    testfn(
        zicog::convolution_safe,
        &sample,
        &coeff,
        "zicog::convolution_safe:",
    );
    testfn(
        zicog::convolution_slow,
        &sample,
        &coeff,
        "zicog::convolution_slow:",
    );

    testfn(
        alice::convolution_serial,
        &sample,
        &coeff,
        "alice::convolution_serial:",
    );
    testfn(
        alice::convolution_parallel,
        &sample,
        &coeff,
        "alice::convolution_parallel:",
    );
}
