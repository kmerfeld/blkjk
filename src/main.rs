extern crate rand;
mod deck;

use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main(){
    let mut deck = deck::deck();
    let mut hand =  Vec::new();
    let mut house_hand = Vec::new();
    hand.push(draw(&deck));
    hand.push(draw(&deck));
    house_hand.push(draw(&deck));
    house_hand.push(draw(&deck));

    println!("drawing two cards");
    for i in 0..2{
        println!("{} {}", hand[i].get_face(), hand[i].get_suit());
    }
    println!("the house is drawing two cards");
    for i in 0..2{
        println!("{} {}", house_hand[i].get_face(), house_hand[i].get_suit());
    }

    let mut total = 0;
    let mut continu = true;
    while continu && total < 22{
        println!("Do you want another card? (1/0)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");

        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if input == 1 {
            let card = draw(&deck);
            println!("you drew {} {}", card.get_face(), card.get_suit());
            hand.push(card);
        }
        if input == 0 {
            //todo ai
            continu = false;
        }
for i in 0..hand.len() { let t = hand[i].get_face();

            total += t;

        }
        println!("your total is  {}",total);
    }


    let mut tot: i32 = 0;
    let mut tot_h: i32 = 0;

    for i in 0..hand.len() {
        tot += hand[i].get_face();
    }

    for i in 0..house_hand.len() {
        tot_h += house_hand[i].get_face();
    }
    while tot_h <= tot && tot_h <= 17 {
        house_hand.push(draw(&deck));
        println!("the house draws a {} {} ",house_hand[house_hand.len() -1].get_face(), house_hand[house_hand.len() -1].get_suit());

        tot_h += house_hand[house_hand.len() -1].get_face();
    }
    println!(" you have {}", tot);
    println!(" the house has  {}", tot_h);


    //Decide the winner
    if tot > tot_h && tot <=21 {
        println!("You win!!!!");
    }
    else if tot <= 21 && tot_h > 21 {
        println!("You win!!!!");
    }
    else if tot == tot_h || tot > 21 && tot_h > 21 {
        println!("Draw!");
    }
    else {
        println!("You lost");
    }
        

}

fn draw(mut deck: &Vec<deck::Card>) -> deck::Card{
    let rand = rand::thread_rng().gen_range(1, deck.len());

    let card = deck[rand].clone();
    //deck.swap_remove(rand);
    return card

}
