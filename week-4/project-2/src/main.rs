use std::io;

fn main() {
    // --- Step 1: Ask if experienced using 1 for Yes, 0 for No ---
    println!("Is employee experienced? (Enter 1 for Yes, 0 for No):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experienced: i32 = input1.trim().parse().expect("Not a number");

    // --- Step 2: Read employee age ---
    println!("Enter employee age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age: i32 = input2.trim().parse().expect("Not a number");

    // --- Step 3: Check criteria using an else if ladder ---
    if experienced == 1 && age >= 40 {
        println!("Incentive is N1,560,000");
    } else if experienced == 1 && age >= 30 && age <= 39 {
        println!("Incentive is N1,480,000");
    } else if experienced == 1 && age < 28 {
        println!("Incentive is N1,300,000");
    } else if experienced == 1 {
        // Covers ages 28 and 29 if experienced
        println!("Incentive is N1,300,000");
    } else {
        // Covers anyone who is NOT experienced (experienced == 0)
        println!("Incentive is N100,000");
    }
}