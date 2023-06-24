

fn print(name:&str){
    println!("{:?}",name)
}
pub fn run(){
    let name_str= "ASadullah khan";
    let name_str1= "ASadullah khan".to_owned();
    let name_str2= String::from("ASadullah khan");
    print(name_str);
    print(&name_str1);
    print(&name_str2);
}