extern crate rand;
mod deck;

use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main(){
    let mut d = deck::deck();
    let deck = &mut d; 
    let mut h =  Vec::new();
    let mut hh = Vec::new();
    let hand = &mut h;
    let house_hand = &mut hh;

    //Draw initial cards
    hand.push(draw(deck));
    hand.push(draw(deck));
    house_hand.push(draw(deck));
    house_hand.push(draw(deck));

    println!("drawing two cards");
    for i in 0..2{
        println!("{} {}", hand[i].get_face(), hand[i].get_suit());
    }
    println!("\nthe house is drawing two cards");
    for i in 0..2{
        println!("{} {}", house_hand[i].get_face(), house_hand[i].get_suit());
    }


    //option to draw more cards
    let mut total = 0;
    let mut continu = true;
    while continu == true && total < 22{
        println!("\nDo you want another card? (1 = yes, 2 = no)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");

        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if input == 1 {
            let card = draw(deck);
            println!("\nyou drew {} {}", card.get_face(), card.get_suit());
            hand.push(card);
        }
        if input == 1 {
            continu = false;
        }
        for i in 0..hand.len() { 
            let t = hand[i].get_face();

            total += t;

        }
        println!("\nyour total is  {}\n",total);
    }


    let mut tot: i32 = 0;
    let mut tot_h: i32 = 0;

    for i in 0..hand.len() {
        tot += hand[i].get_face();
    }

    for i in 0..house_hand.len() {
        tot_h += house_hand[i].get_face();
    }
    while tot_h <= tot || tot_h <= 17 {
        house_hand.push(draw(deck));
        println!("the house draws a {} {} ",house_hand[house_hand.len() -1].get_face(), house_hand[house_hand.len() -1].get_suit());

        tot_h += house_hand[house_hand.len() -1].get_face();
    }
    println!(" you have {}", tot);
    println!(" the house has  {}", tot_h);


    //Decide the winner
    if tot == tot_h {
        println!("Draw!!");
    }
    else if tot >= tot_h && tot <=21 || tot < 22 && tot_h > 21 {
        println!("You win!!!!");
    }
    else {
        println!("You lost");
    }


}
fn draw(deck: &mut Vec<deck::Card>) -> deck::Card{
    let rand = rand::thread_rng().gen_range(1, deck.len());
    return deck.swap_remove(rand);
}
