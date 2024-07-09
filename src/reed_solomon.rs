//! # Reed Solomon codes
//!
//! Reed solomon codes are error correcting codes that create $n$ shards of data and allow $k$
//! shards where $k \leq n$ to recover the underlying data. Reed solomon codes have wide industrial
//! use -- they are used in QR codes, CDs, Hard disks, and even large systems like amazon's S3.
//!
//! Reed solomon codes work by doing arithmetic over finite fields. A reed solomon code, like a
//! (255, 223) code has 223 data bits and 32 parity bits. The 32 parity bits can be used to detect
//! up to 32 errors in the block, and correct up to 16 errors.
//!
//! Reed solomon codes encode the data in the block and the parity bits using lagrange
//! interpolation.
//!
//! Take an example where there are 3 points, (1, 2), (3, 2) and (4, -1) which define a polynomial
//! of degree 2 (since $n$ points uniquely define a polynomial with a degree of $n - 1$.
//! There are 3 lagrange polynomials, one for each point. For the first point, (1, 2), the lagrange
//! polynomial is equal to 1 at the x coordinate (1), and 0 at all the other y coordinates of the
//! remaining points, (3, 4). The second point is (3, 2), so the lagrange polynomial for that point
//! is (3, 1), while the other points are (1, 0) and (4, 0). Finally, the last point is (4, 1),
//! with the other points being (1, 0) and (3, 0).
//!
//! We then interpolate the first polynomial: Since the polynomial has points (1, 1), (3, 0) and
//! (4, 0), it is defined as $l_1(x) = (x - 3)(x - 4)$. After subbing in 1 for $x$, the x value is
//! 6 on the right hand side, and the left hand side has to be 1, so the final polynomial is:
//! $l_1(x) = \frac{1}{6}(x - 3)(x - 4)$.
//! After doing the same for the other points, we multiply each polynomial by the original point's
//! y coordinate, so (2, 2, -1), and then sum the polynomials together to get the unique polynomial.
//! Finally, we can take as many other points on the curve as required and then pack them with the
//! original data. Thus, we take as many bits as required to allow for $k$ to reconstruct the
//! polynomial, so $k$ has to be the number of bits + 1.

// Constants for Reed-Solomon error correction
//
// Reed-Solomon can correct ECC_SIZE known erasures and ECC_SIZE/2 unknown
// erasures. DATA_SIZE is arbitrary, however the total size is limited to
// 255 bytes in a GF(256) field.
//

use gf256::gf256;
use std::fmt;

pub const DATA_SIZE: usize = 223;

pub const ECC_SIZE: usize = 32;

pub const BLOCK_SIZE: usize = DATA_SIZE + ECC_SIZE;

// The generator polynomial in Reed-Solomon is a polynomial with roots (f(x) = 0)
// at fixed points (g^i) in the finite-field.
//
//     ECC_SIZE
// G(x) = ∏ (x - g^i)
//        i
//
// Note that G(g^i) = 0 when i < ECC_SIZE, and that this holds for any
// polynomial * G(x). And we can make a message polynomial a multiple of G(x)
// by appending the remainder, message % G(x), much like CRC.
//
// Thanks to Rust's const evaluation, we can, and do, evaluate this at
// compile time. However, this has a tendency to hit the limit of
// const_eval_limit for large values of ECC_SIZE.
//
// The only current workaround for this is nightly + #![feature(const_eval_limit="0")].
//
// See:
// https://github.com/rust-lang/rust/issues/67217
//

