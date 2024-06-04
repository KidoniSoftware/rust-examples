// Euclidian algorithm
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    }
    else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(61232, gcd(3918848,1653264));
        assert_eq!(3, gcd(357, 234));
        assert_eq!(357, gcd(357, 0));
    }
}
