

pub fn run(){
    let arr1 = vec![1,2,3];
    println!("item at zero index of arr1:{}",arr1[0]);
    let mut arr = Vec::new();
    arr.push(1);
    arr.push(2);
    let len = arr.len();
    println!("arr length:{:?}",len);
    for y in arr{
    println!("arr item:{:?}",y)
    }
}