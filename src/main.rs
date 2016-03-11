extern crate rand;
mod deck;

use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main(){
    let mut  win: i32 = 0;
    let mut lose: i32 = 0;

    loop {

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

        println!("#######drawing two cards#######");
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
            total = 0;
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
            else {
                continu = false;
            }

            total = get_total(hand);
            println!("\nyour total is  {}\n",total);
        }


        let tot: i32 = get_total(house_hand);
        let mut tot_h: i32 = get_total(house_hand);

        while  tot_h < 17 {
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
            println!("You win!!!!\n\n");
            win +=1;
        }
        else {
            println!("You lost\n\n");
            lose +=1;
        }

        println!("you have won {} times", win);
        println!("you have lost {} times", lose);

        println!("Press enter to continue");
        
         let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("failed to read line");


            clear_screen()
    }
}
fn draw(deck: &mut Vec<deck::Card>) -> deck::Card{
    let rand = rand::thread_rng().gen_range(1, deck.len());
    return deck.swap_remove(rand);
}

fn get_total(deck: &Vec<deck::Card>) -> i32 {
    let mut totl = 0;
    let mut aces = Vec::new();
    for i in 0..deck.len() {
        if deck[i].get_face() == 1{
            aces.push(deck[i].clone());
        }
        else {
            totl += deck[i].get_face();
            println!("{}", deck[i].get_face());
        }
    }
    for _ in 0..aces.len() {
        if totl + 11 < 22 {
            totl += 11;
        }
        else {
            totl += 1;
        }

    }
    return totl;
}
fn clear_screen() {
    for _ in 0..80 {
        println!("\n");
    }
}
