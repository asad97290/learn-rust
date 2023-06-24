pub fn run(){
    let my_vec = vec![10,20,30,40];
    for i in &my_vec {
        match i {
            30=>println!("thirty"),
            _=>println!("{}",i)
        }
    }
    println!("length:{}",my_vec.len());
}