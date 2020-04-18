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

#[derive(Clone, Copy)]
pub struct Transformer{}

impl Transformer {
    fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }
    fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    fn h(x: u32, y: u32, z: u32) -> u32 {
        (x ^ z) | (y ^ z)
    }

    fn i(x: u32, y: u32, z: u32) -> u32 {
        (y ^ x) | (x ^ !z)
    }
    
    fn safe_rotate_left(x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32u32 - n))
    }

    fn ff(mut a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        println!("{}", a);
        println!("{}", Self::f(b, c, d));
        println!("{}", x);
        println!("{}", ac);
        a += Self::f(b, c, d) + x + ac;
        a = Self::safe_rotate_left(a, s);
        a += b;
    }

    fn gg(mut a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        a += Self::g(b, c, d) + x + ac;
        a = Self::safe_rotate_left(a, s);
        a += b;
    }

    fn hh(mut a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        a += Self::h(b, c, d) + x + ac;
        a = Self::safe_rotate_left(a, s);
        a += b;
    }
    
    fn ii(mut a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        a += Self::i(b, c, d) + x + ac;
        a = Self::safe_rotate_left(a, s);
        a += b;
    }

    fn transform(&self, state: &mut [u32; 4], input: &[u32; 16]) {
        let (mut a, mut b, mut c, mut d) = (state[0], state[1], state[2], state[3]);
        let mut slice = [0u32; 64];
        // Tn = [ 2^32 * |sin n| ] (n = 1,2,3...,64)
        let computed_vec: Vec<u32> = (1..65).collect::<Vec<u32>>().iter().map(|c|(((*c as f64).sin()) * (std::u32::MAX as f64)) as u32).collect();
        for (i, b) in computed_vec.into_iter().enumerate() {
            slice[i] = b
        }
         const S11: u32 =  7;
         const S12: u32 = 12;
         const S13: u32 = 17;
         const S14: u32 = 22;
         Self::ff(a, b, c, d, input[ 0], S11, slice[0]);
         Self::ff(d, a, b, c, input[ 1], S12, slice[1]);
         Self::ff(c, d, a, b, input[ 2], S13, slice[2]);
         Self::ff(b, c, d, a, input[ 3], S14, slice[3]);
         Self::ff(a, b, c, d, input[ 4], S11, slice[4]);
         Self::ff(d, a, b, c, input[ 5], S12, slice[5]);
         Self::ff(c, d, a, b, input[ 6], S13, slice[6]);
         Self::ff(b, c, d, a, input[ 7], S14, slice[7]);
         Self::ff(a, b, c, d, input[ 8], S11, slice[8]);
         Self::ff(d, a, b, c, input[ 9], S12, slice[9]);
         Self::ff(c, d, a, b, input[10], S13, slice[10]);
         Self::ff(b, c, d, a, input[11], S14, slice[11]);
         Self::ff(a, b, c, d, input[12], S11, slice[12]);
         Self::ff(d, a, b, c, input[13], S12, slice[13]);
         Self::ff(c, d, a, b, input[14], S13, slice[14]);
         Self::ff(b, c, d, a, input[15], S14, slice[15]);

         const S21: u32 =  5;
         const S22: u32 =  9;
         const S23: u32 = 14;
         const S24: u32 = 20;
         Self::gg(a, b, c, d, input[ 1], S21, slice[16]);
         Self::gg(d, a, b, c, input[ 6], S22, slice[17]);
         Self::gg(c, d, a, b, input[11], S23, slice[18]);
         Self::gg(b, c, d, a, input[ 0], S24, slice[19]);
         Self::gg(a, b, c, d, input[ 5], S21, slice[20]);
         Self::gg(d, a, b, c, input[10], S22, slice[21]);
         Self::gg(c, d, a, b, input[15], S23, slice[22]);
         Self::gg(b, c, d, a, input[ 4], S24, slice[23]);
         Self::gg(a, b, c, d, input[ 9], S21, slice[24]);
         Self::gg(d, a, b, c, input[14], S22, slice[25]);
         Self::gg(c, d, a, b, input[ 3], S23, slice[26]);
         Self::gg(b, c, d, a, input[ 8], S24, slice[27]);
         Self::gg(a, b, c, d, input[13], S21, slice[28]);
         Self::gg(d, a, b, c, input[ 2], S22, slice[29]);
         Self::gg(c, d, a, b, input[ 7], S23, slice[30]);
         Self::gg(b, c, d, a, input[12], S24, slice[31]);

         const S31: u32 =  4;
         const S32: u32 = 11;
         const S33: u32 = 16;
         const S34: u32 = 23;
         Self::hh(a, b, c, d, input[ 5], S31, slice[32]);
         Self::hh(d, a, b, c, input[ 8], S32, slice[33]);
         Self::hh(c, d, a, b, input[11], S33, slice[34]);
         Self::hh(b, c, d, a, input[14], S34, slice[35]);
         Self::hh(a, b, c, d, input[ 1], S31, slice[36]);
         Self::hh(d, a, b, c, input[ 4], S32, slice[37]);
         Self::hh(c, d, a, b, input[ 7], S33, slice[38]);
         Self::hh(b, c, d, a, input[10], S34, slice[39]);
         Self::hh(a, b, c, d, input[13], S31, slice[40]);
         Self::hh(d, a, b, c, input[ 0], S32, slice[41]);
         Self::hh(c, d, a, b, input[ 3], S33, slice[42]);
         Self::hh(b, c, d, a, input[ 6], S34, slice[43]);
         Self::hh(a, b, c, d, input[ 9], S31, slice[44]);
         Self::hh(d, a, b, c, input[12], S32, slice[45]);
         Self::hh(c, d, a, b, input[15], S33, slice[46]);
         Self::hh(b, c, d, a, input[ 2], S34, slice[47]);

        const S41: u32 =  6;
        const S42: u32 = 10;
        const S43: u32 = 15;
        const S44: u32 = 21;
        Self::ii(a, b, c, d, input[ 0], S41, slice[48]);
        Self::ii(d, a, b, c, input[ 7], S42, slice[49]);
        Self::ii(c, d, a, b, input[14], S43, slice[50]);
        Self::ii(b, c, d, a, input[ 5], S44, slice[51]);
        Self::ii(a, b, c, d, input[12], S41, slice[52]);
        Self::ii(d, a, b, c, input[ 3], S42, slice[53]);
        Self::ii(c, d, a, b, input[10], S43, slice[54]);
        Self::ii(b, c, d, a, input[ 1], S44, slice[55]);
        Self::ii(a, b, c, d, input[ 8], S41, slice[56]);
        Self::ii(d, a, b, c, input[15], S42, slice[57]);
        Self::ii(c, d, a, b, input[ 6], S43, slice[58]);
        Self::ii(b, c, d, a, input[13], S44, slice[59]);
        Self::ii(a, b, c, d, input[ 4], S41, slice[60]);
        Self::ii(d, a, b, c, input[11], S42, slice[61]);
        Self::ii(c, d, a, b, input[ 2], S43, slice[62]);
        Self::ii(b, c, d, a, input[ 9], S44, slice[63]);

        state[0] += a;
        state[1] += b;
        state[2] += c;
        state[3] += d;
    }
}

