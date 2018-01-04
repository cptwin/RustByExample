fn main() {
	
	// Generally `{}` represent things that will be automatically replaced and stringified
	println!("{} days", 31);
	
	//You can use numbers for positional arguments
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
	
	//You can also name arguments
	println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
	
	//Special formatting can be specified after a colon `:`
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
	
	//You can right-align text with a specified width
	println!("{number:>width$}", number=1, width=6);
	
	//You can pad numbers with extra zeros.
	println!("{number:>0width$}", number=1, width=6);
	
	#[allow(dead_code)]
	struct Structure(i32);
	
	//Some custom types such as the above structure require more complicated handling.
	//println!("This struct `{}` won't print...", Structure(3));
	
}