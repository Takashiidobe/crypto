//! # Shamir Secret Sharing
//!
//! Shamir Secret Sharing(SSS) is an algorithm for splitting a secret $S$ into a number of shares, $n$,
//! where at least $k$ shares are required to recover the original secret.
//! SSS also has a few nice properties:
//! 1. It has [information-theoretic security](https://en.wikipedia.org/wiki/Information-theoretic_security)
//!    which means that no matter how much computing power adversaries that may want to crack the system
//!    have, the system remains secure. This is due to a fundamental fact about the way secrets and
//!    shares are generated, explained below.
//! 2. Knowledge of any $k - 1$ shares gives no information to the information of the secret, $S$.
//! 3. It is extensible, which means the number of shares $n$ can be added or deleted without
//!    affecting the existing shares.
//! 4. It is dynamic, which means that if one of the shares is compromised, all of the given
//!    shares can be recalculated to generate a new polynomial without requiring a change of the
//!    secret.
//! 5. It is flexible, which means more shares can be given based on some condition, e.g. if one
//!    party should have twice as many shares as another party, that can be handled by the system.
//!
//! The first two principles are most important: no adversary can crack the system without having the $k$
//! required shares, and anybody without those $k$ shares gets no information about the secret,
//! $S$.
//!
//! This works by the following principle: Imagine you have a polynomial of degree $n$. You require
//! at least $n + 1$ points to uniquely define the polynomial. If you have $\leq n$ points, there
//! are infinitely many polynomials that could fit the given points, thus you have learned nothing
//! about the secret $S$ and are no more informed than a participant with no information.
//!
//! Imagine you are trying to recover a line (a polynomial with a degree of 1) from some number of
//! given points, $k$. If $k = 1$, there are infinitely many lines that pass through your given
//! point. Let's say our 1 point is the y-intercept, and it has a value of 3. How many lines have a
//! y-intercept of 3? Infinitely many.
//!
//! $$ y = -\infty..\infty + 3 $$
//!
//! Thus you've learned nothing about the line, since you have to take infinitely many guesses to
//! define the line, which is the same as a participant with no information about the given point.
//!
//! This works for a polynomial of any degree. If you want to recover a polynomial with degree 2 (a
//! parabola), you'd need 3 points to recover it.
//!
//! If we have 3 points for our parabola, however, we can recover the unique polynomial. How we do
//! that is discussed in the next section.
//!
//! ## How it works
//!
//! Now that we know what we want to do (keep a secret value by encoding it as a polynomial), let's
//! go over how we do that.
//!
//! First, there should be some function that takes a secret value, and the number of shares
//! desired and returns a polynomial of the desired degree. In this implementation, we set the
//! secret as the y-intercept and each share is a given $x$ value.
//!
//! Second, we should have a function that decrypts the points. If the correct $k$ points are
//! passed to this function, the function returns the secret $S$ back. This is done with lagrange
//! polynomial interpolation. For any polynomial of degree $n$, the algorithm defines a polynomial
//! where $x$ is equal to 1 for the given point $k_i$, and 0 for all other points.
//!
//! Finally, all of the polynomials are multiplied together to recover the original polynomial (the
//! secret polynomial) and its secret value, the y-intercept can be easily found from it.
//!
//! In pseudocode that would look like the following:
//! ```
//! def interpolate(points: list[(x, y)]) -> number:
//!     let y = 0
//!     for (i, (x_0, y_0)) in enumerate(points):
//!         let l_i = 1
//!         for (j, (x_1, y_1)) in enumerate(points):
//!             if i != j:
//!                 l_i *= x_1 / (x_1 - x_0)
//!         y += l_i * y_0
//!     return y
//! ```
//!
//! And we get back the y-intercept.
//!
//! ## Pitfalls
//!
//! However, SSS also has a few pitfalls:
//!
//! 1. Given that it is information-theoretically secure, if one or more wrong values are provided,
//!    where the number of shares still is $\geq k$, an invalid secret will be returned by the
//!    decryption function.
//!
//! Ideally, we would rather the decryption function tell us which share was incorrect, but there
//! is no way for the function to do so as is. This makes the system attackable indirectly --
//! imagine that of the $k$ parties required, $k-1$ parties are honest and $1$ party is dishonest.
//! The dishonest party could then ask everyone to "reveal" their secret, and also reveal their own
//! phony secret. They would then have information of all the other $k-1$ shares required to
//! recover the secret, and could pair it with their required secret to recover the secret alone.
//! The other $k-1$ members would not be able to find out who was dishonest, since the decryption
//! algorithm gives a successful response but the wrong secret.
//!
use oorandom::Rand32;

