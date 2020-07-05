# rust_convolution

A simple 2d convolution in Rust. As fast as possible! 

This is a collection of suggested Rust functions that were propesed as solutions to matching the performace of C.
The suggestions come from repondandts to a question on the Rust user forum: "Rust performance help (convolution)"
: https://users.rust-lang.org/t/rust-performance-help-convolution/44075

The problem is a simple 500 element convolution kernel run over a 20 million element data set. See convolution.c in the top level directory.

The Rust solutions are each held in modules named after their contributor on the above forum thread.

## Build and Run

Build an run as a program that times the various solutions for the 20 miilion element convolution and prints the results:
```
    $ git clone https://github.com/ZiCog/rust_convolution/edit/master/README.md
    $ cd rust_convolution
    $ cargo run --release
```

Or run the benchmarks which use a much smaller kerne and data set:
```
    $ cargo bench
```

## Results

The full convlution produces times like these on my Intel PC:

```
zso::convolution:            Duration 24365ms
119.04489  127.622635
zso::convolution_ffi:        Duration 1396ms
119.04485  127.62263
bjorn3::convolution:         Duration 7074ms
119.04483  127.62263
dodomorandi::convolution:    Duration 8175ms
119.04489  127.622635
pcpthm::convolution:         Duration 1328ms
119.04485  127.62263
zicog::convolution:          Duration 1310ms
119.04485  127.62263
zicog::convolution_fast:     Duration 1358ms
119.04485  127.62263
zicog::convolution_safe:     Duration 8070ms
119.04489  127.622635
alice::convolution_serial:   Duration 1341ms
119.04485  127.62263
alice::convolution_parallel: Duration 348ms
119.04485  127.62263
```

and the benchmark output:

```
test tests::bench_alice_convolution_parallel ... bench:      22,066 ns/iter (+/- 1,441)
test tests::bench_alice_convolution_serial   ... bench:       5,500 ns/iter (+/- 176)
test tests::bench_bjorn3_convolution         ... bench:      12,900 ns/iter (+/- 434)
test tests::bench_dodomorandi_convolution    ... bench:       8,695 ns/iter (+/- 296)
test tests::bench_pcpthm_convolution         ... bench:       5,470 ns/iter (+/- 283)
test tests::bench_zicog_convolution          ... bench:       4,445 ns/iter (+/- 88)
test tests::bench_zicog_convolution_fast     ... bench:       4,723 ns/iter (+/- 109)
test tests::bench_zicog_convolution_safe     ... bench:       8,302 ns/iter (+/- 287)
test tests::bench_zso_convolution            ... bench:      25,268 ns/iter (+/- 852)
test tests::bench_zso_convolution_ffi        ... bench:       4,667 ns/iter (+/- 149)
```








