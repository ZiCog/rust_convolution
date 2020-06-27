fn main() {
    cc::Build::new()
        .file("src/conv.c")
        .flag("-Ofast")
        .flag("-march=native")
        .flag("-funroll-all-loops")
        .compile("libconv.a");
}
