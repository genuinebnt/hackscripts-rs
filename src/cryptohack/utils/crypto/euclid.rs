pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = egcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

#[cfg(test)]
mod tests {
    use cryptohack::utils::crypto::euclid::egcd;

    use crate::cryptohack;

    use super::gcd;

    #[test]
    fn test_gct() {
        assert_eq!(21, gcd(1071, 462));
        assert_eq!(25, gcd(75, 25));
        assert_eq!(1, gcd(11, 7));
    }

    #[test]
    fn test_egcd() {
        assert_eq!((1, 267, -22973), egcd(123211, 1432))
    }
}