#[derive(Clone, Copy)]
struct MD5Context {
    state: [u32; 4],
    count: [u32; 2],
    buffer: [u8; 64],
    transformer: Transformer
}

impl Debug for MD5Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(println!("{:?},{:?},{:?}", self.state.to_vec(), self.count.to_vec(), self.buffer.to_vec()))
    }
}

macro_rules! implement {
    ($kind:ident, $format:expr) => {
        impl fmt::$kind for Digest {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                for value in &self.0 {
                    write!(formatter, $format, value)?;
                }
                Ok(())
            }
        }
    };
}

implement!(LowerHex, "{:02x}");
implement!(UpperHex, "{:02X}");

impl MD5Context {
    fn init() -> Self {
        MD5Context {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            count: [0, 0],
            buffer: [0; 64],
            transformer: Transformer{}
        }
    }
    // fn comsume(mut self, data: &[u8]) {
    //     let mut input = [0u32; 16];
    //
    //     // バイト列を8ずつ分ける
    //     // 0x3F == 111111
    //     // ビットシフト >> 3は 1/8
    //     let mut k = ((self.count[0] >> 3) & 0x3f) as usize;
    //
    //     // inputの長さをもらう
    //     let length = data.len() as u32;
    //
    //     // inputの長さ を 8倍してカウントに追加
    //     self.count[0] += length << 3;
    //     if self.count[0] < length << 3 {
    //         self.count[1] += 1
    //     }
    //     self.count[1] += length >> 29;
    //
    //     for &value in data {
    //         self.buffer[k] = value;
    //         k += 1;
    //         if k == 0x40 {
    //             let mut j = 0;
    //             for i in 0..16 {
    //                 input[i] = ((self.buffer[j + 3] as u32) << 24) |
    //                 ((self.buffer[j + 2] as u32) << 16) |
    //                 ((self.buffer[j + 1] as u32) <<  8) |
    //                 ((self.buffer[j    ] as u32)      );
    //                 j += 4;
    //             }
    //             self.transformer.transform(self.state.as_mut(), &input);
    //             k = 0;
    //         }
    //     }
    // }
    /// Consume data.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn consume<T: AsRef<[u8]>>(&mut self, data: T) {
        consume(self, data.as_ref());
    }

    /// Consume data.
    #[cfg(target_pointer_width = "64")]
    pub fn consume<T: AsRef<[u8]>>(&mut self, data: T) {
        for chunk in data.as_ref().chunks(core::u32::MAX as usize) {
            consume(self, chunk);
        }
    }
    fn compute(mut self) -> Digest {
        let mut input = [0u32; 16];
        let k = ((self.count[0] >> 3) & 0x3f) as usize;
        input[14] = self.count[0];
        input[15] = self.count[1];
        consume(
            &mut self,
            &PADDING[..(if k < 56 { 56 - k } else { 120 - k })],
        );
        let mut j = 0;
        for i in 0..14 {
            input[i] = ((self.buffer[j + 3] as u32) << 24) |
                ((self.buffer[j + 2] as u32) << 16) |
                ((self.buffer[j + 1] as u32) <<  8) |
                ((self.buffer[j    ] as u32)      );
            j += 4;
        }
        println!("hey");
        self.transformer.transform(&mut self.state, &input);
        let mut digest = [0u8; 16];
        let mut j = 0;
        for i in 0..4 {
            digest[j    ] = ((self.state[i]      ) & 0xff) as u8;
            digest[j + 1] = ((self.state[i] >>  8) & 0xff) as u8;
            digest[j + 2] = ((self.state[i] >> 16) & 0xff) as u8;
            digest[j + 3] = ((self.state[i] >> 24) & 0xff) as u8;
            j += 4;
        }
        Digest(digest)
    }
}

