use std::io;

const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
const ANSWER: &str = "The letter e\n";

fn main() {
    let mut trials = 0;

    loop {
        println!("{}", RIDDLE);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        trials += 1;

        if input == ANSWER {
            break;
        }
    }

    println!("Number of trials: {trials}");
}
