#[cfg(test)]
mod tests {
    use cryptohack::utils::crypto::math::quadratic_residue;

    use crate::cryptohack;

    #[test]
    fn solve_quadratic_residues() {
        let p = 29;
        let values = vec![14, 6, 11];
        let mut smallest = std::u32::MAX;
        for value in values {
            let residues = quadratic_residue(value, p);
            if residues.is_some() {
                for residue in residues.unwrap().iter() {
                    if residue < &smallest {
                        smallest = *residue;
                    }
                }
            }
        }
        assert_eq!(smallest, 8);
    }
}
