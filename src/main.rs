
mod modules;
fn main() {
    let run:i32 = 8;
    if run ==1{
        modules::learn_rust::run();
    }
    else if run ==2{
        modules::temprature::run();
    }
    else if run ==3 {
        modules::shipping_box::run();
    }
    else if run ==4{
        modules::vector::run();
    }
    else if run == 5{
        modules::vector_excecies::run();
    }
    else if run == 6{
        modules::string::run();
    }
    else if run == 7{
        modules::drive::run();
    }else if run == 8{
        modules::advance_match::run();
    }
    
}
