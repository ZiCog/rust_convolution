// convolution.c
//
// An experiment in comparing Rust performance to C for a simple convolution.
//
// From this rust-lang discussion: https://users.rust-lang.org/t/rust-performance-help-convolution/44075
//
// Compile with:
//     $ clang -Ofast -march=native -funroll-all-loops convolution.c -o convolution 
//
#include <stdio.h>
#include <malloc.h>
#include <stdint.h>

#define SAMPLELEN (20 * 1000 * 1000)
#define COEFFLEN  500

void convolution(float *out, int *out_length, const float *sample, int samplelen, const float *coeff, int coefflen) {
    int outlen = samplelen - coefflen + 1;
    for (int i = 0; i < outlen; i++) {
        float acc = 0.;
        for (int j = 0; j < coefflen; j++) {
            acc += sample[i + j] * coeff[j];
        }
        out[i] = acc;
    }
    *out_length = outlen;
}

// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)

typedef struct { uint64_t state;  uint64_t inc; } pcg32_t;

pcg32_t* pcg32_new() {
    pcg32_t* self;
    self = malloc(sizeof(pcg32_t));
    self->state=0xdeadbeef01234567; 
    self->inc = 1;
    return self;
}

uint32_t pcg32_rand(pcg32_t* self) {
    uint64_t oldstate = self->state;
    // Advance internal state
    self->state = oldstate * 6364136223846793005ULL + (self->inc | 1);
    // Calculate output function (XSH RR), uses old state for max ILP
    uint32_t xorshifted = ((oldstate >> 18u) ^ oldstate) >> 27u;
    uint32_t rot = oldstate >> 59u;
    return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
}

float pcg32_frand (pcg32_t* self) {
    uint32_t rnd;
    rnd = pcg32_rand(self);
    float r = ((float)rnd / (float)0x100000000);
    return r;
}

int main() {
    pcg32_t* pcg32 = pcg32_new();

    float *sample = malloc(SAMPLELEN * sizeof(float));
    for (int i = 0; i < SAMPLELEN; i++ ) {
        sample[i] = pcg32_frand(pcg32);
    }
    
    float *coeff = malloc(COEFFLEN * sizeof(float));
    for (int i = 0; i < COEFFLEN; i++ ) {
        coeff[i] = pcg32_frand(pcg32);
    }

    int result_len;
    float *result = malloc(SAMPLELEN * sizeof(float));

    convolution(result, &result_len, sample, SAMPLELEN, coeff, COEFFLEN);

    printf("%f\n", result[0]);
}

