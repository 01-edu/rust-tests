// Write a program that prints a riddle, receives input from the user
// and checks that the answer is correct

// The program must allow indefinite number of trials and only quit after the
// correct answer is given

// Every time the user introduces an incorrect answer the program must
// print the riddle
// Before quitting the program must print the number of trials

// Riddle: I am the beginning of the end, and the end of time and
// space. I am essential to creation, and I surround every place. What
// am I?

// Answer: The letter e

use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut counter = 0;

    let trials = loop {
        println!("{}", riddle);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        counter += 1;

        if answer.to_owned() + "\n" == input {
            break counter;
        }
    };

    println!("Number of trials: {}", trials);
}
