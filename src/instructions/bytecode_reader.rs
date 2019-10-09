struct BytecodeReader {
    code: Vec<u8>,
    pc: usize,
}

impl BytecodeReader {
    fn new(code: Vec<u8>, pc: usize) -> BytecodeReader {
        Self { code, pc }
    }
    fn read_u8(&mut self) -> u8 {
        let u = self.code[self.pc];
        self.pc += 1;
        u
    }
    fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }
    fn read_u16(&mut self) -> u16 {
        let high = self.read_u8() as u16;
        let low = self.read_u8() as u16;
        (high << 8) | low
    }
    fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    fn read_i32(&mut self) -> i32 {
        let b1 = self.read_u8() as u32;
        let b2 = self.read_u8() as u32;
        let b3 = self.read_u8() as u32;
        let b4 = self.read_u8() as u32;
        ((b1 << 24) | (b2 << 16) | (b3 << 8) | b4) as i32
    }
    fn read_i32s(&mut self, n: i32) -> Vec<i32> {
        let mut ints = Vec::with_capacity(n as usize);
        for _ in 0..n {
            ints.push(self.read_i32());
        }
        ints
    }
    fn skip_padding(&mut self) {
        while self.pc % 4 != 0 {
            self.read_u8();
        }
    }
}