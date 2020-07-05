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

#if defined(__arm__) || defined(__aarch64__)
  #include <arm_neon.h>
  typedef float32x4_t VECTYPE;
  #define LOADSIMD(x) *((VECTYPE *)x)
#else
  #include <smmintrin.h>
  typedef __v4sf VECTYPE;
  #define LOADSIMD(x) _mm_loadu_ps(x)
#endif

#include <stdio.h>

void convolution_c_vreg(float *out, int *out_length, const float *sample, int samplelen, const float *coeff, int coefflen) {
    if (samplelen % 6 != 0) fprintf(stderr, "This algorythm good only if samplelen = N*6, remain: %d\n", samplelen%6);
    if (coefflen % 4 != 0)  fprintf(stderr, "This algorythm good only if coefflen = N*4, remain: %d\n", coefflen%4);
    int outlen = samplelen - coefflen + 1;
    for (int i=0; i<outlen; i+=6) {
        VECTYPE acc0 = {0., 0., 0., 0.};
        VECTYPE acc1 = {0., 0., 0., 0.};
        VECTYPE acc2 = {0., 0., 0., 0.};
        VECTYPE acc3 = {0., 0., 0., 0.};
        VECTYPE acc4 = {0., 0., 0., 0.};
        VECTYPE acc5 = {0., 0., 0., 0.};
        for (int j=0; j<=coefflen-4; j+=4) {
            VECTYPE svec;
            VECTYPE cvec = LOADSIMD(&coeff[j]);
            int r = i + j;
            svec = LOADSIMD(&sample[r    ]); acc0 += svec * cvec;
            svec = LOADSIMD(&sample[r + 1]); acc1 += svec * cvec;
            svec = LOADSIMD(&sample[r + 2]); acc2 += svec * cvec;
            svec = LOADSIMD(&sample[r + 3]); acc3 += svec * cvec;
            svec = LOADSIMD(&sample[r + 4]); acc4 += svec * cvec;
            svec = LOADSIMD(&sample[r + 5]); acc5 += svec * cvec;
        }
        out[i  ] = acc0[0] + acc0[1] + acc0[2] + acc0[3];
        out[i+1] = acc1[0] + acc1[1] + acc1[2] + acc1[3];
        out[i+2] = acc2[0] + acc2[1] + acc2[2] + acc2[3];
        out[i+3] = acc3[0] + acc3[1] + acc3[2] + acc3[3];
        out[i+4] = acc4[0] + acc4[1] + acc4[2] + acc4[3];
        out[i+5] = acc5[0] + acc5[1] + acc5[2] + acc5[3];
    }
    *out_length = outlen;
}

#if defined (__AVX__)
#include <immintrin.h>
  typedef __v8sf AVXTYPE;
  #define LOADAVX(x) _mm256_loadu_ps(x)

  void convolution_c_avx(float *out, int *out_length, const float *sample, int samplelen, const float *coeff, int coefflen) {
    if (samplelen % 6 != 0) fprintf(stderr, "This algorythm good only if samplelen = N*6, remain: %d\n", samplelen%6);
    if (coefflen % 8 != 0)  fprintf(stderr, "This algorythm good only if coefflen = N*8, remain: %d\n", coefflen%8);
    int outlen = samplelen - coefflen + 1;
    for (int i=0; i<outlen; i+=6) {
        AVXTYPE acc0 = {0., 0., 0., 0., 0., 0., 0., 0.};
        AVXTYPE acc1 = {0., 0., 0., 0., 0., 0., 0., 0.};
        AVXTYPE acc2 = {0., 0., 0., 0., 0., 0., 0., 0.};
        AVXTYPE acc3 = {0., 0., 0., 0., 0., 0., 0., 0.};
        AVXTYPE acc4 = {0., 0., 0., 0., 0., 0., 0., 0.};
        AVXTYPE acc5 = {0., 0., 0., 0., 0., 0., 0., 0.};
        for (int j=0; j<=coefflen-8; j+=8) {
            AVXTYPE svec;
            AVXTYPE cvec = LOADAVX(&coeff[j]);
            int r = i + j;
            svec = LOADAVX(&sample[r    ]); acc0 += svec * cvec;
            svec = LOADAVX(&sample[r + 1]); acc1 += svec * cvec;
            svec = LOADAVX(&sample[r + 2]); acc2 += svec * cvec;
            svec = LOADAVX(&sample[r + 3]); acc3 += svec * cvec;
            svec = LOADAVX(&sample[r + 4]); acc4 += svec * cvec;
            svec = LOADAVX(&sample[r + 5]); acc5 += svec * cvec;
        }
        out[i  ] = acc0[0] + acc0[1] + acc0[2] + acc0[3] + acc0[4] + acc0[5] + acc0[6] + acc0[7];
        out[i+1] = acc1[0] + acc1[1] + acc1[2] + acc1[3] + acc1[4] + acc1[5] + acc1[6] + acc1[7];
        out[i+2] = acc2[0] + acc2[1] + acc2[2] + acc2[3] + acc2[4] + acc2[5] + acc2[6] + acc2[7];
        out[i+3] = acc3[0] + acc3[1] + acc3[2] + acc3[3] + acc3[4] + acc3[5] + acc3[6] + acc3[7];
        out[i+4] = acc4[0] + acc4[1] + acc4[2] + acc4[3] + acc4[4] + acc4[5] + acc4[6] + acc4[7];
        out[i+5] = acc5[0] + acc5[1] + acc5[2] + acc5[3] + acc5[4] + acc5[5] + acc5[6] + acc5[7];
    }
    *out_length = outlen;
}
#endif
