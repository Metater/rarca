use std::convert::TryInto;
use std::mem::size_of;

use crate::constant::BarcaiConstant;

pub struct BarcaiConstantTable {
    data: Vec<u8>,
    table: Vec<BarcaiConstant>
}

impl BarcaiConstantTable {
    pub fn new() -> Self {
        BarcaiConstantTable {
            data: Vec::new(),
            table: Vec::with_capacity(0)
        }
    }

	pub fn get_data(self) -> Vec<u8> {
        self.data
    }

    pub fn load_data(data: Vec<u8>) -> Self {
		let mut table = Vec::new();
		
        let data_length = data.len();
        let mut i = 0usize;
        while i < data_length {
			let constant_type = data[i];
			i += 1;
            match constant_type {
				0 => {
					let size = size_of::<i64>();
					table.push(BarcaiConstant::I64(i64::from_ne_bytes(data[i..i + size].try_into().unwrap())));
					i += size;
				},
				1 => {
					let size = size_of::<f64>();
					table.push(BarcaiConstant::F64(f64::from_ne_bytes(data[i..i + size].try_into().unwrap())));
					i += size;
				},
				2 => {
					let size = size_of::<usize>();
					let length = usize::from_ne_bytes(data[i..i + size].try_into().unwrap());
					i += size;
					table.push(BarcaiConstant::String(String::from_utf8(data[i..i + length].to_vec()).unwrap()));
					i += length;
				},
				n => {
					panic!("tried to load unknown constant type: {}", n);
				}
			}
        }
		BarcaiConstantTable {
			data: Vec::with_capacity(0),
			table
		}
    }

	pub fn get_constant(&self, index: usize) -> &BarcaiConstant {
		&self.table[index]
	}

    pub fn push_i64(&mut self, constant: i64) {
        self.data.push(0u8);
        self.data.extend_from_slice(&constant.to_ne_bytes());
    }

    pub fn push_f64(&mut self, constant: f64) {
        self.data.push(1u8);
        self.data.extend_from_slice(&constant.to_ne_bytes());
    }

    pub fn push_string(&mut self, constant: String) {
        self.data.push(2u8);
		let bytes = constant.as_bytes();
		let length = bytes.len();
		self.data.extend_from_slice(&length.to_ne_bytes());
        self.data.extend_from_slice(&constant.as_bytes());
    }
}