fn consume(
    MD5Context {
        buffer,
        count,
        state,
        transformer,
    }: &mut MD5Context,
    data: &[u8],
) {
    let mut input = [0u32; 16];
    let mut k = ((count[0] >> 3) & 0x3f) as usize;
    let length = data.len() as u32;
    count[0] += length << 3;
    if count[0] < length << 3 {
        count[1] += 1;
    }
    count[1] += length >> 29;
    for &value in data {
        buffer[k] = value;
        k += 1;
        if k == 0x40 {
            let mut j = 0;
            for i in 0..16 {
                input[i] = ((buffer[j + 3] as u32) << 24) |
                    ((buffer[j + 2] as u32) << 16) |
                    ((buffer[j + 1] as u32) <<  8) |
                    ((buffer[j    ] as u32)      );
                j += 4;
            }
            transformer.transform(state, &input);
            k = 0;
        }
    }
}

impl convert::From<MD5Context> for Digest {
    #[inline]
    fn from(context: MD5Context) -> Digest {
        context.compute()
    }
}

#[test]
fn test_context_init() {
    let context = MD5Context::init();
    println!("{:?}", context);
}

#[test]
fn test_compute() {
    let mut context = MD5Context::init();
    context.consume("abc".as_bytes());
    println!("{:x}", context.compute());
}