pub const GENERATOR_POLY: [gf256; ECC_SIZE + 1] = {
    let mut g = [gf256::new(0); ECC_SIZE + 1];
    g[ECC_SIZE] = gf256::new(1);

    // find G(x)
    //
    //     ECC_SIZE
    // G(x) = ∏  (x - g^i)
    //        i
    //
    let mut i = 0usize;
    while i < ECC_SIZE {
        // x - g^i
        let root = [gf256::new(1), gf256::GENERATOR.naive_pow(i as u8)];

        // G(x)*(x - g^i)
        let mut product = [gf256::new(0); ECC_SIZE + 1];
        let mut j = 0usize;
        while j < i + 1 {
            let mut k = 0usize;
            while k < root.len() {
                product[product.len() - 1 - (j + k)] = product[product.len() - 1 - (j + k)]
                    .naive_add(g[g.len() - 1 - j].naive_mul(root[root.len() - 1 - k]));
                k += 1;
            }
            j += 1;
        }
        g = product;

        i += 1;
    }

    g
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    TooManyErrors,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooManyErrors => write!(f, "Too many errors to correct"),
        }
    }
}

fn poly_eval(f: &[gf256], x: gf256) -> gf256 {
    let mut y = gf256::new(0);
    for c in f {
        y = y * x + c;
    }
    y
}

fn poly_scale(f: &mut [gf256], c: gf256) {
    for i in 0..f.len() {
        f[i] *= c;
    }
}

fn poly_add(f: &mut [gf256], g: &[gf256]) {
    debug_assert!(f.len() >= g.len());

    // note g.len() may be <= f.len()!
    for i in 0..f.len() {
        f[f.len() - 1 - i] += g[g.len() - 1 - i];
    }
}

fn poly_mul(f: &mut [gf256], g: &[gf256]) {
    debug_assert!(f[..g.len() - 1].iter().all(|x| *x == gf256::new(0)));

    // This is in-place, at the cost of being a bit confusing,
    // note that we only write to i+j, and i+j is always >= i
    //
    // What makes this confusing is that f and g are both big-endian
    // polynomials, reverse order from what you would expect. And in
    // order to leverage the i+j non-overlap, we need to write to
    // f in reverse-reverse order.
    //
    for i in (0..f.len() - g.len() + 1).rev() {
        let fi = f[f.len() - 1 - i];
        f[f.len() - 1 - i] = gf256::new(0);

        for j in 0..g.len() {
            f[f.len() - 1 - (i + j)] += fi * g[g.len() - 1 - j];
        }
    }
}

fn poly_divrem(f: &mut [gf256], g: &[gf256]) {
    debug_assert!(f.len() >= g.len());

    // find leading coeff to normalize g, note you could avoid
    // this if g is already normalized
    let leading_coeff = g[0];

    for i in 0..(f.len() - g.len() + 1) {
        if f[i] != gf256::new(0) {
            f[i] /= leading_coeff;

            for j in 1..g.len() {
                f[i + j] -= f[i] * g[j];
            }
        }
    }
}

// Encode using Reed-Solomon error correction
//
// Much like in CRC, we want to make the message a multiple of G(x),
// our generator polynomial. We can do this by appending the remainder
// of our message after division by G(x).
//
// ``` text
// c(x) = m(x) - (m(x) % G(x))
// ```
//
// Note we expect the message to only take up the first message.len()-ECC_SIZE
// bytes, but this can be smaller than BLOCK_SIZE
//

pub fn encode(message: &mut [u8]) {
    assert!(message.len() <= BLOCK_SIZE);
    assert!(message.len() >= ECC_SIZE);
    let data_len = message.len() - ECC_SIZE;

    // create copy for polynomial division
    //
    // note if message is < DATA_SIZE we just treat it as a smaller polynomial,
    // this is equivalent to prepending zeros
    //
    let mut divrem = message.to_vec();
    divrem[data_len..].fill(0);

    // divide by our generator polynomial
    poly_divrem(
        unsafe { gf256::slice_from_slice_mut_unchecked(&mut divrem) },
        &GENERATOR_POLY,
    );

    // return message + remainder, this new message is a polynomial
    // perfectly divisable by our generator polynomial
    message[data_len..].copy_from_slice(&divrem[data_len..]);
}

fn find_syndromes(f: &[gf256]) -> Vec<gf256> {
    let mut S = vec![];
    for i in 0..ECC_SIZE {
        S.push(poly_eval(f, gf256::GENERATOR.pow(u8::try_from(i).unwrap())));
    }
    S
}

