struct Temperature {
    degrees: f64,
}
impl Temperature {
    fn show_temp(&self) {
        println!("{}", self.degrees);
    }
    fn create_temp(temp:f64) -> Self {
        Self { degrees:temp}
    }
}
pub fn run() {
    let hot_temp = Temperature { degrees: 9.99 };
    hot_temp.show_temp();
    let freezing = Temperature::create_temp(32.1);
    freezing.show_temp();
    
}
