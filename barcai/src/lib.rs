pub mod constant;
pub mod constant_table;
pub mod value;

use crate::constant_table::BarcaiConstantTable;
use crate::value::BarcaiValue;

pub struct Barcai {
    code: Vec<u8>,
    constants: BarcaiConstantTable
}

impl Barcai {
    pub fn new(code: Vec<u8>, constants: BarcaiConstantTable) -> Self {
        Barcai {
            code,
			constants
        }
    }

    pub fn interpret(self) {
		let mut stack = Vec::new();
		
        let size = self.code.len();
        let mut i = 0usize;
        while i < size {
            let opcode = self.code[i];
			i += 1;
            match opcode {
                0 => { // HALT
                    break;
                },
				1 => { // NOOP
					
				},
				2 => { // Push Constant
					let index = self.code[i];
					i += 1;
					let constant = self.constants.get_constant(index.into());
					stack.push(BarcaiValue::Constant(constant.clone()));
				},
				3 => { // Print
					let value = stack.pop();
					println!("{:?}", value);
				}
                n => {
                    println!("unknown opcode: {}", n);
                }
            }
        }
    }
}