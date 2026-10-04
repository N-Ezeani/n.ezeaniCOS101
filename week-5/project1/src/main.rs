use std::io;

fn main() {

    println!("******** FOOD MENU ********");
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried Rice & Chicken - N3000");
    println!("A - Amala & Ewedu Soup - N2500");
    println!("E - Eba & Egusi Soup - N2000");
    println!("W - White Rice & Stew - N2500");

    let mut food = String::new();

    println!("\nEnter Food Code:");

    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read input");

    let mut quantity = String::new();

    println!("Enter Quantity:");

    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity:i32 = quantity
        .trim()
        .parse()
        .expect("Invalid input");

    let mut price:i32 = 0;

    if food.trim() == "P" {
        price = 3200;
    }
    else if food.trim() == "F" {
        price = 3000;
    }
    else if food.trim() == "A" {
        price = 2500;
    }
    else if food.trim() == "E" {
        price = 2000;
    }
    else if food.trim() == "W" {
        price = 2500;
    }
    else {
        println!("Invalid Food Code");
    }

    let mut total:i32;

    total = price * quantity;

    println!("Total Cost = N{}", total);

    if total > 10000 {

        let discount = total * 5 / 100;

        println!("Discount = N{}", discount);

        total = total - discount;

    }

    println!("Final Amount = N{}", total);

}