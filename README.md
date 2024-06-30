# Crypto

This repo contains toy implementations of the following algorithms:

- [AES (Rijndael)](./src/aes.rs)
- [Diffie-Hellman Key Exchange](./src/diffie_hellman.rs)
- [SHA1](./src/sha1.rs)
- [MD5](./src/md5.rs)
- [HMAC](./src/hmac.rs)
- [Reed-Solomon](./src/reed_solomon.rs)
- [Shamir Secret Sharing](./src/shamir.rs)
- [CRC32](./src/crc.rs)
- [Linear Feedback Shift Registers](./src/lfsr.rs)

- The AES implementation comes from
<https://github.com/5n00py/soft-aes/blob/main/src/aes/aes_core.rs>.

- The Diffie-Hellman Key exchange comes from: <https://exercism.io>.

- The implementations of Reed-Solomon, Shamir secret sharing, CRC, and
LFSRs come from <https://github.com/geky/gf256>.

Some future ideas?

- SHA2, SHA3?
- RSA
- Chacha20, Salsa
