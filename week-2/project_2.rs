fn main() {
     //toshiba variables here
	let toshiba_qty:f64 = 2.0;
	let toshiba_amt:f64 = 450_000.00;
	let toshiba_total = toshiba_qty*toshiba_amt;
      
    //mac quantities here  
	let mac_qty:f64 = 1.0;
	let mac_amt:f64 = 1_500_000.0;
	let mac_total = mac_qty*mac_amt;

	//hp quantities here
	let hp_qty:f64 = 3.0;
	let hp_amt:f64 = 750_000.0;
	let hp_total = hp_qty*hp_amt;

	//dell quantities here
	let dell_qty:f64 = 3.0;
	let dell_amt:f64 = 2_850_000.0;
	let dell_total = dell_qty*dell_amt;

	//acer quantites here 
	let acer_qty:f64 = 1.0;
	let acer_amt:f64 = 250_000.0;
	let acer_total = acer_qty*acer_amt;

	//sum of sales
	let sum = toshiba_total + mac_total + hp_total + dell_total + acer_total;
	println!("Sum of sales: {}",sum);

	//average of sales
	let average = sum/5.0;
	println!("Average of sales: {}",average);
}