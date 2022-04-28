use std::io;
use rand::Rng;

pub struct TicTacToe {
	pub board: Vec<i8>,
	pub selected_symbol: i8,
	pub opponent_symbol: i8,
	pub selected_symbol_char: char
}

impl TicTacToe {
	pub fn play() {
		let mut game = TicTacToe {
			board: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
			selected_symbol: 0,
			opponent_symbol: 0,
			selected_symbol_char: ' '
		};
		game.run();
	}

	fn run(&mut self) {
		self.ask_symbol();
		loop {
			self.print_board();
			self.ask_placement();
			if self.check_win(self.selected_symbol) {
				self.print_board();
				println!("You won!");
				break
			}
			let mut open_places = Vec::new();
			for (i, &place) in self.board.iter().enumerate() {
				if place == 0 {
					open_places.push(i);
				}
			}
			if open_places.len() == 0 {
				self.print_board();
				println!("Scratch ):");
				break
			}
			let opponent_placement = rand::thread_rng().gen_range(0..(open_places.len() as i32));
			self.board[open_places[opponent_placement as usize]] = self.opponent_symbol;
			if self.check_win(self.opponent_symbol) {
				self.print_board();
				println!("Opponent won ):");
				break
			}
		}
	}

	fn ask_symbol(&mut self) {
		loop {
			println!("Would you like to be X or O?");
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Failed to read line");
			match input.trim().to_ascii_uppercase().as_str() {
				"X" => {
					self.selected_symbol = 1;
					self.opponent_symbol = 2;
					self.selected_symbol_char = 'X';
					break
				}
				"O" => {
					self.selected_symbol = 2;
					self.opponent_symbol = 1;
					self.selected_symbol_char = 'O';
					break
				}
				_ => continue
			}
		}
	}

	fn ask_placement(&mut self) {
		println!("Where would you like to place your {}?", self.selected_symbol_char);
		println!("Enter a number 1-9 that is not taken.");
		loop {
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Failed to read line");
			match input.trim().parse::<usize>() {
				Err(_) => {
					println!("Not a number.");
					continue
				},
				Ok(place) => {
					if place < 1 || place > 9 {
						println!("Invalid number.");
						continue
					}
					if self.board[place - 1] != 0 {
						println!("Place taken.");
						continue
					}
					self.board[place - 1] = self.selected_symbol;
					break
				}
			}
		}
	}

	fn check_win(&self, value: i8) -> bool {
		self.check_three_places(0, 1, 2, value) ||
		self.check_three_places(3, 4, 5, value) ||
		self.check_three_places(6, 7, 8, value) ||
		self.check_three_places(0, 3, 6, value) ||
		self.check_three_places(1, 4, 7, value) ||
		self.check_three_places(2, 5, 8, value) ||
		self.check_three_places(0, 4, 8, value) ||
		self.check_three_places(2, 4, 6, value)
	}

	fn check_three_places(&self, a: usize, b: usize, c: usize, value: i8) -> bool {
		self.board[a] == value && self.board[b] == value && self.board[c] == value
	}

	fn print_board(&self) {
		println!("{}", self.format_row(0));
		println!("---------");
		println!("{}", self.format_row(3));
		println!("---------");
		println!("{}", self.format_row(6));
	}
	
	fn format_row(&self, row_start_index: usize) -> String {
		let mut row = String::new();
		row.push_str(self.get_board_char(row_start_index).as_str());
		row.push_str(" | ");
		row.push_str(self.get_board_char(row_start_index + 1).as_str());
		row.push_str(" | ");
		row.push_str(self.get_board_char(row_start_index + 2).as_str());
		row
	}

	fn get_board_char(&self, board_index: usize) -> String {
		match self.board[board_index] {
			1 => String::from("X"),
			2 => String::from("O"),
			_ => (board_index + 1).to_string()
		}
	}
}