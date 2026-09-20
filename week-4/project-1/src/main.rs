use std::io;

fn main() {
    // --- Step 1: Read 'a' ---
    println!("Enter value for a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f32 = input_a.trim().parse().expect("Not a number");

    // --- Step 2: Read 'b' ---
    println!("Enter value for b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f32 = input_b.trim().parse().expect("Not a number");

    // --- Step 3: Read 'c' ---
    println!("Enter value for c:");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c: f32 = input_c.trim().parse().expect("Not a number");

    // --- Step 4: Calculate discriminant d = b*b - 4*a*c ---
    let d: f32 = (b * b) - (4.0 * a * c);

    // --- Step 5: Check conditions ---
    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("One root: {}", root);
    } else {
        println!("No real roots");
    }
}