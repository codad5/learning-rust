// CREATING A STRUCT
pub struct Human<'a> {
    pub name: String,
    pub age : u32,
    pub gender : &'a str,
}


// ADDING METHODS TO STRUCT

impl<'a> Human<'a> {
    // PUBLIC METHODS
    //  YOU HAVE TO PASS IN SELF AS REFRENCE
    pub fn say_name(&self) {
        println!("my name is {}", self.name);
    }

    pub fn can_drink(&self) -> bool 
    {
        return self.age > 18
    }

    // PRIVATE METHODS 
    fn reset_age (&mut self)
    {
        self.age = 0;
    }
}