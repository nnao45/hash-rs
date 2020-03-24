use bitvec::prelude::*;

use std::iter::repeat;

const TEN_RADIX: u32 = 10;

struct MD5Input {
    bytes: Vec<Vec<u32>>,
}

impl MD5Input {
    fn new(bv: &[u8]) -> Self {
        MD5Input {
            bytes: bv
                .iter()
                .map(|b| {
                    format!("{:b}", b)
                        .chars()
                        .into_iter()
                        .map(|c| c.to_digit(TEN_RADIX).unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
        }
    }
}

fn main() {
    let mut bv = bitvec![Msb0, u8; 0, 1, 0, 1];;
    println!("{}", bv);
    let bit_arr: Vec<u32> = format!("{:b}", 123)
        .chars()
        .into_iter()
        .map(|c| c.to_digit(TEN_RADIX).unwrap())
        .collect();
    "abc".as_bytes().iter().for_each(|c| println!("{:b}", c));
    let bit_arr2 = MD5Input::new("abc".as_bytes());
    println!("{:?}", bit_arr2.bytes);
}

#[cfg(test)]
mod tests {
    use crate::MD5Input;

    #[test]
    fn test_md5_input() {
        let b = "abc".as_bytes();
        let expect: Vec<String> = b
            .iter()
            .map(|c| format!("{:b}", c))
            .collect();
        let input = MD5Input::new(b);
        let actual: Vec<String> = input
            .bytes
            .into_iter()
            .map(|v| {
                v.iter()
                    .fold("".to_string(), |init, c| init + format!("{}", c).as_ref())
            })
            .collect();

        assert_eq!(expect, actual)
    }
}
