    .section .rodata

    .global modadd256_blob
    .type   modadd256_blob, @object
    .balign 4
modadd256_blob:
    .incbin "modadd_256.blob"

    .global modadd384_blob
    .type   modadd384_blob, @object
    .balign 4
modadd384_blob:
    .incbin "modadd_384.blob"

    .global modsub256_blob
    .type   modsub256_blob, @object
    .balign 4
modsub256_blob:
    .incbin "modsub_256.blob"

    .global modsub384_blob
    .type   modsub384_blob, @object
    .balign 4
modsub384_blob:
    .incbin "modsub_384.blob"

    .global modmul256_blob
    .type   modmul256_blob, @object
    .balign 4
modmul256_blob:
    .incbin "modmul_256.blob"

    .global modmul384_blob
    .type   modmul384_blob, @object
    .balign 4
modmul384_blob:
    .incbin "modmul_384.blob"

    .global modinv256_blob
    .type   modinv256_blob, @object
    .balign 4
modinv256_blob:
    .incbin "modinv_256.blob"

    .global modinv384_blob
    .type   modinv384_blob, @object
    .balign 4
modinv384_blob:
    .incbin "modinv_384.blob"

    .global xxone_mul384_blob
    .type   xxone_mul384_blob, @object
    .balign 4
xxone_mul384_blob:
    .incbin "extfield_xxone_mul_384.blob"
