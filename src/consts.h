/*
 * Copyright Supranational LLC
 * Licensed under the Apache License, Version 2.0, see LICENSE for details.
 * SPDX-License-Identifier: Apache-2.0
 */
#ifndef __BLS12_381_ASM_CONST_H__
#define __BLS12_381_ASM_CONST_H__
#include "vect.h"

#ifndef BLST_ALIGNED
#if defined(__GNUC__) || defined(__clang__)
#define BLST_ALIGNED(x) __attribute__((aligned(x)))
#elif defined(_MSC_VER)
#define BLST_ALIGNED(x) __declspec(align(x))
#else
#define BLST_ALIGNED(x)
#endif
#endif

extern const vec384 BLS12_381_P BLST_ALIGNED(8);
extern const limb_t BLS12_381_p0;
static const limb_t p0 = (limb_t)0x89f3fffcfffcfffd;  /* -1/P */
typedef union { vec384 p12[12]; vec384x p2; vec384 p; } radix384;
extern const radix384 BLS12_381_Rx BLST_ALIGNED(8); /* (1<<384)%P, "radix", one-in-Montgomery */
extern const vec384 BLS12_381_RR BLST_ALIGNED(8);   /* (1<<768)%P, "radix"^2, to-Montgomery   */

#define ONE_MONT_P TO_LIMB_T(0x760900000002fffd), \
                   TO_LIMB_T(0xebf4000bc40c0002), \
                   TO_LIMB_T(0x5f48985753c758ba), \
                   TO_LIMB_T(0x77ce585370525745), \
                   TO_LIMB_T(0x5c071a97a256ec6d), \
                   TO_LIMB_T(0x15f65ec3fa80e493)

#define ZERO_384 (BLS12_381_Rx.p2[1])

extern const vec256 BLS12_381_r BLST_ALIGNED(8);    /* order */
static const limb_t r0 = (limb_t)0xfffffffeffffffff;  /* -1/r */
extern const vec256 BLS12_381_rRR BLST_ALIGNED(8);  /* (1<<512)%r, "radix"^2, to-Montgomery   */

#endif
