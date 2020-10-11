

/* Define NULL pointer value and the offset() macro */

#ifndef NULL
#ifdef  __cplusplus
#define NULL    0
#else
#define NULL    ((void *)0)
#endif
#endif

/* TODO: support for 32 bit */

#ifndef _PTRDIFF_T_DEFINED
#define __PTRDIFF_TYPE__ long long int
typedef __PTRDIFF_TYPE__ ptrdiff_t;
#define _PTRDIFF_T_DEFINED
#endif

#ifndef _SIZE_T_DEFINED
typedef unsigned long long int size_t;
#define _SIZE_T_DEFINED
#endif


#ifndef _WCHAR_T_DEFINED
typedef unsigned short wchar_t;
#define _WCHAR_T_DEFINED
#endif

typedef struct { long long __ll; long double __ld; } max_align_t;

#define offsetof(s,m)   (size_t)&(((s *)0)->m)
