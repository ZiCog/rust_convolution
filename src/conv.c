void convolution_c(float *out, int *out_length, const float *sample, int samplelen, const float *coeff, int coefflen) {
    int outlen = samplelen - coefflen + 1;
    for (int i=0; i<outlen; i++) {
        float acc = 0.;
        for (int j=0; j<coefflen; j++) {
            acc += sample[i + j] * coeff[j];
        }
        out[i] = acc;
    }
    *out_length = outlen;
}
