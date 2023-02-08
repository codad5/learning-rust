enum Direction {
    Up, 
    Down, 
    Left, 
    Right
}
//using constant 
const  MAX_NUM : u32 = 300;
//  using struct 
struct Human {
    age : u32,
    gender : &str
}
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

    // printing a statement 
    // there are two way of prinitng out a dynamic statement 
    // one way is to use this print!("i am a {}", variable_name)
    // the other way is to use this print!("i am a {variable_name}"")
    let sum = sum(8, 4);
    println!("{sum}");


    //  about refrencing in variable 
    // for instance a variable job 
    let job = "cooking";
    let james_job = &job;
    println!("{james_job}");
    // but we can change james_job 
    // for instance the code in the next line will throw an errir
    // james_job = "driving";
    // to be able to change the value we can di 
    let mike_job = &mut job;
    // we can now do 
    // mike_job = "driving";
    // note the code above gives an error 
    // because we have to put an asterik(*) in front of the variable name 
    *mike_job = "driving";
    println!("{mike_job}");

}

// creating a new function 
fn  sum(num1 : i32, num2 : i32) -> i32 {
    // println!(num1+num2)
    return num1+num2;
}
