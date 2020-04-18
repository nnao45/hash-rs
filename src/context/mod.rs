use std::borrow::BorrowMut;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::{convert, fmt, ops};

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
pub struct Transformer {}

impl Transformer {
    fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }
    fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    fn add(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }

    fn safe_rotate_left(x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32u32 - n))
    }

    fn ff(a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        *a = Self::add(Self::add(Self::add(a.to_owned(), Self::f(b, c, d)), x), ac);
        *a = Self::safe_rotate_left(a.to_owned(), s);
        *a = Self::add(a.to_owned(), b);
    }

    fn gg(a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        *a = Self::add(Self::add(Self::add(a.to_owned(), Self::g(b, c, d)), x), ac);
        *a = Self::safe_rotate_left(a.to_owned(), s);
        *a = Self::add(a.to_owned(), b);
    }

    fn hh(a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        *a = Self::add(Self::add(Self::add(a.to_owned(), Self::h(b, c, d)), x), ac);
        *a = Self::safe_rotate_left(a.to_owned(), s);
        *a = Self::add(a.to_owned(), b);
    }

    fn ii(a: &mut u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) {
        *a = Self::add(Self::add(Self::add(a.to_owned(), Self::i(b, c, d)), x), ac);
        *a = Self::safe_rotate_left(a.to_owned(), s);
        *a = Self::add(a.to_owned(), b);
    }

    fn transform(&self, state: &mut [u32; 4], input: &[u32; 16]) {
        let (mut a, mut b, mut c, mut d) = (state[0], state[1], state[2], state[3]);
        let mut slice = [0u32; 64];
        println!("round0: a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        const S11: u32 = 7;
        const S12: u32 = 12;
        const S13: u32 = 17;
        const S14: u32 = 22;
        Self::ff(a.borrow_mut(), b, c, d, input[0], S11, 3614090360);
        Self::ff(d.borrow_mut(), a, b, c, input[1], S12, 3905402710);
        Self::ff(c.borrow_mut(), d, a, b, input[2], S13, 606105819);
        Self::ff(b.borrow_mut(), c, d, a, input[3], S14, 3250441966);
        Self::ff(a.borrow_mut(), b, c, d, input[4], S11, 4118548399);
        Self::ff(d.borrow_mut(), a, b, c, input[5], S12, 1200080426);
        Self::ff(c.borrow_mut(), d, a, b, input[6], S13, 2821735955);
        Self::ff(b.borrow_mut(), c, d, a, input[7], S14, 4249261313);
        Self::ff(a.borrow_mut(), b, c, d, input[8], S11, 1770035416);
        Self::ff(d.borrow_mut(), a, b, c, input[9], S12, 2336552879);
        Self::ff(c.borrow_mut(), d, a, b, input[10], S13, 4294925233);
        Self::ff(b.borrow_mut(), c, d, a, input[11], S14, 2304563134);
        Self::ff(a.borrow_mut(), b, c, d, input[12], S11, 1804603682);
        Self::ff(d.borrow_mut(), a, b, c, input[13], S12, 4254626195);
        Self::ff(c.borrow_mut(), d, a, b, input[14], S13, 2792965006);
        Self::ff(b.borrow_mut(), c, d, a, input[15], S14, 1236535329);

        println!("round1: a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        const S21: u32 = 5;
        const S22: u32 = 9;
        const S23: u32 = 14;
        const S24: u32 = 20;
        Self::gg(a.borrow_mut(), b, c, d, input[1], S21, 4129170786);
        Self::gg(d.borrow_mut(), a, b, c, input[6], S22, 3225465664);
        Self::gg(c.borrow_mut(), d, a, b, input[11], S23, 643717713);
        Self::gg(b.borrow_mut(), c, d, a, input[0], S24, 3921069994);
        Self::gg(a.borrow_mut(), b, c, d, input[5], S21, 3593408605);
        Self::gg(d.borrow_mut(), a, b, c, input[10], S22, 38016083);
        Self::gg(c.borrow_mut(), d, a, b, input[15], S23, 3634488961);
        Self::gg(b.borrow_mut(), c, d, a, input[4], S24, 3889429448);
        Self::gg(a.borrow_mut(), b, c, d, input[9], S21, 568446438);
        Self::gg(d.borrow_mut(), a, b, c, input[14], S22, 3275163606);
        Self::gg(c.borrow_mut(), d, a, b, input[3], S23, 4107603335);
        Self::gg(b.borrow_mut(), c, d, a, input[8], S24, 1163531501);
        Self::gg(a.borrow_mut(), b, c, d, input[13], S21, 2850285829);
        Self::gg(d.borrow_mut(), a, b, c, input[2], S22, 4243563512);
        Self::gg(c.borrow_mut(), d, a, b, input[7], S23, 1735328473);
        Self::gg(b.borrow_mut(), c, d, a, input[12], S24, 2368359562);

        println!("round2: a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        const S31: u32 = 4;
        const S32: u32 = 11;
        const S33: u32 = 16;
        const S34: u32 = 23;
        Self::hh(a.borrow_mut(), b, c, d, input[5], S31, 4294588738);
        Self::hh(d.borrow_mut(), a, b, c, input[8], S32, 2272392833);
        Self::hh(c.borrow_mut(), d, a, b, input[11], S33, 1839030562);
        Self::hh(b.borrow_mut(), c, d, a, input[14], S34, 4259657740);
        Self::hh(a.borrow_mut(), b, c, d, input[1], S31, 2763975236);
        Self::hh(d.borrow_mut(), a, b, c, input[4], S32, 1272893353);
        Self::hh(c.borrow_mut(), d, a, b, input[7], S33, 4139469664);
        Self::hh(b.borrow_mut(), c, d, a, input[10], S34, 3200236656);
        Self::hh(a.borrow_mut(), b, c, d, input[13], S31, 681279174);
        Self::hh(d.borrow_mut(), a, b, c, input[0], S32, 3936430074);
        Self::hh(c.borrow_mut(), d, a, b, input[3], S33, 3572445317);
        Self::hh(b.borrow_mut(), c, d, a, input[6], S34, 76029189);
        Self::hh(a.borrow_mut(), b, c, d, input[9], S31, 3654602809);
        Self::hh(d.borrow_mut(), a, b, c, input[12], S32, 3873151461);
        Self::hh(c.borrow_mut(), d, a, b, input[15], S33, 530742520);
        Self::hh(b.borrow_mut(), c, d, a, input[2], S34, 3299628645);

        println!("round3: a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        const S41: u32 = 6;
        const S42: u32 = 10;
        const S43: u32 = 15;
        const S44: u32 = 21;
        Self::ii(a.borrow_mut(), b, c, d, input[0], S41, 4096336452);
        Self::ii(d.borrow_mut(), a, b, c, input[7], S42, 1126891415);
        Self::ii(c.borrow_mut(), d, a, b, input[14], S43, 2878612391);
        Self::ii(b.borrow_mut(), c, d, a, input[5], S44, 4237533241);
        Self::ii(a.borrow_mut(), b, c, d, input[12], S41, 1700485571);
        Self::ii(d.borrow_mut(), a, b, c, input[3], S42, 2399980690);
        Self::ii(c.borrow_mut(), d, a, b, input[10], S43, 4293915773);
        Self::ii(b.borrow_mut(), c, d, a, input[1], S44, 2240044497);
        Self::ii(a.borrow_mut(), b, c, d, input[8], S41, 1873313359);
        Self::ii(d.borrow_mut(), a, b, c, input[15], S42, 4264355552);
        Self::ii(c.borrow_mut(), d, a, b, input[6], S43, 2734768916);
        Self::ii(b.borrow_mut(), c, d, a, input[13], S44, 1309151649);
        Self::ii(a.borrow_mut(), b, c, d, input[4], S41, 4149444226);
        Self::ii(d.borrow_mut(), a, b, c, input[11], S42, 3174756917);
        Self::ii(c.borrow_mut(), d, a, b, input[2], S43, 718787259);
        Self::ii(b.borrow_mut(), c, d, a, input[9], S44, 3951481745);

        println!("round4: a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        state[0] = Self::add(state[0], a);
        state[1] = Self::add(state[1], b);
        state[2] = Self::add(state[2], c);
        state[3] = Self::add(state[3], d);
    }
}

#[derive(Clone, Copy)]
struct MD5Context {
    state: [u32; 4],
    count: [u32; 2],
    buffer: [u8; 64],
    transformer: Transformer,
}

impl Debug for MD5Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(println!(
            "{:?},{:?},{:?}",
            self.state.to_vec(),
            self.count.to_vec(),
            self.buffer.to_vec()
        ))
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
            transformer: Transformer {},
        }
    }
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
            input[i] = ((self.buffer[j + 3] as u32) << 24)
                | ((self.buffer[j + 2] as u32) << 16)
                | ((self.buffer[j + 1] as u32) << 8)
                | (self.buffer[j] as u32);
            j += 4;
        }
        self.transformer.transform(&mut self.state, &input);
        let mut digest = [0u8; 16];
        let mut j = 0;
        for i in 0..4 {
            digest[j] = ((self.state[i]) & 0xff) as u8;
            digest[j + 1] = ((self.state[i] >> 8) & 0xff) as u8;
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
        count[1] = count[1].wrapping_add(1);
    }
    count[1] = count[1].wrapping_add(length >> 29);
    for &value in data {
        buffer[k] = value;
        k += 1;
        if k == 0x40 {
            let mut j = 0;
            for i in 0..16 {
                input[i] = ((buffer[j + 3] as u32) << 24)
                    | ((buffer[j + 2] as u32) << 16)
                    | ((buffer[j + 1] as u32) << 8)
                    | (buffer[j] as u32);
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
    let inputs = [
        "",
        "a",
        "abc",
        "message digest",
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "0123456789012345678901234567890123456789012345678901234567890123",
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
    ];
    let outputs = [
        "d41d8cd98f00b204e9800998ecf8427e",
        "0cc175b9c0f1b6a831c399e269772661",
        "900150983cd24fb0d6963f7d28e17f72",
        "f96b697d7cb7938d525a2f31aaf161d0",
        "c3fcd3d76192e4007dfb496cca67e13b",
        "d174ab98d277d9f5a5611c2c9f419d9f",
        "7f7bfd348709deeaace19e3f535f8c54",
        "57edf4a22be3c955ac49da2e2107b67a",
    ];
    for (input, &output) in inputs.iter().zip(outputs.iter()) {
        let mut context = MD5Context::init();
        context.consume(input.as_bytes());
        assert_eq!(format!("{:x}", context.compute()), output);
    }
}
