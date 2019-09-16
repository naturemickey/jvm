pub struct ClassReader {
    data: Vec<u8>,
    pc: usize,
}

impl ClassReader {
    pub fn new(data: Vec<u8>) -> ClassReader {
        Self { data, pc: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.data[self.pc];
        self.pc += 1;
        res
    }

    pub fn read_u16(&mut self) -> u16 {
        const S: usize = std::mem::size_of::<u16>();
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.data[i];
        }
        self.pc += S;
        u16::from_be_bytes(bytes)
    }

    pub fn read_u32(&mut self) -> u32 {
        const S: usize = std::mem::size_of::<u32>();
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.data[i];
        }
        self.pc += S;
        u32::from_be_bytes(bytes)
    }

    pub fn read_u64(&mut self) -> u64 {
        const S: usize = 8;
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.data[i];
        }
        self.pc += S;
        u64::from_be_bytes(bytes)
    }

    pub fn read_u16s(&mut self) -> Vec<u16> {
        let mut n = self.read_u16();
        let mut v = Vec::new();
        while n > 0 {
            v.push(self.read_u16());
            n -= 1;
        }
        v
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut n = length;
        let mut v = Vec::new();
        while n > 0 {
            v.push(self.read_u8());
            n -= 1;
        }
        v
    }

//    pub fn read<T>(&mut self) -> T {
//        const s: usize = std::mem::size_of::<T>();
//        let mut bytes: [u8; s] = [0; s];
//        for i in self.pc..self.pc + s {
//            bytes[i - self.pc] = self.data[i];
//        }
//        self.pc += s;
//        T::from_be_bytes(bytes)
//    }
}