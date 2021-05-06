use rand::Rng;

pub type Tiles<T> = [[T; 20]; 20];

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileState {
	Covered(bool),
	Uncovered,
	Mine(bool),
}
impl Default for TileState {
	fn default() -> Self {
		Self::Covered(false)
	}
}

#[derive(Debug)]
pub struct Chunk {
	pub mines: Tiles<TileState>,
	pub numbers: Option<Tiles<u8>>,
}

impl Chunk {
	pub fn new() -> Self {
		let mut tiles = Tiles::<TileState>::default();
		let mut rng = rand::thread_rng();
		let mut placed_mines = 0u8;
		let target_mines = rng.gen_range(45..=55);
		while placed_mines < target_mines {
			let x = rng.gen_range(0..20);
			let y = rng.gen_range(0..20);

			// make sure the tile is not already a mine
			if let TileState::Covered(_) = tiles[x][y] {
				tiles[x][y] = TileState::Mine(false);
				placed_mines += 1;
			}
		}
		Self {
			mines: tiles,
			numbers: None,
		}
	}

	pub fn init(
		&mut self,
		top_left: &Tiles<TileState>,
		top: &Tiles<TileState>,
		top_right: &Tiles<TileState>,
		right: &Tiles<TileState>,
		bot_right: &Tiles<TileState>,
		bot: &Tiles<TileState>,
		bot_left: &Tiles<TileState>,
		left: &Tiles<TileState>,
	) {
		let mut numbers = Tiles::<u8>::default();

		for x in 0..20 {
			for y in 0..20 {
				if let TileState::Mine(_) = self.mines[x][y] {
					continue;
				}

				let mut number = 0;

				match x {
					0 => {
						match y {
							0 => {
								if let TileState::Mine(_) = top_left[19][19] {
									number += 1;
								}
								if let TileState::Mine(_) = left[19][1] {
									number += 1;
								}
								if let TileState::Mine(_) = top[0][19] {
									number += 1;
								}
								if let TileState::Mine(_) = top[1][19] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y + 1] {
									number += 1;
								}
							}
							19 => {
								if let TileState::Mine(_) = bot_left[19][0] {
									number += 1;
								}
								if let TileState::Mine(_) = left[19][18] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[0][0] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[1][0] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y - 1] {
									number += 1;
								}
							}
							_ => {
								if let TileState::Mine(_) = left[19][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = left[19][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[0][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[0][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[1][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[1][y + 1] {
									number += 1;
								}
							}
						}
						if let TileState::Mine(_) = left[19][y] {
							number += 1;
						}
						if let TileState::Mine(_) = self.mines[x + 1][y] {
							number += 1;
						}
					}
					19 => {
						match y {
							0 => {
								if let TileState::Mine(_) = top_right[0][19] {
									number += 1;
								}
								if let TileState::Mine(_) = right[0][18] {
									number += 1;
								}
								if let TileState::Mine(_) = top[18][19] {
									number += 1;
								}
								if let TileState::Mine(_) = top[19][19] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[18][1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[19][1] {
									number += 1;
								}
							}
							19 => {
								if let TileState::Mine(_) = bot_right[0][0] {
									number += 1;
								}
								if let TileState::Mine(_) = right[0][18] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[18][0] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[19][0] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[18][18] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[19][18] {
									number += 1;
								}
							}
							_ => {
								if let TileState::Mine(_) = right[0][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = right[0][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y + 1] {
									number += 1;
								}
							}
						}
						if let TileState::Mine(_) = right[0][y] {
							number += 1;
						}
						if let TileState::Mine(_) = self.mines[x - 1][y] {
							number += 1;
						}
					}
					_ => {
						match y {
							0 => {
								if let TileState::Mine(_) = top[x][19] {
									number += 1;
								}
								if let TileState::Mine(_) = top[x - 1][19] {
									number += 1;
								}
								if let TileState::Mine(_) = top[x + 1][19] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y + 1] {
									number += 1;
								}
							}
							19 => {
								if let TileState::Mine(_) = bot[x][0] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[x - 1][0] {
									number += 1;
								}
								if let TileState::Mine(_) = bot[x + 1][0] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y - 1] {
									number += 1;
								}
							}
							_ => {
								if let TileState::Mine(_) = self.mines[x][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y + 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x - 1][y - 1] {
									number += 1;
								}
								if let TileState::Mine(_) = self.mines[x + 1][y - 1] {
									number += 1;
								}
							}
						}
						if let TileState::Mine(_) = self.mines[x - 1][y] {
							number += 1;
						}
						if let TileState::Mine(_) = self.mines[x + 1][y] {
							number += 1;
						}
					}
				}
				numbers[x][y] = number;
			}
		}

		self.numbers = Some(numbers);
	}
}
