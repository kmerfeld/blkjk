#[derive(Clone, Debug)]
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
        Card::new("spade", 10),
        Card::new("spade", 10),
        Card::new("spade", 10),
        Card::new("diamond", 1),
        Card::new("diamond", 2),
        Card::new("diamond", 3),
        Card::new("diamond", 4),
        Card::new("diamond", 5),
        Card::new("diamond", 6),
        Card::new("diamond", 7),
        Card::new("diamond", 8),
        Card::new("diamond", 9),
        Card::new("diamond", 10),
        Card::new("diamond", 10),
        Card::new("diamond", 10),
        Card::new("diamond", 10),
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
        Card::new("heart", 10),
        Card::new("heart", 10),
        Card::new("heart", 10),
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
        Card::new("club", 10),
        Card::new("club", 10),
        Card::new("club", 10),
        ];
    return deck
}
