IEEE 128-bit floating point support for PowerPC

https://gcc.gnu.org/wiki/Ieee128PowerPC
1.4 Original PowerPC GCC long double support
Internally within the GCC compiler, the TFmode type is used to represent the default 128-bit floating point type.

On older Linux systems, the default for long double was to use the double type.

In the past, the default for Linux and AIX systems was changed, and the new default for long double is the IBM extended double type. There are switches (-mlong-double-64 and -mlong-double-128) to control the default for long double.

On the RTEMS system embedded system, the default for long double is double. If you use the -mlong-double-128 option, the default for long double is IEEE 754R 128-bit floating point. It is believed that this is the only PowerPC variant that is supported by GCC that used IEEE 754R. It is not clear if any RTEMS user has ever used the IEEE 754R 128-bit support.

Internally within GCC, the TFmode type is used to represent the default 128-bit floating point support. This means there are a parallel set of TFmode operations for doing IBM extended double and for doing IEEE 754R operations.

2.1 Options and defaults
The option -mfloat128 was added to enable IEEE 754R support It is currently not default, but when the final patches are committed, it will be default if the VSX instruction set is enabled.

The compiler already supports two undocumented options (-mabi=ieeelongdouble and -mabi=ibmlongdouble) to control whether long double has the IBM extended double type or uses the IEEE 754R 128-bit format.

## PPC32/64 long double design
  So for ppc 32 target, we can force user to use -mlong-double-64 to force long double to be 64bit ieee double
  And for systems supporting the Vector/Scalar (VSX) instruction set that was introduced with the Power7 server architecture.
  We can to force use  -mfloat128 -mabi=ieeelongdouble to force long  double to be 128bit ieee double

## X86 long double design