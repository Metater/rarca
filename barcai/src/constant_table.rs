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

    pub fn load(data: Vec<u8>) -> Self {
        let size = data.len();
        let i = 0usize;
        while i < size {
            
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data
    }

    pub fn push_i64(&mut self, constant: i64) {
        self.data.push(0u8);
        self.data.extend_from_slice(&constant.to_le_bytes());
    }

    pub fn push_f64(&mut self, constant: f64) {
        self.data.push(1u8);
        self.data.extend_from_slice(&constant.to_le_bytes());
    }

    pub fn push_string(&mut self, constant: String) {
        self.data.push(2u8);
        self.data.extend_from_slice(&constant.as_bytes());
    }

    pub fn read_i64_consant(&self, start: usize) -> i64 {
        i64::from_le_bytes(self.data[start..start + 8].try_into().unwrap())
    }

    pub fn read_f64_consant(&self, start: usize) -> f64 {
        f64::from_le_bytes(self.data[start..start + 8].try_into().unwrap())
    }

    pub fn read_string_consant(&self, start: usize) -> String {
        String::from_utf8(self.data[start..].to_vec()).unwrap()
    }
}