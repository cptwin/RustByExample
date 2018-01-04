fn main() {
	//Example of a line comment
	
	//This won't execute
	// println!("Hello, world!");
	
	/*
	* This is a block comment, much like Java, C# etc.
	* More lines go in here
	*/
	
	/*
	Actually you don't need the * per line at all in Rust
	*/
	
	let x = 5 + /* 90 + */ 5;
	println!("Is `x` 10 or 100? x = {}", x);
}