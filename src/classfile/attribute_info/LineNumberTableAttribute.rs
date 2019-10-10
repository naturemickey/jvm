pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>
}

struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTableAttribute {
    fn new(reader: &mut ClassReader) -> LineNumberTableAttribute {
        let line_number_table_length = reader.read_u16();
        let mut line_number_table = Vec::new();
        for _ in 0..line_number_table_length {
            let start_pc = reader.read_u16();
            let line_number = reader.read_u16();
            line_number_table.push(LineNumberTableEntry { start_pc, line_number });
        }
        Self { line_number_table }
    }
}