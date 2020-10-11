#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <setjmp.h>

int main(int argc, char* argv) {
    jmp_buf buf;
    float f = 1000.0f;
    double d = 1000000000.0;
    long double ld = 100000.0f;

    int32_t i32f = (int32_t)f;
    int32_t i32d = (int32_t)d;
    uint32_t u32f = (uint32_t)f;
    uint32_t u32d = (uint32_t)d;

    int64_t i64f = (int64_t)f;
    int64_t i64d = (int64_t)d;
    uint64_t u64f = (uint64_t)f;
    uint64_t u64d = (uint64_t)d;

    f = (float)i32f;
    d = (double)i32d;
    f = (float)u32f;
    d = (double)u32d;

    f = (float)i64f;
    d = (double)i64d;
    f = (float)u64f;
    d = (double)u64d;
    ld = (long double)u64d;
    printf("Hello, the world %.10Lf\n", ld);
    setjmp(buf);
}