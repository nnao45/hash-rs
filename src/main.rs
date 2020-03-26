use std::borrow::Borrow;

const S11: u32 = 7;
const S12: u32 = 12;
const S13: u32 = 17;
const S14: u32 = 22;
const S21: u32 = 5;
const S22: u32 = 9;
const S23: u32 = 14;
const S24: u32 = 20;
const S31: u32 = 4;
const S32: u32 = 11;
const S33: u32 = 16;
const S34: u32 = 23;
const S41: u32 = 6;
const S42: u32 = 10;
const S43: u32 = 15;
const S44: u32 = 21;

const PADDING:[u8; 64] = [0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn f(x: u8, y: u8, z: u8) -> u8 {
    x & y | !x & z
}

fn g(x: u8, y: u8, z: u8) -> u8 {
    x & z | y & !z
}

fn h(x: u8, y: u8, z: u8) -> u8 {
    x ^ z | y ^ z
}

fn i(x: u8, y: u8, z: u8) -> u8 {
    y ^ x | x ^ !z
}

fn rotate_left(x: u8, n: u8) -> u8 {
    x << n | x >> (32 - n)
}

fn double_order(a: u8, b: u8, c: u8, d: u8, x: u8, s: u8, ac: u8, op: Box<dyn Fn(u8, u8, u8) -> u8>) -> u8 {
    rotate_left(op.borrow(b, c, d) + x + ac, s) + b
}

fn main() {

}