mod topic;

//using constant 
const  MAX_NUM : u32 = 300;
//  using struct 

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

    // creating range in rust
    let range  = 2..30;


    


    // printing a statement 
    // there are two way of prinitng out a dynamic statement 
    // one way is to use this print!("i am a {}", variable_name)
    // the other way is to use this print!("i am a {variable_name}"")
    let sum = sum(8, 4);
    println!("{sum}");


    //  about refrencing in variable 
    // for instance a variable job 
    let mut job = "cooking";
    let james_job = &job;
    println!("{james_job}");
    // but we can change james_job 
    // for instance the code in the next line will throw an errir
    // james_job = "driving";
    // to be able to change the value we can d0
    let mike_job = &mut job;
    // we can now do 
    // mike_job = "driving";
    // note the code above gives an error 
    // because we have to put an asterik(*) in front of the variable name 
    *mike_job = "driving";
    println!("{mike_job}");

    let human = topic::using_structs::human::Human { name : "Chibueze".to_owned(), age : 19, gender : "Male" };

    human.say_name();

}

// creating a new function 
fn  sum(num1 : i32, num2 : i32) -> i32 {
    // println!(num1+num2)
    return num1+num2;
}
