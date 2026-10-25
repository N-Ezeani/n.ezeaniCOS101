use std::io;
use std::io::Write;


fn main(){
    let  mut file = std::fs::File::create("project2.csv").expect("create failed");
    let mut student_name : Vec<String> = Vec::new();
    let mut matric_number : Vec<String> = Vec::new();
    let mut department : Vec<String> = Vec::new();
    let mut level : Vec<u32> = Vec::new();
    
    println!("How many students are in the class");
    let mut no_students = String::new();
    io::stdin().read_line(&mut no_students).expect("Failed to read input");
    let no_students:usize = no_students.trim().parse().expect("input not an integer");

    file.write("Student Name, Matric.Number,Department,Level\n".as_bytes()).expect("write failed");

    for i in 0..no_students{
        
        let mut name = String::new();
        println!("Enter student {} Name",i+1);
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();
        student_name.push(name);

        let mut matric_no = String::new();
        println!("Enter student {} Matric Number",i+1);
        io::stdin().read_line(&mut matric_no).expect("Failed to read input");
        let matric_no = matric_no.trim().to_string();
        matric_number.push(matric_no);

        let mut dept = String::new();
        println!("Enter student {} department",i+1);
        io::stdin().read_line(&mut dept).expect("Failed to read input");
        let dept = dept.trim().to_string();
        department.push(dept);

        let mut lvl = String::new();
        println!("Enter student {} level",i+1);
        io::stdin().read_line(&mut lvl).expect("Failed to read input");
        let lvl = lvl.trim().parse().expect("Not a valid number");
        level.push(lvl);

        let table = format!("{},{},{},{}\n",student_name[i],matric_number[i],department[i],level[i]);

        file.write(table.as_bytes()).expect("write failed");


    }
    println!("File Written succesfully");

}