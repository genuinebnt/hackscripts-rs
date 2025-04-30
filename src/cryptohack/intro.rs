#[cfg(test)]
mod tests {
    use base64::prelude::*;
    use num_bigint::BigUint;
    use num_traits::Num;

    use cryptohack::utils::{
        bytes::{
            common::{bytes_to_string, hex_to_bytes, long_to_bytes},
            xor::xor,
        },
        crypto::euclid::{egcd, gcd},
    };

    use crate::cryptohack;

    #[test]
    fn solve_ascii() {
        let input = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        let got = bytes_to_string(&input);
        let want = "crypto{ASCII_pr1nt4bl3}";
        assert_eq!(got, want);
    }

    #[test]
    fn solve_hex() {
        let input = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
        let output = bytes_to_string(&hex_to_bytes(input).unwrap());
        assert_eq!(output, "crypto{You_will_be_working_with_hex_strings_a_lot}");
    }

    #[test]
    fn solve_base64() {
        let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
        let output = hex_to_bytes(input).unwrap();
        assert_eq!(
            BASE64_STANDARD.encode(output),
            "crypto/Base+64+Encoding+is+Web+Safe/"
        );
    }

    #[test]
    fn solve_bytes_and_big_integers() {
        let long =
            "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
        let long = BigUint::from_str_radix(long, 10).unwrap();
        let bytes = long_to_bytes(long).unwrap();
        assert_eq!(bytes_to_string(&bytes), "crypto{3nc0d1n6_4ll_7h3_w4y_d0wn}");
    }

    #[test]
    fn solve_xor_starter() {
        let value = "label";
        let output = xor(value.as_bytes(), &[13]);
        assert_eq!(
            "crypto{aloha}",
            format!("crypto{{{}}}", bytes_to_string(&output))
        );
    }

    #[test]
    fn solve_xor_properties() {
        let key1 = hex_to_bytes("a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313").unwrap();
        let key2_xor_key3 =
            hex_to_bytes("c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1").unwrap();
        let key1_xor_key2_xor_key3 = xor(&key2_xor_key3, &key1);

        let flag_xor_key1_xor_key3_xor_key2 =
            hex_to_bytes("04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf").unwrap();

        let flag = xor(&flag_xor_key1_xor_key3_xor_key2, &key1_xor_key2_xor_key3);

        assert_eq!("crypto{x0r_i5_ass0c1at1v3}", bytes_to_string(&flag));
    }

    #[test]
    fn solve_favorite_byte() {
        let input =
            hex_to_bytes("73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d")
                .unwrap();
        let mut flag = "".to_string();
        for i in 0..128 {
            let result = xor(&input, &[i]);
            if result.starts_with(b"crypto{") {
                flag = bytes_to_string(&result);
                break;
            }
        }
        assert_eq!("crypto{0x10_15_my_f4v0ur173_by7e}", flag);
    }

    #[test]
    fn solve_you_either_know_xor_you_dont() {
        let input = hex_to_bytes(
            "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104",
        )
        .unwrap();
        let result = bytes_to_string(&xor(&input, b"crypto{"));
        let result = bytes_to_string(&xor(&input, b"myXORkey"));
        assert_eq!(result, "crypto{1f_y0u_Kn0w_En0uGH_y0u_Kn0w_1t_4ll}");
    }

    #[test]
    fn solve_greatest_common_divisor() {
        assert_eq!(gcd(66528, 52920), 1512);
    }

    #[test]
    fn solve_extended_gcd() {
        let (_gcd, u, v) = egcd(26513, 32321);
        let result = if u < v { u } else { v };
        assert_eq!(result, -8404);
    }

    #[test]
    fn solve_arithmetic_1() {
        let x = 11 % 6;
        let y: u64 = 8146798528947 % 17;
        let result = if x < y { x } else { y };
        assert_eq!(4, result);
    }

    #[test]
    fn solve_modular_inverting() {
        let (_gcd, u, _v) = egcd(13, 3);
        assert_eq!(-1, _v);
    }
}
