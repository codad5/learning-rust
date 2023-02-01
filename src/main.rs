enum Direction {
    Up, 
    Down, 
    Left, 
    Right
}
//using constant 
const  MAX_NUM : u32 = 300;
fn main() {
    // Trying a single comment
    /*
    This is a double line comment 
    you can see this 
     */   
    println!("Hello, Codad5!");
    // using variable 
    let mut x = 45;
    println!("The value of x is {}", x);
    x = 60;
    println!("The value of x is {}", x);
    
    // Learning data type

    let _f :i32 = 45; // i32 ( 32 bit integer)
    let _z : i64 = 89; // i64 ( 64 bit integer ( they contain larger number than i32))
    let _j : f32 = 7.6; // f32 (A float data type )
    let _y : u32 = 46; // u32 ( This means the varibale not support negative values)
    let _b : bool = false; // a boolean


    // conditional statement 
    //if statement
    // >, <, ==, !=
    if x > 40 {
        println!("X is greater than 40")
    }
    else if x == 40 {
        println!("X is equal to 40")
    }
    else {
        println!("it was not greater than 40")
    }

    //loops 
    //infinite loop
    let mut _p = 0;
    loop {
        _p += 1;
        if _p == 27 {
            continue;
        }
        if _p > 50 {
            break;
        }
    println!("The number of p is {}", _p);

    }
    while _p < 60 {
        println!("p is lesser than 60");
        _p += 1;
    }


    // creating a range in rust
    let range = 2..MAX_NUM; // a range of integer from 2-20
    //  For loop
    for i in range {
        println!("value of i is {}" ,i)
    }

    // a vector in rust 
    let cars = vec!["volvo", "tesla", "honda", "mercedes"];
    // looping though a vector
    // i have to use the iter method so i can use the values  benath
    //  a simple for-loop on vector 
    // for a in cars.iter(){
    //     println!("{}",a);
    // }
    for (index, a) in cars.iter().enumerate() {
        println!("car type {} has index of {}", a, index)
    }
    // with out the iter method attached to the cars vector in the for-loop above  i wont be able to use the varaible under
    println!("{}", cars[0]);


    // using enums 
    let players_direction:Direction = Direction::Up;
    match players_direction {
        Direction::Up => println!("The player is moving up"),
        Direction::Down => println!("The player is moving up"),
        Direction::Left | Direction::Right => todo!()
    }
    let test = callme();
}

fn callme() ->  u128{
    println!("I am running call me");
    return 43 as u128
}