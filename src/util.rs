pub fn modulo(base: isize, modulo: isize) -> isize {
	if base < 0 {
		(modulo + (base % modulo)) % modulo
	} else {
		base % modulo
	}
}

pub fn div_20(mut num: isize) -> isize {
	if num < 0 {
		num -= 19;
	}
	num / 20
}
