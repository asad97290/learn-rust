fn apply(num: i32) -> String {
    if 2 > num {
        return "hello".to_string();
    } else {
        return "bye".to_string();
    }
}
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

struct Human {
    name: String,
    age: i32,
    height: f32,
    is_male: bool,
    blood_group: char,
}

enum Color {
    Red,
    Green,
    Blue,
}
fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn disply_human_info(human: &Human) {
    println!("{}", human.name.to_string());
    println!("{}", human.age);
    println!("{}", human.height);
    println!("{}", human.is_male);
    println!("{}", human.blood_group);
}

pub fn run() {
    let name: String = apply(1);
    let name1: &str = "!";
    let height: f32 = 6.7;
    let mut age: i32 = 25;

    let is_male: bool = true;
    println!("{name}{name1} world");
    println!("height: {}", height);
    println!("is male: {}", is_male);
    loop {
        if age > 28 {
            break;
        }
        println!("you are {age} year old");
        age += 1;
    }

    let (one, two, three) = one_two_three();
    println!("{one},{},{}", two, three);

    println!("");
    print_color(Color::Blue);
    print_color(Color::Red);
    print_color(Color::Green);

    let mut i: i32 = 4;
    while i < 8 {
        i += 1;
        if i == 6 {
            continue;
        }
        println!("{i}");
    }

    let num = 7;
    let message = match num {
        7 => true,
        _ => false,
    };
    println!("{message}");

    let is_num_seven = num == 7;
    println!("{is_num_seven}");

    for j in 1..8 {
        println!("{j}");
    }

    let human = Human {
        name: "asad".to_string(),
        age: 25,
        height: 6.7,
        is_male: true,
        blood_group: 'A',
    };
    disply_human_info(&human);
    disply_human_info(&human);
}
