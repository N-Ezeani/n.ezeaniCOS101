use std::io;

fn trapezium() -> f64 {
    let mut base1 = String::new();
    let mut base2 = String::new();
    let mut height = String::new();

    println!("Enter base 1:");
    io::stdin().read_line(&mut base1).expect("Failed");

    println!("Enter base 2:");
    io::stdin().read_line(&mut base2).expect("Failed");

    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed");

    let base1: f64 = base1.trim().parse().expect("Invalid");
    let base2: f64 = base2.trim().parse().expect("Invalid");
    let height: f64 = height.trim().parse().expect("Invalid");

    return height / 2.0 * (base1 + base2);
}

fn rhombus() -> f64 {
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut diagonal1).expect("Failed");

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut diagonal2).expect("Failed");

    let diagonal1: f64 = diagonal1.trim().parse().expect("Invalid");
    let diagonal2: f64 = diagonal2.trim().parse().expect("Invalid");

    return 0.5 * diagonal1 * diagonal2;
}

fn parallelogram() -> f64 {
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut base).expect("Failed");

    println!("Enter altitude:");
    io::stdin().read_line(&mut altitude).expect("Failed");

    let base: f64 = base.trim().parse().expect("Invalid");
    let altitude: f64 = altitude.trim().parse().expect("Invalid");

    return base * altitude;
}

fn cube() -> f64 {
    let mut side = String::new();

    println!("Enter side:");
    io::stdin().read_line(&mut side).expect("Failed");

    let side: f64 = side.trim().parse().expect("Invalid");

    return 6.0 * side * side;
}

fn cylinder() -> f64 {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut radius).expect("Failed");

    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed");

    let radius: f64 = radius.trim().parse().expect("Invalid");
    let height: f64 = height.trim().parse().expect("Invalid");

    let pi = 22.0 / 7.0;

    return pi * radius * radius * height;
}

fn main() {
    let mut choice = String::new();

    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");

    println!("Enter your choice:");

    io::stdin().read_line(&mut choice).expect("Failed");

    let choice: i32 = choice.trim().parse().expect("Invalid");

    if choice == 1 {
        println!("Answer = {}", trapezium());
    }
    else if choice == 2 {
        println!("Answer = {}", rhombus());
    }
    else if choice == 3 {
        println!("Answer = {}", parallelogram());
    }
    else if choice == 4 {
        println!("Answer = {}", cube());
    }
    else if choice == 5 {
        println!("Answer = {}", cylinder());
    }
    else {
        println!("Invalid Choice");
    }
}