// THIS IS AN INFINITE LOOP USUAGE
pub fn print_even_number_from(from : i32, end : i32)
{
    let mut num = from;
    loop {
        // this break statement is added to avoid inifinte loop
        if num >= end { break; }
        if num % 2 == 0  {
            println!("{}", num);
        }
        num+=1;
    }
}

// WHILE LOOP USUAGE
pub fn print_odd_num_lesser_than(num: usize)
{
    let mut count = 0;
    while count < num {
        if count % 2 != 0 {
            println!("{}", count);
        }
        count+=1;
    }
}

// FOR LOOP USUAGE
pub fn count_words(words : &Vec<&str>) -> Vec<usize> {
    let mut word_count_array : Vec<usize> = Vec::new();
    // for (index, word) in words.iter().enumerate() {} this can also be used if index of array is neeeded 
    for word in words.iter() {
        word_count_array.push(word.chars().count());
    }
    return word_count_array
}

// FOR LOOP FOR PRINTING RANGE OF NUMBERS
pub fn print_range(min: usize, max: usize)
{
    for i in min..max {
        println!("{}", i)
    }
}