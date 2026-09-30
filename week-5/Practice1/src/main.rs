fn main(){
    let name = "aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "km 52 lekki-Epe Expressway, Ibeju-lekki, lagos";
    println!("Name:{}",name);
    println!("University:{},\nAddreess;{}",uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department:{},\nschool:{}",department,school);
}
