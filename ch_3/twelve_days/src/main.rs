fn print_intro(index: usize) {
    let intro_1 = "On the ";
    let intro_2 = " day of Christmas\n\
		   My true love gave to me";

    let num = [ "first", "second", "third", "fourth", "fifth", "sixth",
		 "seventh", "eighth", "ninth", "tenth", "eleventh",
		 "twelfth" ];

    println!("{intro_1}{}{intro_2}", num[index]);
}



fn main() {

    let intro = "On the first day of Christmas\n\
		 My true love gave to me";
    let once = "A partridge in a pear tree.";

    
    let lines = [ "Twelve drummers drumming,",
		   "Eleven pipers piping,",
		   "Ten lords a-leaping,",
		   "Nine ladies dancing,",
		   "Eight maids a-milking,",
		   "Seven swans a-swimming,",
		   "Six geese a-laying,",
		   "Five golden rings,",
		   "Four calling birds,",
		   "Three French hens,",
		   "Two turtle doves",
		   "And a partridge in a pear tree." ];

    for day in (0..12) {
	print_intro(day);
	if day == 0 {
	    println!("{once}");
	    println!();
	    continue;
	}

	for i in (11 - day)..12 {
	    println!("{}", lines[i]);
	}
	println!();
    }
}
