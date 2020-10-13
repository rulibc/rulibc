Tornado 2.2
```
struct iovec {
	caddr_t	iov_base;
	int	iov_len;
};
```
VxWorks 653 2.4
```
struct iovec {
	caddr_t	iov_base;
	int	iov_len;
};
```
VxWorks 6.9
```
/*
 * XXX
 * iov_base should be a void *.
 */
struct iovec {
	char	*iov_base;	/* Base address. */
	size_t	 iov_len;	/* Length. */
};
```

VxWorks 6.9
```
#if defined(_WRS_KERNEL)
#define WCHAR_MIN	0
#define WCHAR_MAX	USHRT_MAX
#define WINT_MIN	INT_MIN
#define WINT_MAX	INT_MAX
#endif /* _WRS_KERNEL */
```
VxWorks 6.9 wchar.h
```
#define WCHAR_MIN	_WCMIN
#define WCHAR_MAX	_WCMAX
#define WEOF	((wint_t)(-1))
````