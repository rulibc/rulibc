#ifndef _BITS_WCHAR_H
#define _BITS_WCHAR_H
#include <stdint.h>

#if defined(_MSC_VER)

#include "wchar_rulibc_msvc.h"

#else

#define WEOF (0xffffffffu)
#define WCHAR_MIN (0)
#define WCHAR_MAX (0x7fffffff)

#define __need_size_t
#define __need_wchar_t
#define __need_wint_t
#define __need_NULL

#endif

#endif /* _BITS_WCHAR_H */
