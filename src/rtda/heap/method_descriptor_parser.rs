struct MethodDescriptorParser {
    raw: Vec<u8>,
    offset: usize,
    parsed: MethodDescriptor,
}

impl MethodDescriptorParser {
    fn new(descriptor: &str) -> Self {
        let vu8 = descriptor.as_bytes();
        let mut raw = Vec::with_capacity(vu8.len());
        for i in vu8 {
            raw.push(i.clone());
        }
        Self { raw, offset: 0, parsed: MethodDescriptor::new() }
    }

    fn parse(descriptor: &str) -> MethodDescriptor {
        let mut parser = Self::new(descriptor);
        parser.start_params();
        parser.parse_param_types();
        parser.end_params();
        parser.parse_return_type();
        parser.finish();
        parser.parsed
    }

    fn start_params(&mut self) {
        if self.read_u8() != '(' as u8 {
            self.cause_panic();
        }
    }

    fn end_params(&mut self) {
        if self.read_u8() != ')' as u8 {
            self.cause_panic();
        }
    }

    fn finish(&mut self) {
        if self.offset != self.raw.len() {
            self.cause_panic();
        }
    }

    fn cause_panic(&mut self) {
        panic!("BAD descriptor: {:?}", self.raw);
    }

    fn read_u8(&mut self) -> u8 {
        let b = self.raw[self.offset];
        self.offset += 1;
        b
    }

    fn unread_u8(&mut self) {
        self.offset -= 1;
    }

    fn parse_param_types(&mut self) {
        loop {
            let t = self.parse_field_type();
            if t != "".to_string() {
                self.parsed.add_parameter_type(t);
            } else {
                break;
            }
        }
    }

    fn parse_return_type(&mut self) {
        if self.read_u8() == 'V' as u8 {
            self.parsed.return_type = "V".to_string();
        } else {
            self.unread_u8();
            let t = self.parse_field_type();
            if t != "".to_string() {
                self.parsed.return_type = t;
            } else {
                self.cause_panic();
            }
        }
    }

    fn parse_field_type(&mut self) -> String {
        match self.read_u8() as char {
            'B' => "B".to_string(),
            'C' => "C".to_string(),
            'D' => "D".to_string(),
            'F' => "F".to_string(),
            'I' => "I".to_string(),
            'J' => "J".to_string(),
            'S' => "S".to_string(),
            'Z' => "Z".to_string(),
            'L' => return self.parse_object_type(),
            '[' => self.parse_array_type(),
            _ => {
                self.unread_u8();
                "".to_string()
            }
        }
    }

    fn parse_object_type(&mut self) -> String {
        let unread: Vec<u8> = self.raw.drain(self.offset..).collect();

        let mut b = false;
        let mut semicolon_index = 0;
        for i in unread {
            if i == ';' as u8 {
                b = true;
                break;
            }
            semicolon_index += 1;
        }

        if !b {
            self.cause_panic();
            "".to_string()
        } else {
            let obj_start = self.offset - 1;
            let obj_end = self.offset + semicolon_index + 1;
            self.offset = obj_end;
            let descriptor = self.raw.drain(obj_start..obj_end).collect();
            String::from_utf8(descriptor).unwrap()
        }
    }

    fn parse_array_type(&mut self) -> String {
        let arr_start = self.offset - 1;
        self.parse_field_type();
        let arr_end =  self.offset;
        let descriptor = self.raw.drain(arr_start..arr_end).collect();
        String::from_utf8(descriptor).unwrap()
    }
}