fn find_forney_syndromes(codeword: &[gf256], S: &[gf256], erasures: &[usize]) -> Vec<gf256> {
    let mut S = S.to_vec();
    for j in erasures {
        let Xj = gf256::GENERATOR.pow(u8::try_from(codeword.len() - 1 - j).unwrap());
        for i in 0..S.len() - 1 {
            S[i] = S[i + 1] - S[i] * Xj;
        }
    }

    // trim unnecessary syndromes
    S.drain(S.len() - erasures.len()..);
    S
}

fn find_erasure_locator(codeword: &[gf256], erasures: &[usize]) -> Vec<gf256> {
    let mut Λ = vec![gf256::new(0); erasures.len() + 1];
    let Λ_len = Λ.len();
    Λ[Λ_len - 1] = gf256::new(1);

    for j in erasures {
        poly_mul(
            &mut Λ,
            &[
                -gf256::GENERATOR.pow(u8::try_from(codeword.len() - 1 - j).unwrap()),
                gf256::new(1),
            ],
        );
    }

    Λ
}

fn find_error_locator(S: &[gf256]) -> Vec<gf256> {
    // the current estimate for the error locator polynomial
    let mut Λ = vec![gf256::new(0); S.len() + 1];
    let Λ_len = Λ.len();
    Λ[Λ_len - 1] = gf256::new(1);

    let mut prev_Λ = Λ.clone();
    let mut delta_Λ = Λ.clone();

    // the current estimate for the number of errors
    let mut v = 0;

    for i in 0..S.len() {
        let mut delta = S[i];
        for j in 1..v + 1 {
            delta += Λ[Λ.len() - 1 - j] * S[i - j];
        }

        prev_Λ.rotate_left(1);

        if delta != gf256::new(0) {
            if 2 * v <= i {
                core::mem::swap(&mut Λ, &mut prev_Λ);
                poly_scale(&mut Λ, delta);
                poly_scale(&mut prev_Λ, delta.recip());
                v = i + 1 - v;
            }

            delta_Λ.copy_from_slice(&prev_Λ);
            poly_scale(&mut delta_Λ, delta);
            poly_add(&mut Λ, &delta_Λ);
        }
    }

    // trim leading zeros
    let zeros = Λ.iter().take_while(|x| **x == gf256::new(0)).count();
    Λ.drain(0..zeros);

    Λ
}

fn find_error_locations(codeword: &[gf256], Λ: &[gf256]) -> Vec<usize> {
    let mut error_locations = vec![];
    for j in 0..codeword.len() {
        let Xj = gf256::GENERATOR.pow(u8::try_from(codeword.len() - 1 - j).unwrap());
        let zero = poly_eval(&Λ, Xj.recip());
        if zero == gf256::new(0) {
            // found an error location!
            error_locations.push(j);
        }
    }

    error_locations
}

fn find_error_magnitudes(
    codeword: &[gf256],
    S: &[gf256],
    Λ: &[gf256],
    error_locations: &[usize],
) -> Vec<gf256> {
    // find the erasure evaluator polynomial
    //
    // Ω(x) = S(x)*Λ(x) mod x^2v
    //
    let mut Ω = vec![gf256::new(0); S.len() + Λ.len() - 1];
    let Ω_len = Ω.len();
    Ω[Ω_len - S.len()..].copy_from_slice(&S);
    Ω[Ω_len - S.len()..].reverse();
    poly_mul(&mut Ω, &Λ);
    Ω.drain(..Ω.len() - S.len());

    // find the formal derivative of Λ
    //
    // Λ'(x) = Σ i*Λi*x^(i-1)
    //        i=1
    //
    let mut Λ_prime = vec![gf256::new(0); Λ.len() - 1];
    for i in 1..Λ.len() {
        let mut sum = gf256::new(0);
        for _ in 0..i {
            sum += Λ[Λ.len() - 1 - i];
        }
        let Λ_prime_len = Λ_prime.len();
        Λ_prime[Λ_prime_len - 1 - (i - 1)] = sum;
    }

    // find the error magnitudes
    //
    //        Xj*Ω(Xj^-1)
    // Yj = - -----------
    //         Λ'(Xj^-1)
    //
    // we need to be careful to avoid a divide-by-zero here, this can happen
    // in some cases (provided with incorrect erasures?)
    //
    let mut error_magnitudes = vec![];
    for j in error_locations {
        let Xj = gf256::GENERATOR.pow(u8::try_from(codeword.len() - 1 - j).unwrap());
        let Yj = (-Xj * poly_eval(&Ω, Xj.recip()))
            .checked_div(poly_eval(&Λ_prime, Xj.recip()))
            .unwrap_or(gf256::new(0));
        error_magnitudes.push(Yj);
    }

    error_magnitudes
}

