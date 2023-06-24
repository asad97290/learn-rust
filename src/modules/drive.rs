
#[derive(Debug, Copy, Clone)]
enum Status{
    Todo,
    InProgress,
    Done(i32),
}



#[derive(Debug, Copy, Clone)]
struct Ticket{
    status:Status,
    story_points:u32
}

fn print(ticket:Ticket){
    println!("{:#?}",ticket);
    println!("{:#?}",ticket.status);
}
pub fn run(){
    let ticket = Ticket{
        status:Status::Done(12), 
        story_points:1, 
    };
    print(ticket);
    print(ticket);
    
}