pub struct BytecodeReader<'a> {
    code: &'a Vec<u8>,
    pc: i32,
}

impl<'a> BytecodeReader<'a> {
    pub fn new(code: &'a Vec<u8>, pc: i32) -> BytecodeReader<'a> {
        Self { code, pc }
    }
    pub fn read_u8(&mut self) -> u8 {
        let u = self.code[self.pc];
        self.pc += 1;
        u
    }
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }
    pub fn read_u16(&mut self) -> u16 {
        let high = self.read_u8() as u16;
        let low = self.read_u8() as u16;
        (high << 8) | low
    }
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    pub fn read_i32(&mut self) -> i32 {
        let b1 = self.read_u8() as u32;
        let b2 = self.read_u8() as u32;
        let b3 = self.read_u8() as u32;
        let b4 = self.read_u8() as u32;
        ((b1 << 24) | (b2 << 16) | (b3 << 8) | b4) as i32
    }
    pub fn read_i32s(&mut self, n: i32) -> Vec<i32> {
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
    pub fn reset(&mut self, pc: i32) {
        self.pc = pc;
    }
    pub fn pc(&self) -> i32 { self.pc }
}