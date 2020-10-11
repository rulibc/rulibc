
typedef char* va_list;

void __cdecl __va_start(va_list* , ...);
#define va_start(ap, x) ((void)(__va_start(&ap, x)))
#define va_arg(ap, t)                                                    \
    ((sizeof(t) > sizeof(__int64) || (sizeof(t) & (sizeof(t) - 1)) != 0) \
        ? **(t**)((ap += sizeof(__int64)) - sizeof(__int64))             \
        :  *(t* )((ap += sizeof(__int64)) - sizeof(__int64)))
#define va_end(ap)        ((void)(ap = (va_list)0))

#define va_copy(destination, source) ((destination) = (source))

