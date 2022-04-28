pub mod constant;
pub mod constant_table;

pub struct Barcai {
    code: Vec<u8>
}

impl Barcai {
    pub fn new(code: Vec<u8>) -> Self {
        Barcai {
            code
        }
    }

    pub fn interpret(self) {
        let size = self.code.len();
        let mut i = 0usize;
        while i < size {
            let opcode = self.code[i];
            match opcode {
                0 => { // HALT
                    break;
                },

                n => {
                    println!("unknown opcode{}", n);
                }
            }
            i += 1;
        }
    }
}