pub fn is_correct(codeword: &[u8]) -> bool {
    let codeword = unsafe { gf256::slice_from_slice_unchecked(codeword) };

    // find syndromes, syndromes of all zero means there are no errors
    let syndromes = find_syndromes(codeword);
    syndromes.iter().all(|s| *s == gf256::new(0))
}

pub fn correct_erasures(codeword: &mut [u8], erasures: &[usize]) -> Result<usize, Error> {
    let codeword = unsafe { gf256::slice_from_slice_mut_unchecked(codeword) };

    // too many erasures?
    if erasures.len() > ECC_SIZE {
        return Err(Error::TooManyErrors);
    }

    // find syndromes, syndromes of all zero means there are no errors
    let S = find_syndromes(codeword);
    if S.iter().all(|s| *s == gf256::new(0)) {
        return Ok(0);
    }

    // find erasure locator polynomial
    let Λ = find_erasure_locator(codeword, &erasures);

    // find erasure magnitudes using Forney's algorithm
    let erasure_magnitudes = find_error_magnitudes(codeword, &S, &Λ, &erasures);

    // correct the errors
    for (&Xj, Yj) in erasures.iter().zip(erasure_magnitudes) {
        codeword[Xj] += Yj;
    }

    // re-find the syndromes to check if we were able to find all errors
    let S = find_syndromes(codeword);
    if !S.iter().all(|s| *s == gf256::new(0)) {
        return Err(Error::TooManyErrors);
    }

    Ok(erasures.len())
}

pub fn correct_errors(codeword: &mut [u8]) -> Result<usize, Error> {
    let codeword = unsafe { gf256::slice_from_slice_mut_unchecked(codeword) };

    // find syndromes, syndromes of all zero means there are no errors
    let S = find_syndromes(codeword);
    if S.iter().all(|s| *s == gf256::new(0)) {
        return Ok(0);
    }

    // find error locator polynomial
    let Λ = find_error_locator(&S);

    // too many errors?
    let error_count = Λ.len() - 1;
    if error_count * 2 > ECC_SIZE {
        return Err(Error::TooManyErrors);
    }

    // find error locations
    let error_locations = find_error_locations(codeword, &Λ);

    // find erasure magnitude using Forney's algorithm
    let error_magnitudes = find_error_magnitudes(codeword, &S, &Λ, &error_locations);

    // correct the errors
    for (&Xj, Yj) in error_locations.iter().zip(error_magnitudes) {
        codeword[Xj] += Yj;
    }

    // re-find the syndromes to check if we were able to find all errors
    let S = find_syndromes(codeword);
    if !S.iter().all(|s| *s == gf256::new(0)) {
        return Err(Error::TooManyErrors);
    }

    Ok(error_locations.len())
}

