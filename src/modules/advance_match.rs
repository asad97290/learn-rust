enum Ticket{
    Backstage{name:String,price:f64},
    Vip{name:String,price:f64},
    Standard{price:f64}
}
pub fn run(){
    let tickets:Vec<Ticket> = vec![
        Ticket::Backstage { name: "asad".to_owned(), price:23.4  },
        Ticket::Vip { name: "saif".to_owned(), price:20.4  },
        Ticket::Standard { price:15.4  }
        ];


        for i in tickets{
            match i {
                Ticket::Backstage{name,price} => println!("{} {}",name,price),
                Ticket::Vip{name,price} =>println!("{} {}",name,price),
                Ticket::Standard{price} =>println!("{}",price)
            }
        }
}


