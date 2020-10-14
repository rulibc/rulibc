#include <stddef.h>
#include <wchar.h>
#include <wctype.h>
#include <stdio.h>
#include <stdint.h>


#define PRINT_TYPE_INFO(typename) do { \
    typename val = (typename)(-1); \
    int is_unsigned = 0; \
    if (val > 0) {is_unsigned = 1;} \
    printf("sizeof " #typename ":%d is_unsigned:%d\n", (int)sizeof(typename), is_unsigned); \
} while(0)

int main() {
    PRINT_TYPE_INFO(char);
    PRINT_TYPE_INFO(int);
    PRINT_TYPE_INFO(unsigned int);

    PRINT_TYPE_INFO(long);
    PRINT_TYPE_INFO(unsigned long);

    PRINT_TYPE_INFO(int8_t);
    PRINT_TYPE_INFO(int16_t);
    PRINT_TYPE_INFO(int32_t);
    PRINT_TYPE_INFO(int64_t);

    PRINT_TYPE_INFO(uint8_t);
    PRINT_TYPE_INFO(uint16_t);
    PRINT_TYPE_INFO(uint32_t);
    PRINT_TYPE_INFO(uint64_t);

    PRINT_TYPE_INFO(signed char);
    PRINT_TYPE_INFO(short);
    PRINT_TYPE_INFO(long long);

    PRINT_TYPE_INFO(unsigned char);
    PRINT_TYPE_INFO(unsigned short);
    PRINT_TYPE_INFO(unsigned long long);

    PRINT_TYPE_INFO(intmax_t);
    PRINT_TYPE_INFO(uintmax_t);

    PRINT_TYPE_INFO(size_t);
    PRINT_TYPE_INFO(ptrdiff_t);
    PRINT_TYPE_INFO(intptr_t);
    PRINT_TYPE_INFO(uintptr_t);
#if !defined(_MSC_VER) && !defined(__RULIBC__)
    PRINT_TYPE_INFO(ssize_t);
#endif

    PRINT_TYPE_INFO(wchar_t);
    PRINT_TYPE_INFO(wint_t);
    PRINT_TYPE_INFO(wctype_t);

    printf("WEOF:%lld\n", (long long)WEOF);
    printf("long double size:%d\n", (int)sizeof(long double));

    return 0;
}