// A terrible dependency-free hack.
fn rand() -> u8 {
    unsafe {
      *(&std::time::Instant::now() as *const std::time::Instant as *const u8)
    }
}

fn readln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read from Stdin");
    input
}

fn main() {
    let number = rand() % 101;
    println!("Guess a number between 0 and 100");
    loop {
        let input = readln().trim().parse::<u8>().expect("Passed number isn't a correct number between 0 and 100");
        if input < number {
            println!("Higher...");
            continue;
        } else if input > number {
            println!("Lower...");
            continue;
        } else {
            println!("Correct!");
            break;
	}
    }
}
