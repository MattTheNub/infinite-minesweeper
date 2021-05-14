#[macro_use]
extern crate lazy_static;

mod chunk;
mod colors;
mod map;
mod util;

use std::io::Cursor;

use crate::{chunk::TileState, map::Map};
use speedy2d::{
	color::Color,
	dimen::Vector2,
	image::{ImageFileFormat, ImageSmoothingMode},
	shape::Rectangle,
	window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHelper},
	Graphics2D, Window,
};

// drawing a frame only once causes flickering for some reason
// drawing it twice causes it sometimes; let's use 5 to be safe
pub const REDRAW_TIME: u8 = 5;
const CTRL_PRESSED: u8 = 1 << 0;
const CTRL_USED_MOUSE: u8 = 1 << 1;
static NUMBERS: &[u8] = include_bytes!("../assets/numbers.png");
static FLAG: &[u8] = include_bytes!("../assets/flag.png");
static GAME_OVER: &[u8] = include_bytes!("../assets/game_over.png");

fn main() {
	let window = Window::new_centered("Infinite Minesweeper", (640, 480)).unwrap();
	window.run_loop(WindowHandler {
		map: Map::new(),
		mouse_pos: Vector2::new(-1, -1),
		base_scroll: Vector2::new(-1, -1),
		size: Vector2::new(640, 480),
		scroll: Vector2::new(0, 0),
		ctrl_press: 0,
		lost: false,
	});
}

struct WindowHandler {
	map: Map,
	mouse_pos: Vector2<isize>,
	base_scroll: Vector2<isize>,
	size: Vector2<isize>,
	scroll: Vector2<isize>,
	ctrl_press: u8,
	lost: bool,
}

impl speedy2d::window::WindowHandler for WindowHandler {
	fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
		if self.ctrl_press & CTRL_USED_MOUSE != 0 {
			self.map.redraw = REDRAW_TIME;
			self.scroll = self.mouse_pos - self.base_scroll;
		}

		if self.map.redraw != 0 {
			if self.lost {
				let game_over = graphics
					.create_image_from_file_bytes(
						Some(ImageFileFormat::PNG),
						ImageSmoothingMode::NearestNeighbor,
						Cursor::new(GAME_OVER),
					)
					.unwrap();

				graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
				graphics.draw_rectangle_image(
					Rectangle::new(
						Vector2::new(self.size.x as f32 * 0.1, self.size.y as f32 * 0.1),
						Vector2::new(self.size.x as f32 * 0.9, self.size.y as f32 * 0.9),
					),
					&game_over,
				)
			} else {
				let numbers = graphics
					.create_image_from_file_bytes(
						Some(ImageFileFormat::PNG),
						ImageSmoothingMode::NearestNeighbor,
						Cursor::new(NUMBERS),
					)
					.unwrap();
				let flag = graphics
					.create_image_from_file_bytes(
						Some(ImageFileFormat::PNG),
						ImageSmoothingMode::NearestNeighbor,
						Cursor::new(FLAG),
					)
					.unwrap();
				graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));

				let x_start = self.scroll.x;
				let x_end = self.size.x + self.scroll.x;
				let y_start = self.scroll.y;
				let y_end = self.size.y + self.scroll.y;

