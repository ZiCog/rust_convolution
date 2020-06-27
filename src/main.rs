use convolution::*;
use instant::*;

const SAMPLELEN: usize = 20_000_000;
const COEFFLEN: usize = 500;

mod pcg;
use pcg::*;

fn main() {
    let mut pcg32 = Pcg32::new();

    let mut sample: Vec<f32> = vec![0.0; SAMPLELEN];
    for i in 0..SAMPLELEN {
        sample[i] = pcg32.frand();
    }

    let mut coeff: Vec<f32> = vec![0.0; COEFFLEN];
    for i in 0..COEFFLEN {
        coeff[i] = pcg32.frand();
    }

    let now = Instant::now();
    let result = zso::convolution(&sample, &coeff);
    println!(
        "zso::convolution:            Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = zso::convolution_ffi(&sample, &coeff);
    println!(
        "zso::convolution_ffi:        Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = bjorn3::convolution(&sample, &coeff);
    println!(
        "bjorn3::convolution:         Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = dodomorandi::convolution(&sample, &coeff);
    println!(
        "dodomorandi::convolution:    Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = pcpthm::convolution(&sample, &coeff);
    println!(
        "pcpthm::convolution:         Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = zicog::convolution(&sample, &coeff);
    println!(
        "zicog::convolution:          Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = zicog::convolution_fast(&sample, &coeff);
    println!(
        "zicog::convolution_fast:     Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = zicog::convolution_safe(&sample, &coeff);
    println!(
        "zicog::convolution_safe:     Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = alice::convolution_serial(&sample, &coeff);
    println!(
        "alice::convolution_serial:   Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = alice::convolution_parallel(&sample, &coeff);
    println!(
        "alice::convolution_parallel: Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);

    let now = Instant::now();
    let result = rusty_ron::re_re_conv_f32_compiletime(&sample, &coeff);
    println!(
        "rusty_ron::re_re_conv_f32_compiletime: Duration {}ms",
        now.elapsed().as_millis()
    );
    println!("{}  {}", result[0], result[SAMPLELEN - COEFFLEN]);
}
