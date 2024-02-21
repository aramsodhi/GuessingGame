// import libraries for reading input from the command line and generating random numbers
use std::io;
use rand::Rng;

fn main() {
    // print the title and rules of the game
    println!("Guess the number between 1 and 10!");

    // set the number of guesses the user has equal to 3
    let mut number_of_guesses = 3;

    // set the range for what the random number is set to
    let lower_range: i32 = 1;
    let upper_range: i32 = 10;

    // generate a random number between lower and upper ranges
    // add one to upper range because it is not inclusive
    let mut rng = rand::thread_rng();
    let answer = rng.gen_range(lower_range..upper_range);

    // main game loop
    // repeat while the user still has guesses
    while number_of_guesses > 0 {
        // tell the user what to input
        println!("Input your guess: ");

        // create a string to store the user's input
        let mut input_line = String::new();

        // read from the command line and store in input_line variable
        // print failed to read input if there is an error
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read input");

        // convert the string input to a 32-bit signed integer
        // tell the user if what they inputted is not an integer
        let guess: i32 = input_line.trim().parse().expect("Input is not a number.");

        // tell the user if their input is not in between the upper and lower ranges then break from loop and stop program
        if !((guess >= lower_range) & (guess <= upper_range)) {
            println!("Input is not within range");
            break;
        }

        // check if the user guessed correctly
        // if so, break the loop and stop program
        if guess == answer {
            println!("You guessed the number correctly!");
            break;

        // tell the user if their guess is too high and subtract one from guesses
        } else if guess > answer {
            number_of_guesses -= 1;
            println!("Too high. You have {number_of_guesses} guesses left.");

        // tell the user if their guess is too low and subtract one from guesses
        } else {
            number_of_guesses -= 1;
            println!("Too low. You have {number_of_guesses} guesses left.");
        }
    }
}
