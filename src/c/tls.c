#if defined(_WIN32)

#include <stddef.h>
#include <Windows.h>


//-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Section Attributes
//
//-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#pragma section(".CRT$XCA",    long, read) // First C++ Initializer
#pragma section(".CRT$XCAA",   long, read) // Startup C++ Initializer
#pragma section(".CRT$XCZ",    long, read) // Last C++ Initializer

#pragma section(".CRT$XDA",    long, read) // First Dynamic TLS Initializer
#pragma section(".CRT$XDZ",    long, read) // Last Dynamic TLS Initializer

#pragma section(".CRT$XIA",    long, read) // First C Initializer
#pragma section(".CRT$XIAA",   long, read) // Startup C Initializer
#pragma section(".CRT$XIAB",   long, read) // PGO C Initializer
#pragma section(".CRT$XIAC",   long, read) // Post-PGO C Initializer
#pragma section(".CRT$XIC",    long, read) // CRT C Initializers
#pragma section(".CRT$XIYA",   long, read) // VCCorLib Threading Model Initializer
#pragma section(".CRT$XIYAA",  long, read) // XAML Designer Threading Model Override Initializer
#pragma section(".CRT$XIYB",   long, read) // VCCorLib Main Initializer
#pragma section(".CRT$XIZ",    long, read) // Last C Initializer

#pragma section(".CRT$XLA",    long, read) // First Loader TLS Callback
#pragma section(".CRT$XLC",    long, read) // CRT TLS Constructor
#pragma section(".CRT$XLD",    long, read) // CRT TLS Terminator
#pragma section(".CRT$XLZ",    long, read) // Last Loader TLS Callback

#pragma section(".CRT$XPA",    long, read) // First Pre-Terminator
#pragma section(".CRT$XPB",    long, read) // CRT ConcRT Pre-Terminator
#pragma section(".CRT$XPX",    long, read) // CRT Pre-Terminators
#pragma section(".CRT$XPXA",   long, read) // CRT stdio Pre-Terminator
#pragma section(".CRT$XPZ",    long, read) // Last Pre-Terminator

#pragma section(".CRT$XTA",    long, read) // First Terminator
#pragma section(".CRT$XTZ",    long, read) // Last Terminator

#pragma section(".CRTMA$XCA",  long, read) // First Managed C++ Initializer
#pragma section(".CRTMA$XCZ",  long, read) // Last Managed C++ Initializer

#pragma section(".CRTVT$XCA",  long, read) // First Managed VTable Initializer
#pragma section(".CRTVT$XCZ",  long, read) // Last Managed VTable Initializer

#pragma section(".rdata$T",    long, read)

#pragma section(".rtc$IAA",    long, read) // First RTC Initializer
#pragma section(".rtc$IZZ",    long, read) // Last RTC Initializer

#pragma section(".rtc$TAA",    long, read) // First RTC Terminator
#pragma section(".rtc$TZZ",    long, read) // Last RTC Terminator

#define _CRTALLOC(x) __declspec(allocate(x))



/* Thread Local Storage index for this .EXE or .DLL */

unsigned long _tls_index = 0;

/* Special symbols to mark start and end of Thread Local Storage area. */

#pragma data_seg(".tls")

#if defined (_M_X64)
_CRTALLOC(".tls")
#endif  /* defined (_M_X64) */
char _tls_start = 0;

#pragma data_seg(".tls$ZZZ")

#if defined (_M_X64)
_CRTALLOC(".tls$ZZZ")
#endif  /* defined (_M_X64) */
char _tls_end = 0;

#pragma data_seg()

uintptr_t __security_cookie = 0xAABBCCDD;

void __fastcall __security_check_cookie(uintptr_t cookie)
{
    if (cookie != __security_cookie)
        __debugbreak();
}

#endif