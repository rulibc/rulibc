#include <string.h>
#include <stdio.h>

#include "test_helpers.h"

int main(void) {
    printf("%s\n", strstr("In rulibc we trust", "rust"));
    printf("%s\n", strstr("In rulibc we trust", "libc"));
    printf("%s\n", strstr("In rulibc we trust", "bugs"));
    printf("%s\n", strstr("IN RULIBC WE TRUST", "rust"));
    printf("%s\n", strcasestr("IN RULIBC WE TRUST", "rust"));
}
