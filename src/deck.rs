use std::io;
use std::io::prelude::*;


pub struct Card {
    s: String,
    n: i32,

}

impl Card {

    pub fn new(s: &str, n: i32) -> Card {
        Card {
            s: s.to_string(),
            n: n,
        }
    }
    pub fn get_suit(&self) -> &String {
        &self.s
    }
    pub fn get_face(&self) -> i32 {
        self.n
    }
     //pub fn clone(&self) -> Self {
       // Card {
         //   n: self.n,
     //       s: self.s,
     //   }
    //}
}
impl Clone for Card {
    fn clone(&self) -> Self { *self }
}

pub fn deck() -> Vec<Card>{

    let deck = vec![
        Card::new("spade", 1),
        Card::new("spade", 2),
        Card::new("spade", 3),
        Card::new("spade", 4),
        Card::new("spade", 5),
        Card::new("spade", 6),
        Card::new("spade", 7),
        Card::new("spade", 8),
        Card::new("spade", 9),
        Card::new("spade", 10),
        Card::new("spade", 11),
        Card::new("spade", 12),
        Card::new("spade", 13),
        Card::new("ace", 1),
        Card::new("ace", 2),
        Card::new("ace", 3),
        Card::new("ace", 4),
        Card::new("ace", 5),
        Card::new("ace", 6),
        Card::new("ace", 7),
        Card::new("ace", 8),
        Card::new("ace", 9),
        Card::new("ace", 10),
        Card::new("ace", 11),
        Card::new("ace", 12),
        Card::new("ace", 13),
        Card::new("heart", 1),
        Card::new("heart", 2),
        Card::new("heart", 3),
        Card::new("heart", 4),
        Card::new("heart", 5),
        Card::new("heart", 6),
        Card::new("heart", 7),
        Card::new("heart", 8),
        Card::new("heart", 9),
        Card::new("heart", 10),
        Card::new("heart", 11),
        Card::new("heart", 12),
        Card::new("heart", 13),
        Card::new("club", 1),
        Card::new("club", 2),
        Card::new("club", 3),
        Card::new("club", 4),
        Card::new("club", 5),
        Card::new("club", 6),
        Card::new("club", 7),
        Card::new("club", 8),
        Card::new("club", 9),
        Card::new("club", 10),
        Card::new("club", 11),
        Card::new("club", 12),
        Card::new("club", 13),
        ];
    return deck
}
