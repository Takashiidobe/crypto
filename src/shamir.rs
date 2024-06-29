use oorandom::Rand32;

use crate::galois_field::Gf256Aes;

fn poly_random(rng: &mut Rand32, secret: Gf256Aes, degree: usize) -> Vec<Gf256Aes> {
    let mut f = vec![secret];
    for _ in 0..degree {
        let num = rng.rand_range(1..255) as u8;
        f.push(Gf256Aes::new(num));
    }
    f
}

fn poly_eval(f: &[Gf256Aes], x: Gf256Aes) -> Gf256Aes {
    let mut y = Gf256Aes::new(0);
    for c in f.iter().rev() {
        y = y * x + c;
    }
    y
}

fn poly_interpolate(xs: &[Gf256Aes], ys: &[Gf256Aes]) -> Gf256Aes {
    assert!(xs.len() == ys.len());

    let mut y = Gf256Aes::new(0);
    for (i, (x0, y0)) in xs.iter().zip(ys).enumerate() {
        let mut li = Gf256Aes::new(1);
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
        let f = poly_random(&mut rng, Gf256Aes::new(*x), k - 1);

        // assign each share with a point at f(i)
        for i in 0..n {
            shares[i].push(poly_eval(&f, Gf256Aes::new(i as u8 + 1)).0);
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
        .map(|s| Gf256Aes::new(s.as_ref()[0]))
        .collect::<Vec<_>>();
    for i in 1..len {
        let ys = shares
            .iter()
            .map(|s| Gf256Aes::new(s.as_ref()[i]))
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
