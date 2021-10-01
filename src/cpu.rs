pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        CPU::new()
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    // todo: implement addressing mode usage
    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }

                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }

                0xb5 => {
                    self.lda(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }

                0xAD => {
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 2;
                }

                0xbd => {
                    self.lda(&AddressingMode::Absolute_X);
                    self.program_counter += 2;
                }

                0xb9 => {
                    self.lda(&AddressingMode::Absolute_Y);
                    self.program_counter += 2;
                }

                0xa1 => {
                    self.lda(&AddressingMode::Indirect_X);
                    self.program_counter += 1;
                }

                0xb1 => {
                    self.lda(&AddressingMode::Indirect_Y);
                    self.program_counter += 1;
                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                0x00 => return,

                _ => todo!(),
            }
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        //shifting 8 bits to the right
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_operand_address(addressing_mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        dbg!(
            addr,
            value,
            self.register_a,
            self.register_x,
            self.register_y
        );
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn update_zero_flag(&mut self, value: u8) {
        //check if register a is 0 and if it is we set zero flag to 1 else we set it to 0
        if value == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, value: u8) {
        //check if the negative bit of register a is set if it is we set the negative bit of the status
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                //maybe make pointer u16? try after tests have been written to validate functionality
                //probably not possible for the sake of the wrapping add on u8
                let pointer: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(pointer as u16);
                let hi = self.mem_read(pointer.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                //maybe make pointer u16? try after tests have been written to validate functionality
                //probably not possible for the sake of the wrapping add on u8
                let pointer: u8 = base.wrapping_add(self.register_y);
                let lo = self.mem_read(pointer as u16);
                let hi = self.mem_read(pointer.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lda_status_flags(cpu: CPU) {
        assert_eq!(cpu.status & 0b0000_0010, 0b00);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        lda_status_flags(cpu);
    }

    #[test]
    fn test_0xa5_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xc1, 0x01);

        cpu.load_and_run(vec![0xa5, 0xc1, 0x00]);
        assert_eq!(cpu.register_a, 0x01);
        lda_status_flags(cpu);
    }

    #[test]
    fn test_0xb5_lda_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xc2, 0x01);

        cpu.load_and_run(vec![0xe8, 0xb5, 0xc1, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        lda_status_flags(cpu);
    }

    #[test]
    fn test_0xad_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xc1c2, 0x01);

        cpu.load_and_run(vec![0xad, 0xc2, 0xc1, 0x00]);
        assert_eq!(cpu.register_a, 0x01);
        lda_status_flags(cpu);
    }

    #[test]
    fn test_0xbd_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xc1c4, 0x01);

        cpu.load_and_run(vec![0xe8, 0xe8, 0xbd, 0xc2, 0xc1, 0x00]);
        assert_eq!(cpu.register_a, 0x01);
        lda_status_flags(cpu);
    }

    //TODO: TEST AFTER OP CODE TO MODIFY Y HAS BEEN IMPLEMENTED

    // #[test]
    // fn test_0xb9_lda_absolute_y() {
    //     let mut cpu = CPU::new();
    //     cpu.mem_write(0xc1c5, 0x01);
    //     cpu.register_y = 0x03;
    //
    //     cpu.load_and_run(vec![0xb9, 0xc2, 0xc1, 0x00]);
    //     assert_eq!(cpu.register_a, 0x04);
    //     lda_status_flags(cpu);
    // }

    #[test]
    fn test_0xa1_lda_indirect_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x06, 0x01);
        cpu.mem_write(0x01, 0x03);

        cpu.load_and_run(vec![0xe8, 0xe8, 0xa1, 0x04, 0x00]);
        assert_eq!(cpu.register_a, 0x03);
        lda_status_flags(cpu);
    }

    //TODO: TEST AFTER OP CODIE TO MODIFY Y HAS BEEN IMPLEMENTED

    // #[test]
    // fn test_0xb1_lda_indirect_y() {
    //     let mut cpu = CPU::new();
    //     cpu.mem_write(0x07, 0xcc);
    //     cpu.mem_write(0xcc, 0xff);
    //     //todo: increment y to 0x03
    //     cpu.load_and_run(vec![0xb1, 0x04, 0x00]);
    //     assert_eq!(cpu.register_a, 0xff);
    //     lda_status_flags(cpu);
    // }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_0xaa_txa_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_lda_txa_inx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1)
    }
}
