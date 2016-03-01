extern crate rand;
mod deck;
use rand::Rng;
use std::io;

fn main(){
    let mut deck = deck::deck();
    let d = deck::Card::new("ace",1);
    let mut discard = vec![d];

    draw(&deck, &discard);
}

fn draw(deck: &Vec<deck::Card>, discard: &Vec<deck::Card>) -> deck::Card{
    let rand = rand::thread_rng().gen_range(1, deck.len());
    println!("{} {} {}", rand, deck[rand].get_face(), deck[rand].get_suit());

    let card = deck[rand].clone();
    deck.swap_remove(rand);
    return card

}