pub fn correct(codeword: &mut [u8], erasures: &[usize]) -> Result<usize, Error> {
    let codeword = unsafe { gf256::slice_from_slice_mut_unchecked(codeword) };

    // too many erasures?
    if erasures.len() > ECC_SIZE {
        return Err(Error::TooManyErrors);
    }

    // find syndromes, syndromes of all zero means there are no errors
    let S = find_syndromes(codeword);
    if S.iter().all(|s| *s == gf256::new(0)) {
        return Ok(0);
    }

    // find Forney syndromes, hiding known erasures from the syndromes
    let forney_S = find_forney_syndromes(codeword, &S, &erasures);

    // find error locator polynomial
    let Λ = find_error_locator(&forney_S);

    // too many errors/erasures?
    let error_count = Λ.len() - 1;
    let erasure_count = erasures.len();
    if error_count * 2 + erasure_count > ECC_SIZE {
        return Err(Error::TooManyErrors);
    }

    // find all error locations
    let mut error_locations = find_error_locations(codeword, &Λ);
    error_locations.extend_from_slice(&erasures);

    // re-find error locator polynomial, this time including both
    // errors and erasures
    let Λ = find_erasure_locator(codeword, &error_locations);

    // find erasure magnitude using Forney's algorithm
    let error_magnitudes = find_error_magnitudes(codeword, &S, &Λ, &error_locations);

    // correct the errors
    for (&Xj, Yj) in error_locations.iter().zip(error_magnitudes) {
        codeword[Xj] += Yj;
    }

    // re-find the syndromes to check if we were able to find all errors
    let S = find_syndromes(codeword);
    if !S.iter().all(|s| *s == gf256::new(0)) {
        return Err(Error::TooManyErrors);
    }

    Ok(error_locations.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reed_solomon() {
        let mut data = (0..255).collect::<Vec<u8>>();
        encode(&mut data);
        assert!(is_correct(&data));

        // correct up to k known erasures
        for i in 0..(255 - 223) {
            data[0..i].fill(b'x');
            let res = correct_erasures(&mut data, &(0..i).collect::<Vec<_>>());
            assert_eq!(res.ok(), Some(i));
            assert_eq!(&data[0..223], &(0..223).collect::<Vec<u8>>());
        }

        // correct up to k/2 unknown errors
        for i in 0..(255 - 223) / 2 {
            data[0..i].fill(b'x');
            let res = correct_errors(&mut data);
            assert_eq!(res.ok(), Some(i));
            assert_eq!(&data[0..223], &(0..223).collect::<Vec<u8>>());
        }
    }

    #[test]
    fn reed_solomon_any() {
        let mut data = (0..255).collect::<Vec<u8>>();
        encode(&mut data);

        // try any single error
        for i in 0..255 {
            data[i] = b'\xff';
            let res = correct_errors(&mut data);
            assert_eq!(res.ok(), Some(1));
            assert_eq!(&data[0..223], &(0..223).collect::<Vec<u8>>());
        }
    }

    #[test]
    fn reed_solomon_burst() {
        let mut data = (0..255).collect::<Vec<u8>>();
        encode(&mut data);

        // try any burst of k/2 errors
        for i in 0..255 - ((255 - 223) / 2) {
            data[i..i + ((255 - 223) / 2)].fill(b'\xff');
            let res = correct_errors(&mut data);
            assert_eq!(res.ok(), Some((255 - 223) / 2));
            assert_eq!(&data[0..223], &(0..223).collect::<Vec<u8>>());
        }
    }

    // try a shortened message
    #[test]
    fn reed_solomon_shortened() {
        let mut data = (0..40).collect::<Vec<u8>>();
        encode(&mut data);
        assert!(is_correct(&data));

        // correct up to k known erasures
        for i in 0..(40 - 8) {
            data[0..i].fill(b'x');
            let res = correct_erasures(&mut data, &(0..i).collect::<Vec<_>>());
            assert_eq!(res.ok(), Some(i));
            assert_eq!(&data[0..8], &(0..8).collect::<Vec<u8>>());
        }

        // correct up to k/2 unknown errors
        for i in 0..(40 - 8) / 2 {
            data[0..i].fill(b'x');
            let res = correct_errors(&mut data);
            assert_eq!(res.ok(), Some(i));
            assert_eq!(&data[0..8], &(0..8).collect::<Vec<u8>>());
        }
    }
}
