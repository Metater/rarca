use barcai::Barcai;

fn main() {
    println!("Hello, world!");

    let mut constants = Vec::new();

    barcai::constants::push_i64_constant(&mut constants, 10);

    let i = barcai::constants::read_i64_consant(&constants, 1);
    print!("{}", i);

    let code = vec![0u8; 1024];

    let barcai = Barcai::new(code);

    barcai.interpret();
}