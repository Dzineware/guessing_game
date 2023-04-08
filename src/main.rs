use std::io;

/*
to accept user input from the prompte we need the io standard library
Why do everything from scratch when Rust comes loaded with minimal well-tested libraries
not all of them need to be mentioned at the header
there default stds defined in the Rust standard library that are integrated automatically
in a process called prelude - a collection of names that are automatically brought into scope
io is not in the prelude so it is imported using the use statment

*/
//import the rand library, which we added from crates, the Rust registery
use rand::Rng;

//With the following we will bring an enum type with variants to compare values from the standard library
use std::cmp::Ordering;

fn main() {
    // Main is the entry point for any Rust program
    // use the println macro to display the message
    println!("This is a Guessing Game. Input only numbers.");
    println!("The game will keep rolling until your successfully guess the number.");
    // we called the thread_rng function to generate a random number between 1 and 100
    //local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // the following one is good for testing insead of running it over and over
    //println!("The secret number is : {secret_number}");

    //give the user multiple chances of guessing
    //without a way of limiting or stopping the iteration
    //we will be forced to go Ctrl + C to stop such iteration
    loop {
        // use the println macro, again to prompte the user to input a value
        println!("Please Input your guess here:");

        // create a variable to store user input
        // consiering that user inpits change over time we have to make sure that the variable is mutable using mut keyword
        // by default variables are immutable in Rust, cant be changed
        // overriding that feature is quite easy with mut
        let mut guess = String::new();
        //guess is a variable, now mutable, that stores string value
        //::new is an associated function of the type, String, in this case
        //it creates an empty sting- making new value
        //guess is now currently bound to a new, empty instance of String data type, UTF-8 growable format

        //stdin is function in the io module of the Rust standard library
        //handles input at the prompte
        //if hadn't placed the header statement on top, we could do it like as follows:
        //std::io::stdin()
        io::stdin()
            // the following method is called on the standard input handle, to take whatever is typed by the user
            //it knows where to store it as well a reference (indicated by &) of a mutable variable, guess, is passed to it
            //The reference approach is used to save us from making multiple copies of the same value in a single variable
            //multiple parts of this program now have access to the memory address - reference - of the variable guess
            .read_line(&mut guess)
            // the .method_name() syntax makes such lines of code readable
            //read_line has a return value -of enumeration type- enum for short - for multiple value storage
            //Result's variants - possible states - Ok and Err - their name is indicatively clear, right?
            //the Result variants have methods that are defined on them
            //in this case an instance of Result has a method called expect, which you can call to see which variant is being returned
            .expect("Failed to read line");
        // we no longer trust the compiler to infer the type of the input
        //because we are about to compare numeric values
        //input type will be converted from string to integer by the following line
        //shadowing the previously created guess variable, no need for another unique variable
        //and it has to be cleaned using trim for and converted to an int type using parse
        //It was originally inputted as a string, remember
        //the variable is annotated
        let guess: u32 = match guess.trim().parse() {
            //first arm of Result from parse()
            Ok(num) => num,
            //second arm of Result from parse()
            Err(_) => continue,
        };

        // println!("You Guessed: {guess}"); one is a macro that spits outputs
        // the curly bracket is a placeholder
        //the little crab pincer
        //serves to hold value in space
        //in this case we are outputing a result of an expression so no need to place variable name - guess
        println!("You Guessed: {guess}");
        //here the comp method is called to compare the value of guess with the generated random number
        //match expression is used to make a choice
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You Guessed a Smaller Number."),
            Ordering::Greater => println!("You Guessed a Bigger Number."),
            Ordering::Equal => {
                println!("You Got it! You Guessed the Correct Number.");
                break;
                //we can break out of this loop when the end user wins, that is one way to go about it
            }
        }
    }
}
