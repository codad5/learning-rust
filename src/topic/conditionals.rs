pub fn is_a_multiple_of(numerator : i32, denominator : i32) -> bool 
{
    let remainder = numerator % denominator;
    if remainder == 0 { true } else { false}
}

pub fn is_even(number: i32) -> bool
{
    return number % 2 == 0
}