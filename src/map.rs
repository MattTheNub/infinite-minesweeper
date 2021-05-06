use std::collections::HashMap;

use crate::{
	chunk::{Chunk, TileState},
	util, REDRAW_TIME,
};

pub struct Map {
	data: HashMap<(isize, isize), Chunk>,
	pub initialized: bool,
	pub redraw: u8,
}
impl Map {
	pub fn new() -> Self {
		Self {
			data: HashMap::<(isize, isize), Chunk>::default(),
			initialized: false,
			redraw: REDRAW_TIME,
		}
	}

	fn ensure_chunk(&mut self, x: isize, y: isize) {
		if !self.data.contains_key(&(x, y)) {
			self.data.insert((x, y), Chunk::new());
		}
	}
	fn ensure_numbers(&mut self, x: isize, y: isize) {
		let chunk = self.get_chunk_for_tile(x, y);
		if let None = chunk.numbers {
			let top_left = self.get_chunk_for_tile(x - 20, y - 20).mines;
			let top = self.get_chunk_for_tile(x, y - 20).mines;
			let top_right = self.get_chunk_for_tile(x + 20, y - 20).mines;
			let right = self.get_chunk_for_tile(x + 20, y).mines;
			let bot_right = self.get_chunk_for_tile(x + 20, y + 20).mines;
			let bot = self.get_chunk_for_tile(x, y + 20).mines;
			let bot_left = self.get_chunk_for_tile(x - 20, y + 20).mines;
			let left = self.get_chunk_for_tile(x - 20, y).mines;

			let chunk = self.get_chunk_for_tile_mut(x, y);
			chunk.init(
				&top_left, &top, &top_right, &right, &bot_right, &bot, &bot_left, &left,
			);
		}
	}

	pub fn get_chunk(&mut self, x: isize, y: isize) -> &Chunk {
		self.ensure_chunk(x, y);
		self.data.get(&(x, y)).unwrap()
	}
	pub fn get_chunk_mut(&mut self, x: isize, y: isize) -> &mut Chunk {
		self.ensure_chunk(x, y);
		self.data.get_mut(&(x, y)).unwrap()
	}
	pub fn get_chunk_for_tile(&mut self, x: isize, y: isize) -> &Chunk {
		self.get_chunk(util::div_20(x), util::div_20(y))
	}
	pub fn get_chunk_for_tile_mut(&mut self, x: isize, y: isize) -> &mut Chunk {
		self.get_chunk_mut(util::div_20(x), util::div_20(y))
	}
	pub fn get_tile_state(&mut self, x: isize, y: isize) -> TileState {
		self.get_chunk_for_tile(x, y).mines[util::modulo(x, 20) as usize]
			[util::modulo(y, 20) as usize]
	}
	pub fn get_tile_state_mut(&mut self, x: isize, y: isize) -> &mut TileState {
		&mut self.get_chunk_for_tile_mut(x, y).mines[util::modulo(x, 20) as usize]
			[util::modulo(y, 20) as usize]
	}
	pub fn get_tile_number(&mut self, x: isize, y: isize) -> u8 {
		self.ensure_numbers(x, y);
		self.get_chunk_for_tile(x, y).numbers.unwrap()[util::modulo(x, 20) as usize]
			[util::modulo(y, 20) as usize]
	}
	// Returns true if the tile was a mine
	pub fn uncover(&mut self, x: isize, y: isize) -> bool {
		self.redraw = REDRAW_TIME;

		if !self.initialized {
			// ensure there are no mines in a 5x5 area around the first tile
			for dx in -2..=2 {
				for dy in -2..=2 {
					let tile_state = self.get_tile_state_mut(x + dx, y + dy);
					*tile_state = TileState::Covered(false);
				}
			}
			self.initialized = true;
		}

		let tile_state = self.get_tile_state_mut(x, y);

		match *tile_state {
			TileState::Covered(flagged) | TileState::Mine(flagged) => {
				if flagged {
					return false;
				}
			}
			_ => (),
		}

		if let TileState::Mine(_) = *tile_state {
			true
		} else {
			*tile_state = TileState::Uncovered;
			if self.get_tile_number(x, y) == 0 {
				for dx in -1..=1 {
					for dy in -1..=1 {
						if (dx != 0 || dy != 0)
							&& self.get_tile_state(x + dx, y + dy) != TileState::Uncovered
						{
							self.uncover(x + dx, y + dy);
						}
					}
				}
			}

			false
		}
	}

	pub fn try_flag(&mut self, x: isize, y: isize) {
		self.redraw = REDRAW_TIME;
		let tile_state = self.get_tile_state_mut(x, y);
		match *tile_state {
			TileState::Covered(flagged) => *tile_state = TileState::Covered(!flagged),
			TileState::Mine(flagged) => *tile_state = TileState::Mine(!flagged),
			TileState::Uncovered => (),
		}
	}
}
