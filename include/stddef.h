#ifndef _STDDEF_H
#define _STDDEF_H

#include "sys/__shared_header_rulibc.h"

#if defined(_MSC_VER)

#include "stddef_rulibc_msvc.h"

#else

#define NULL 0

#ifndef __PTRDIFF_TYPE__
#define __PTRDIFF_TYPE__ long int
#endif
typedef __PTRDIFF_TYPE__ ptrdiff_t;

#ifndef __cplusplus
typedef int wchar_t;
#endif /* #ifndef __cplusplus */
typedef int wint_t;


typedef long unsigned int size_t;

typedef struct { long long __ll; long double __ld; } max_align_t;

#define offsetof(type, member) __builtin_offsetof(type, member)

#endif

#endif /* _STDDEF_H */
