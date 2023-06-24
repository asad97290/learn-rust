enum Color{
    Red,
    Blue
}
impl Color {
    fn print(&self){
        match self {
            Color::Red => println!("color red"),
            Color::Blue => println!("color blue"),
        }
    }
}
struct Dimension{
    width:f64,
    height:f64,
    depth:f64,
}
impl Dimension{
 fn print(&self){
    println!("width: {}",self.width);
    println!("height: {}",self.height);
    println!("depth: {}",self.depth);
 }   
}
struct Box {
    dimensions:Dimension,
    color:Color,
    weight:f64
}

impl Box{
    fn new(
        dimensions:Dimension,
        weight:f64,
        color:Color
    )->Self{
        Self {dimensions,weight,color}
    }
    fn print_box(&self){
       
        println!("weight: {}",self.weight);
        self.dimensions.print();
        self.color.print();
    }
}


pub fn run(){
    let box_dimentions:Dimension = Dimension { width: 23.5, height: 55.7, depth: 11.2 };
    let shipping_box1:Box = Box::new(box_dimentions,44.6,Color::Blue);
    shipping_box1.print_box();
}