				for x in ((x_start % 20 - 20)..(x_end - x_start)).step_by(20) {
					for y in ((y_start % 20 - 20)..(y_end - y_start)).step_by(20) {
						let tile_state = self
							.map
							.get_tile_state(util::div_20(x - x_start), util::div_20(y - y_start));

						match tile_state {
							TileState::Covered(flagged) | TileState::Mine(flagged) => {
								graphics.draw_rectangle(
									Rectangle::new(
										Vector2::new(x as f32, y as f32),
										Vector2::new((x + 20) as f32, (y + 20) as f32),
									),
									if self.mouse_pos.x >= x
										&& self.mouse_pos.x < (x + 20) && self.mouse_pos.y >= y
										&& self.mouse_pos.y < (y + 20)
									{
										*colors::HOVER
									} else {
										match util::modulo((x - x_start) / 20, 2)
											+ util::modulo((y - y_start) / 20, 2)
										{
											0 | 2 => *colors::COVERED_EVEN,
											1 | 3 => *colors::COVERED_ODD,
											_ => unreachable!(),
										}
									},
								);
								if flagged {
									graphics.draw_rectangle_image(
										Rectangle::new(
											Vector2::new(x as f32, y as f32),
											Vector2::new((x + 20) as f32, (y + 20) as f32),
										),
										&flag,
									);
								}
							}
							TileState::Uncovered => {
								graphics.draw_rectangle(
									Rectangle::new(
										Vector2::new(x as f32, y as f32),
										Vector2::new((x + 20) as f32, (y + 20) as f32),
									),
									match util::modulo((x - x_start) / 20, 2)
										+ util::modulo((y - y_start) / 20, 2)
									{
										0 | 2 => *colors::UNCOVERED_EVEN,
										1 | 3 => *colors::UNCOVERED_ODD,
										_ => unreachable!(),
									},
								);
								let number = self.map.get_tile_number(
									util::div_20(x - x_start),
									util::div_20(y - y_start),
								);
								if number != 0 {
									graphics.draw_rectangle_image_subset_tinted(
										Rectangle::new(
											Vector2::new(x as f32, y as f32),
											Vector2::new((x + 20) as f32, (y + 20) as f32),
										),
										Color::WHITE,
										Rectangle::new(
											Vector2::new(0.125 * (number - 1) as f32, 0.0),
											Vector2::new(0.125 * number as f32, 1.0),
										),
										&numbers,
									)
								}
							}
						}
					}
				}

				if cfg!(debug_assertions) {
					for x in ((x_start % 400 - 400)..(x_end - x_start)).step_by(400) {
						graphics.draw_rectangle(
							Rectangle::new(
								Vector2::new((x - 1) as f32, 0.0),
								Vector2::new((x + 1) as f32, self.size.x as f32),
							),
							Color::RED,
						)
					}
					for y in ((y_start % 400 - 400)..(y_end - y_start)).step_by(400) {
						graphics.draw_rectangle(
							Rectangle::new(
								Vector2::new(0.0, (y - 1) as f32),
								Vector2::new(self.size.x as f32, (y + 1) as f32),
							),
							Color::RED,
						)
					}
				}
			}
			self.map.redraw -= 1;
		}
		helper.request_redraw();
	}

	fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vector2<f32>) {
		self.map.redraw = REDRAW_TIME;
		self.mouse_pos = Vector2::new(position.x as isize, position.y as isize);
	}

	fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper, button: MouseButton) {
		if !self.lost {
			match button {
				MouseButton::Left => {
					if self.ctrl_press == 0 {
						if self.map.uncover(
							util::div_20(self.mouse_pos.x - self.scroll.x),
							util::div_20(self.mouse_pos.y - self.scroll.y),
						) {
							self.lost = true;
						}
					}
					self.ctrl_press &= !CTRL_USED_MOUSE;
				}
				MouseButton::Right => {
					self.map.try_flag(
						util::div_20(self.mouse_pos.x - self.scroll.x),
						util::div_20(self.mouse_pos.y - self.scroll.y),
					);
				}
				_ => {}
			}
		}
	}
	fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper, button: MouseButton) {
		if !self.lost {
			match button {
				MouseButton::Left => {
					if self.ctrl_press != 0 {
						self.ctrl_press |= CTRL_USED_MOUSE;
						self.base_scroll = self.mouse_pos - self.scroll;
					}
				}
				_ => {}
			}
		}
	}

	fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: Vector2<u32>) {
		self.size = Vector2::new(size_pixels.x as isize, size_pixels.y as isize);
	}

	fn on_key_down(
		&mut self,
		_helper: &mut WindowHelper,
		virtual_key_code: Option<VirtualKeyCode>,
		_scancode: KeyScancode,
	) {
		if !self.lost {
			if let Some(code) = virtual_key_code {
				match code {
					VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
						self.ctrl_press |= CTRL_PRESSED;
					}
					_ => (),
				}
			}
		}
	}
	fn on_key_up(
		&mut self,
		_helper: &mut WindowHelper,
		virtual_key_code: Option<VirtualKeyCode>,
		_scancode: KeyScancode,
	) {
		if !self.lost {
			if let Some(code) = virtual_key_code {
				match code {
					VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
						self.ctrl_press &= !CTRL_PRESSED;
					}
					_ => (),
				}
			}
		}
	}
}
