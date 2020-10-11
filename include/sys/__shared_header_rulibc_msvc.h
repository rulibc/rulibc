
#if defined(_MSC_VER) && _MSC_VER >= 1500
/*
 *
 * For vs 2008 sp1, vs 2010 sp1, vs 2013 and latter version of vs support
 * for execution_character_set support("utf-8")
 *
 * Fo vs 2005 and vs2012 are not supported as they doesn't support for
 * execution_character_set nor raw string literal.
 *
 * For vs 2003 and ealier version of visual studio, the execution_character_set
 * are raw as source code encoding, so only need save the source with utf-8
 * encoding.
*/
#pragma execution_character_set("utf-8")

#endif
