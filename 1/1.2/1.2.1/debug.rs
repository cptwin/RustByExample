//Derives the `fmt::debug` for `Structure` this `Structure` contains a single `i32`
#[derive(Debug)]
struct Structure(i32);

//Putting a `Structure` inside the structure of a `Deep` object and we'll make it printable
#[derive(Debug)]
struct Deep(Structure);

fn main() {
	debug_function_one();
	debug_function_two();
}

fn debug_function_one() {
	//Printing with `{:?}` is similar to with `{}`
	println!("{:?} months in a year.", 12);
	println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
	
	//`Structure` is printable!
	println!("Now {:?} will print!", Structure(3));
	
	//According to these notes, the problem with derive is there isn't any control over how the result will look
	//What if you want to just display a 7
	println!("Now {:?} will print!", Deep(Structure(7)));
}

#[derive(Debug)]
struct Person<'a> {
	name: &'a str,
	age: u8
}

fn debug_function_two() {
	let name = "Peter";
	let age = 27;
	let peter = Person { name, age };
	
	//Pretty print :D
	println!("{:?}", peter);
}