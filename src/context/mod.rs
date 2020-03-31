use std::{convert, fmt, ops};
use std::ops::Deref;
use std::fmt::{Debug, Formatter, Error};

const PADDING: [u8; 64] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// A digest.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Digest(pub [u8; 16]);

pub struct Functions{}

impl Functions {
    pub fn f(x: &u8, y: &u8, z: &u8) -> u8 {
        x & y | !x & z
    }

    pub fn g(x: &u8, y: &u8, z: &u8) -> u8 {
        x & z | y & !z
    }

    pub fn h(x: &u8, y: &u8, z: &u8) -> u8 {
        x ^ z | y ^ z
    }

    pub fn i(x: &u8, y: &u8, z: &u8) -> u8 {
        y ^ x | x ^ !z
    }
}


struct MD5Context {
    state: [u32; 4],
    count: [u32; 2],
    buffer: [u32; 64]
}

impl Debug for MD5Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(println!("{:?},{:?},{:?}", self.state.to_vec(), self.count.to_vec(), self.buffer.to_vec()))
    }
}

impl MD5Context {
    fn init() -> Self {
        let mut init = [0u32; 64];
        let temp_buf: Vec<u32> = (1..65).collect::<Vec<u32>>().iter().map(|c|(((*c as f64).sin()) * (std::u32::MAX as f64)) as u32).collect();
        for (i, b) in temp_buf.into_iter().enumerate() {
            init[i] = b
        }
        MD5Context {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            count: [0, 0],
            buffer: init,
        }
    }
}

#[test]
fn test_context_init() {
    let mut context = MD5Context::init();
    println!("{:?}", context);
}