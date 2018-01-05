//Import via `use`
use std::fmt;

//Part One

struct Structure(i32);

//In order to use `{}` marker, the trait `fmt::Display` must be implemented manually
impl fmt::Display for Structure {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		//Write strictly the first element into the supplied output stream `f:`
		//Returns `fmt::Result` which indicates whether the operation succeeded or failed
		//Note that `write!` use syntax which is similar to `println!`
		write!(f, "{}", self.0)
	}
}

//Part Two

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// User `self.number` to refer to each positional data point.
		write!(f, "({}, {})", self.0, self.1)
	}
}

#[derive(Debug)]
struct Point2D {
	x: f64,
	y: f64,
}

#[derive(Debug)]
struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Point2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		//Customize so only `x` and `y` are denoted
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

fn main() {
	let minmax = MinMax(0, 14);
	
	println!("Compare structures:");
	println!("Display: {}", minmax);
	println!("Debug: {:?}", minmax);
	
	let big_range = MinMax(-300, 300);
	let small_range = MinMax(-3, 3);
	
	println!("The bug range is {big} and the small is {small}", small = small_range, big = big_range);
	
	let point = Point2D { x: 3.3, y: 7.2};
	
	println!("Compare points:");
	println!("Display: {}", point);
	println!("Debug: {:?}", point);
	
	//Error both `debug` and `display` were implemented but `{:b}` requires `fmt::Binary`
	//This next line won't work
	//println!("What does Point2D look like in binary: {:b}?", point);
	
	let complex = Complex { real: 3.3, imag: 7.2};
	
	println!("Display: {}", complex);
	println!("Debug: {:?}", complex);
}