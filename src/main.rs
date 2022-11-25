use std::io;
use std::io::Write;

extern "C" {
    fn rand() -> i32;
    fn srand(seed: i32);
    fn getpid() -> i32;
}

fn inputnum(prompt: &str) -> i32 {
    let mut inp = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inp).unwrap();
    let inp: i32 = match inp.trim().parse() {
        Ok(num) => num,
        Err(_) => -1
    };
    return inp;
}

fn main() {
    let rn: i32;
    unsafe {
        srand(getpid());
        rn = rand() % 100;
    }
    loop {
        let guess = inputnum("enter number: ");
        if guess < 0 {
            continue;
        }
        if guess < rn {
            println!("too small");
        } else if guess > rn {
            println!("too large");
        } else {
            println!("perfectly balanced, as all things should be!");
            break;
        }
    }
}
