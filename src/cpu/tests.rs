#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

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

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xe8]);
        assert_eq!(cpu.register_x, 0x01);
    }
}
