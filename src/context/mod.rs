struct MD5Context {
    state: [u32; 4],
    count: [u32; 2],
    buffer: [char; 64]
}

impl MD5Context {
    fn init() -> Self {
        MD5Context {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            count: [0, 0],
            buffer: ['a' ;64],
        }
    }

    fn decode(&mut self, input: &[char], input_len: u32) -> Vec<u32> {
        let mut i = 0;
        let mut j = 0;
        let mut result = vec![];
        while j < input_len {
            result[i] = input[j] | (input[j + 1] << 8) | (input[j + 2] << 16) | (input[j + 3] << 24);
            i += 1;
            j += 4;
        }
        result
    }

    fn transform(&self) {
        let a = self.state[0];
        let b = self.state[1];
        let c = self.state[2];
        let d = self.state[3];
        let x : [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    fn update(&mut self, input: &char, input_len: u32) {
        let index = (self.count[0] >> 3) & 0x3F;
        if (self.count[0] + input_len << 3) < (input_len << 3) {
            self.count[1] + 1;
            self.count[1] + (input_len >> 29);
        };

        let part_len = 64 - index;

        if (input_len >= part_len) {
            MD5ContextOutput::init(self.buffer, part_len)
        }
    }
}