use gf256::gf256;

/// This function generates a random polynomial for Shamir's secret sharing.
/// It takes a secret and degree of polynomial to create (the amount of shares)
/// It sets the y-intercept to the secret passed in and then generates as many points as there are
/// shares, and returns the polynomial, with the secret as the first item.
/// Imagine you're trying to create a polynomial with a degree of 2:
/// In math notation that would look like this: $ ax^2 + bx + c $
/// Since the y-intercept will be the secret, we can set the secret to $c$.
/// Then, we generate two values, $a$ and $b$, which define the $x^2$ portion of the polynomial and
/// the $x$.
/// Imagine our $a$ is 7, our $b$ is 5 and our secret is 8. The polynomial would look like this:
/// $7x^2 + 5x + 8$.
/// In code, since we populate the values in reverse, that would be: `vec![8, 5, 7]`.
fn poly_random(rng: &mut Rand32, secret: gf256, degree: usize) -> Vec<gf256> {
    let mut f = vec![secret];
    for _ in 0..degree {
        let num = rng.rand_range(1..255) as u8;
        f.push(gf256::new(num));
    }
    f
}

/// This function takes a polynomial and evaluates it.
fn poly_eval(f: &[gf256], x: gf256) -> gf256 {
    let mut y = gf256::new(0);
    for c in f.iter().rev() {
        y = y * x + c;
    }
    y
}

fn poly_interpolate(xs: &[gf256], ys: &[gf256]) -> gf256 {
    assert!(xs.len() == ys.len());

    let mut y = gf256::new(0);
    for (i, (x0, y0)) in xs.iter().zip(ys).enumerate() {
        let mut li = gf256::new(1);
        for (j, (x1, _y1)) in xs.iter().zip(ys).enumerate() {
            if i != j {
                li *= x1 / (x1 - x0);
            }
        }

        y += li * y0;
    }

    y
}

pub fn generate(secret: &[u8], n: usize, k: usize) -> Vec<Vec<u8>> {
    // we only support up to 255 shares
    assert!(
        n <= usize::try_from(255).unwrap_or(usize::MAX),
        "exceeded {} shares",
        255
    );
    let mut shares = vec![vec![]; n];
    let mut rng = Rand32::new(0);

    // we need to store the x coord somewhere, so just prepend the share with it
    for i in 0..n {
        shares[i].push(u8::try_from(i + 1).unwrap());
    }

    for x in secret {
        // generate a random polynomial for each byte
        let f = poly_random(&mut rng, gf256::new(*x), k - 1);

        // assign each share with a point at f(i)
        for i in 0..n {
            shares[i].push(poly_eval(&f, gf256::new(i as u8 + 1)).0);
        }
    }

    shares
}

pub fn reconstruct<S: AsRef<[u8]>>(shares: &[S]) -> Vec<u8> {
    assert!(
        shares
            .windows(2)
            .all(|ss| ss[0].as_ref().len() == ss[1].as_ref().len()),
        "mismatched share length"
    );

    let mut secret = vec![];
    let len = shares.first().map(|s| s.as_ref().len()).unwrap_or(0);
    if len == 0 {
        return secret;
    }

    // x is prepended to each share
    let xs = shares
        .iter()
        .map(|s| gf256::new(s.as_ref()[0]))
        .collect::<Vec<_>>();
    for i in 1..len {
        let ys = shares
            .iter()
            .map(|s| gf256::new(s.as_ref()[i]))
            .collect::<Vec<_>>();
        secret.push(poly_interpolate(&xs, &ys).0);
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let shares = generate(b"secret secret secret!", 5, 4);

        // <4 can't reconstruct secret
        assert_ne!(reconstruct(&shares[..1]), b"secret secret secret!");
        assert_ne!(reconstruct(&shares[..2]), b"secret secret secret!");
        assert_ne!(reconstruct(&shares[..3]), b"secret secret secret!");

        // >=4 can reconstruct secret
        assert_eq!(reconstruct(&shares[..4]), b"secret secret secret!");
        assert_eq!(reconstruct(&shares[..5]), b"secret secret secret!");
    }
}
