#if defined(__GNUC__) && __GNUC__>=5
#define _must_assume(condition) if (!(condition)) __builtin_unreachable()
#elif defined(_MSC_VER)
#define _must_assume(condition) __assume(condition)
#elif defined(__builtin_assume)
#define _must_assume(condition) __builtin_assume(condition)
#else
#error "assume unimpl"
#endif
