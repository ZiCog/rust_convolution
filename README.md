# rust_convolution

A simple 2d convolution in Rust. As fast as possible! 

This is a collection of suggested Rust functions that were propesed as solutions to matching the performace of C.
The suggestions come from repondandts to a question on the Rust user forum: "Rust performance help (convolution)"
: https://users.rust-lang.org/t/rust-performance-help-convolution/44075

The problem is a simple 500 element convolution kernel run over a 20 million element data set. See convolution.c in the top level directory.

The Rust solutions are each held in modules named after their contributor on the above forum thread.

# Build and Run
```
    $ git clone https://github.com/ZiCog/rust_convolution/edit/master/README.md
    $ cd rust_convolution
    $ cargo run --release
```
