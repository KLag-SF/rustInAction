struct CPU {
    current_operation: u16,
    registers: [u8: 2],
}

impl CPU{
    fn read_opcode(&self) -> u16{
        self.current_operation
    }
}
fn main() {
}
