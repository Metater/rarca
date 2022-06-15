use barcai::Barcai;
use barcai::constant_table::BarcaiConstantTable;
use barcai::constant::BarcaiConstant;

fn get_constants() -> BarcaiConstantTable {
	let mut constants = BarcaiConstantTable::new();
	
	constants.push_string(String::from("Hello, world!"));
	constants.push_string(String::from("Hello!"));
	constants.push_i64(321312);
	constants.push_f64(3.14159);
	constants.push_string(String::from("Hello, world!"));

	let data = constants.get_data();

	BarcaiConstantTable::load_data(data)
}

fn main() {
	let constants = get_constants();

	/*
	for i in 0..5 {
		println!("{:?}", constants.get_constant(i))	
	}
	*/

    let code = vec![
					2u8, 4u8, // push constant 4
					3u8, // print top of stack
					2u8, 3u8, // push constant 3
					3u8, // print top of stack
				   ];

    let barcai = Barcai::new(code, constants);

    barcai.interpret();
}