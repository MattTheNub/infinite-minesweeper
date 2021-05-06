use speedy2d::color::Color;

lazy_static! {
	pub static ref COVERED_ODD: Color = Color::from_hex_rgb(0x64a16f);
	pub static ref COVERED_EVEN: Color = Color::from_hex_rgb(0x598f62);
	pub static ref UNCOVERED_ODD: Color = Color::from_hex_rgb(0xf5f5f5);
	pub static ref UNCOVERED_EVEN: Color = Color::from_hex_rgb(0xe5e5e5);
	pub static ref HOVER: Color = Color::from_hex_rgb(0x7ecc8b);
}
