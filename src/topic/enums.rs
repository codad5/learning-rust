pub enum Direction {
    Up, 
    Down, 
    Left, 
    Right
}

pub fn whats_is_player_direction(players_direction : Direction) 
{
    match players_direction {
        Direction::Up => println!("The player is moving up"),
        Direction::Down => println!("The player is moving up"),
        Direction::Left | Direction::Right => todo!()
    }
}