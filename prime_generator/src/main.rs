use num::integer::Roots;
use rand::Rng;
use std::io;

fn main() {
    println!("Type 'help' for help");
    loop {
        let mut select = String::new();

        let options = [
            "generator".to_string(),
            "detector".to_string(),
            "random".to_string(),
            "factors".to_string(),
        ];

        for i in 1..options.len() + 1 {
            // prints options allowing more inputs to easily be introduced
            println!("[{}]: {}", i, options[i - 1])
        }
        
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");
        
        if select.trim() == "help" {
            println!("Input the number left of the function to activate it\nPress 0 to quit the program");
        }
        
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match select {
            // matches the given select value to a function else
            0 => std::process::exit(0),
            1 => prime_gen(),
            2 => prime_detct(),
            3 => prime_rand(),
            4 => prime_fact(),
            _ => println!("Not an option"),
            //Note!: if the number input is too large for select it will fail to print "Not an option"
            // I would like to find a solution to this without increasing the value of select or making it unsigned
            // Note! too bad idc anymore its now an i32 lol
        }
    }
}

fn prime_gen() {
    // simply reads inputs

    let mut input = String::new();
    let mut input2 = String::new();

    println!("Enter a range minimum");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    println!("Enter a range limit");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_gen();
            return;
        }
    };
    let input2: u128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_gen();
            return;
        }
    };


    loop {
        // prime_gen loop 1

        for i in input2..input + 1 {
            // prime_gen loop 2

            let input = i; //not sure why this is here but I trust myself

            let mut prime = true;

            for i in 2..input.sqrt() + 1 {
                // prime_gen loop 3

                if input % i == 0 {
                    prime = false;
                    break;
                }
            }

            if prime == true {
                println!("{input} is prime");
            }
      
        }
        println!("Done!");
        break;
    }
}

fn prime_detct() {
    let mut prime = true;
    let mut input = String::new();

    println!("Do not enter anything larger than 2^128");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_detct();
            return;
        }
    };

    if input % 2 == 0 {
        println!("{input} is not prime and is divisible by 2");
        prime = false;
    }

    let mut isthree: u128 = 2;

    for i in 1..input.sqrt() + 1 {
        // runs through range of 1 through root input

        if isthree == 2 {
            isthree = 0;
            continue;
        } else {
            isthree += 1;
        }

        if input % (i*2)+1 == 0 {
            // checks if it is prime or not and breaks if true
            println!("{input} is not prime and is divisible by {i}");
            prime = false;
            break;
        }

    }

    if prime == true {
        println!("{input} is prime");
    }
    println!("Done!")
}

fn prime_rand() {
    // takes in all inputs
    let mut input = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut prime_tries: u128 = 0;

    println!("Enter range minimum");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let input2: u128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_rand();
            return;
        }
    };

    println!("Enter range limit");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_rand();
            return;
        }
    };

    if input2 > input {
        println!("Error: range minimum was greater than range limit");
        return;
    }

    println!("Enter try limit");
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read line");

    let input3: u128 = match input3.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_rand();
            return;
        }
    };

    loop {
        let mut prime = true;

        let prime_num = rand::thread_rng().gen_range(input2..=input); // generates random number within the given range

        for i in 2..prime_num.sqrt() + 1 {
            if prime_num % i == 0 {
                prime = false;
                break;
            }
        }

        if prime == true {
            if prime_tries == input3 {
                // limits the number of tries to input3
                break;
            } else {
                prime_tries += 1;
            }
            println!("{prime_num} is prime");
        }
    }
    println!("Done!");
}



fn prime_fact() { // ai will replace us all one day

    let mut input = String::new();

    println!("Enter positive interger");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            prime_fact();
            return;
        }
    };
    if input < 2 {
        println!("Error: 1 and 0 are not valid inputs");
        return;
    }
    let input_og = input;
    
    let mut factors: Vec<u128> = Vec::new();
    
    let mut i = 2;
    
    while input > 1 {
    
        while input % i == 0 {
            factors.push(i);
            input /= i;
        }
        i += 1;
    }
    if input_og == factors[0] {
        println!("{} is prime", input_og)
    } else {
        println!("{:?}", factors)
    }
}


/* My attempt at not being replaced by an ai

fn prime_fact() {
    let input = 20;
    let mut prime = true;
    let mut factors: Vec<u128> = Vec::new();
    loop {
        for j in factors {
            for i in 2..j {
                if j % i == 0 {                    
                    prime = false;
                    factors.push(i);
                    factors.push(j/i);
                    break;
                }
            }
        }
        println!("{:?}", factors);
    }
} */