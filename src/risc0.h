#ifndef __BLS12_381_ASM_RISC0_H__
#define __BLS12_381_ASM_RISC0_H__

#include "vect.h"

#if defined(_MSC_VER)
#define ALWAYS_INLINE __forceinline
#elif defined(__GNUC__) || defined(__clang__)
#define ALWAYS_INLINE __attribute__((always_inline)) inline
// Fallback for other compilers (just use standard inline)
#else
#define ALWAYS_INLINE inline
#endif

extern void risc0_bigint_modmul_256_unchecked(const limb_t*, const limb_t*,
                                              const limb_t*, limb_t*);
extern void risc0_bigint_modmul_384_unchecked(const limb_t*, const limb_t*,
                                              const limb_t*, limb_t*);
extern void risc0_bigint_modinv_256_unchecked(const limb_t*, const limb_t*,
                                              limb_t*);
extern void risc0_bigint_modinv_384_unchecked(const limb_t*, const limb_t*,
                                              limb_t*);
extern void risc0_bigint_extfield_xxone_mul_384_unchecked(
    const limb_t*, const limb_t*, const limb_t*, const limb_t*, limb_t*);
#define risc0_modmul_256_unchecked risc0_bigint_modmul_256_unchecked
#define risc0_modmul_384_unchecked risc0_bigint_modmul_384_unchecked
#define risc0_modinv_256_unchecked risc0_bigint_modinv_256_unchecked
#define risc0_modinv_384_unchecked risc0_bigint_modinv_384_unchecked
#define risc0_xxone_mul_384_unchecked                                          \
    risc0_bigint_extfield_xxone_mul_384_unchecked

extern void risc0_bigint_modadd_256(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modadd_384(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modsub_256(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modsub_384(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modmul_256(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modmul_384(const limb_t*, const limb_t*, const limb_t*,
                                    limb_t*);
extern void risc0_bigint_modinv_256(const limb_t*, const limb_t*, limb_t*);
extern void risc0_bigint_modinv_384(const limb_t*, const limb_t*, limb_t*);
extern void risc0_bigint_extfield_xxone_mul_384(const limb_t*, const limb_t*,
                                                const limb_t*, const limb_t*,
                                                limb_t*);
#define risc0_modadd_256 risc0_bigint_modadd_256
#define risc0_modadd_384 risc0_bigint_modadd_384
#define risc0_modsub_256 risc0_bigint_modsub_256
#define risc0_modsub_384 risc0_bigint_modsub_384
#define risc0_modmul_256 risc0_bigint_modmul_256
#define risc0_modmul_384 risc0_bigint_modmul_384
#define risc0_modinv_256 risc0_bigint_modinv_256
#define risc0_modinv_384 risc0_bigint_modinv_384
#define risc0_xxone_mul_384 risc0_bigint_extfield_xxone_mul_384

extern void sys_sha_buffer(unsigned int* out_state,
                           const unsigned int* in_state,
                           const unsigned char* buf, unsigned int count);
#define risc0_sha256_buffer sys_sha_buffer

extern void sys_panic(const void* msg_ptr, unsigned int len);

// This macro ensures a condition is true, otherwise it panics.
// It's active in all build modes, including release.
#define ENSURE(condition)                                                      \
    do {                                                                       \
        if (!(condition)) {                                                    \
            sys_panic((const unsigned char*)"unreachable", 11);                \
        }                                                                      \
    } while (0)

// Montgomery constant R^-1 mod r (for the 256-bit scalar field r)
extern const vec256 BLS12_381_r_R_INV;
// Montgomery constant R^-1 mod P (for the 384-bit prime field P)
extern const vec384 BLS12_381_P_R_INV;

// Square of the prime field modulus P (P^2), as a 768-bit integer
extern const limb_t BLS12_381_PP[NLIMBS(768)];

extern const vec256 ZERO_256;
extern const vec256 THREE_256;
extern const vec384 THREE_384;
extern const vec384 TWO_INV_384;

#endif // __BLS12_381_ASM_RISC0_H__
