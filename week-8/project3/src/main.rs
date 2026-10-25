use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Merge.csv").expect("create failed");
    file.write_all("EFFC Convicted MInisters\n".as_bytes()).expect("Write failed");
    let header = "Commisioner,Ministry,Zone\n";
    file.write_all(header.as_bytes()).expect("Write Failed");
    let name_ppl = vec!("Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye");
    let ministry = vec!("Internal Affairs","Justice,Defence","Defense","Power & steel","Petroleum");
    let zone = vec!("South West","North East","South South","South West","South East");

    for i in 0..name_ppl.len(){
        let row = format!("{},{},{}\n",name_ppl[i],ministry[i],zone[i]);
        file.write_all(row.as_bytes()).expect("write failed");
    }
    println!("written succesfully");
}