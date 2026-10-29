use std::io;

struct Laptops{
   brand:String,
   quantity:u32,
   price:u32
}
impl Laptops{
    fn calculate_price(&self)->u32{
        self.quantity as u32 *self.price
    }
}


fn main() {
    let mut laptops = vec![
    Laptops{brand: String::from("hp"),quantity: 3, price:650000},
    Laptops{brand: String::from("ibm"), quantity: 3, price:755000},
    Laptops{brand: String::from("toshiba"),quantity:3, price:555000},
    Laptops{brand: String::from("dell"),quantity:3,price:850000},

    ];

    for laptop in &mut laptops {
        println!("enter quantity {} (3 only):",laptop.brand);

        let mut read_1 = String::new();
        io::stdin().read_line(&mut read_1).expect("Failed to read");

        let mut quantity: u32 = read_1.trim().parse().expect("Failed to input");

        while quantity <3 {
            println!("buy at least 3 laptops per brand");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            quantity = input.trim().parse().expect("Failed to read");
        }

        laptop.quantity = quantity;
    }
    let mut total = 0;
    println!("\ntotal purchase");

    for laptop in &laptops{
        let amount = laptop.calculate_price();

        println!("brand {}\n",laptop.brand);
        println!("quantity {}\n",laptop.quantity);
        println!("unit price {:.2}",laptop.price);
        println!("total cost {:.2}\n",amount );

        total += amount;

    }
    println!("total amount spent {:.2}",total );
}
