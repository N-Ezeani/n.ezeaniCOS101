fn main() {
    
    let v = vec![10,20,30];

    let v2 = v;

    display(v2);

    //println!("In main {:?}",display(v2));
}
fn display(x:Vec<i32>){
    println!("inside display{:?}",x);
}
