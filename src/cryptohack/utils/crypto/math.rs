pub fn quadratic_residue(value: u32, modulus: u32) -> Option<Vec<u32>> {
    let mut result = Vec::new();
    for i in 1..modulus {
        if (i.pow(2) % modulus) == value {
            result.push(i);
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::quadratic_residue;

    #[test]
    fn test_quadratic_residue() {
        assert_eq!(quadratic_residue(5, 29), Some(vec![11, 18]));
        assert_eq!(quadratic_residue(18, 29), None);
    }
}
