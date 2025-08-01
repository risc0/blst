#include "risc0.h"

const vec256 BLS12_381_r_R_INV = {
    TO_LIMB_T(0x13f75b69fe75c040), TO_LIMB_T(0xab6fca8f09dc705f),
    TO_LIMB_T(0x7204078a4f77266a), TO_LIMB_T(0x1bbe869330009d57)};
const vec384 BLS12_381_P_R_INV = {
    TO_LIMB_T(0xf4d38259380b4820), TO_LIMB_T(0x7fe11274d898fafb),
    TO_LIMB_T(0x343ea97914956dc8), TO_LIMB_T(0x1797ab1458a88de9),
    TO_LIMB_T(0xed5e64273c4f538b), TO_LIMB_T(0x14fec701e8fb0ce9)};

const limb_t BLS12_381_PP[] = {
    TO_LIMB_T(0x26aa00001c718e39), TO_LIMB_T(0x7ced6b1d76382eab),
    TO_LIMB_T(0x162c338362113cfd), TO_LIMB_T(0x66bf91ed3e71b743),
    TO_LIMB_T(0x292e85a87091a049), TO_LIMB_T(0x1d68619c86185c7b),
    TO_LIMB_T(0xf53149330978ef01), TO_LIMB_T(0x50a62cfd16ddca6e),
    TO_LIMB_T(0x66e59e49349e8bd0), TO_LIMB_T(0xe2dc90e50e7046b4),
    TO_LIMB_T(0x4bd278eaa22f25e9), TO_LIMB_T(0x02a437a4b8c35fc7)};

const vec256 ZERO_256 = {0};
const vec256 THREE_256 = {TO_LIMB_T(3ull), TO_LIMB_T(0ull), TO_LIMB_T(0ull),
                          TO_LIMB_T(0ull)};
const vec384 THREE_384 = {TO_LIMB_T(3ull), TO_LIMB_T(0ull), TO_LIMB_T(0ull),
                          TO_LIMB_T(0ull), TO_LIMB_T(0ull), TO_LIMB_T(0ull)};
const vec384 TWO_INV_384 = {
    TO_LIMB_T(0xdcff7fffffffd556), TO_LIMB_T(0x0f55ffff58a9ffff),
    TO_LIMB_T(0xb39869507b587b12), TO_LIMB_T(0xb23ba5c279c2895f),
    TO_LIMB_T(0x258dd3db21a5d66b), TO_LIMB_T(0xd0088f51cbff34d)};
