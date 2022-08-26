// A terrible dependency-free hack.
fn rand() -> u64 {
    let now = std::time::SystemTime::now();
    let some_number = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
	let mut xorshift = some_number;
	xorshift ^= xorshift << 13;
	xorshift ^= xorshift >> 7;
	xorshift ^= xorshift << 17;
    return xorshift;
}

fn main() {
    let number = rand() % 101;
    println!("Guess a number between 0 and 100");
    loop {
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        let input = input_string.trim().parse::<u64>().unwrap();
        if input < number {
            println!("Higher...");
            continue;
        }
        if input > number {
            println!("Lower...");
            continue;
        }
        println!("Correct!");
        break;
    }
}