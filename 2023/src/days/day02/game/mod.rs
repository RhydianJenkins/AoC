#[derive(Debug)]
pub struct Game {
    id: u8,
    max_num_red: u8,
    max_num_green: u8,
    max_num_blue: u8,
}

impl Game {
    pub fn new(id: u8, max_num_red: u8, max_num_green: u8, max_num_blue: u8) -> Game {
        Game {
            id,
            max_num_red,
            max_num_green,
            max_num_blue,
        }
    }
}
