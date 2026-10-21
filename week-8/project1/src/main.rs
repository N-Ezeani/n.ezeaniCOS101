use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("nigerian_brewies.csv").expect("Failed to create file");
    let header = "lager,stout,non-alcholic";
    file.write_all(header.as_bytes()).expect("couldn't create header");

   
    let row_1 = "\n33 export,legend,Maltina";
    file.write_all(row_1.as_bytes()).expect("couldn't create row_1");

    
    let row_2 = "\nDesperados,Turbo king,Amstel Malta";
    file.write_all(row_2.as_bytes()).expect("couldn't create row_2");

   
    let row_3 = "\nGoldberg,Williams,Malta Gold";
    file.write_all(row_3.as_bytes()).expect("couldn't create row_3");

    
    let row_4 = "\nGulder,,Fayrouz";
    file.write_all(row_4.as_bytes()).expect("couldn't create row_4");

    
    let row_5 = "\nHeineken,,";
    file.write_all(row_5.as_bytes()).expect("couldn't create row_5");

   
    let row_6 = "\nStar,,";
    file.write_all(row_6.as_bytes()).expect("couldn't create row_6");
}
