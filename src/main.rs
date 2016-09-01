extern crate rand;
mod deck;

use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main(){

    clear_screen();
    //initialize score values
    let mut  win: i32 = 0;
    let mut lose: i32 = 0;


    //game loop
    loop {
        //initialize hands and the deck
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
        println!("\nthe house is draws its first card");
        println!("{} {}", house_hand[0].get_face(), house_hand[0].get_suit());


        //option to draw more cards
        let mut total = 0;
        let mut continu = true;

        while continu == true && total < 22{
            //reset value so you dont add cards multiple times
            total = 0;

            println!("\nDo you want another card? (1 = yes, 2 = no)");
            //input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("failed to read line");


            clear_screen();
            //parse input into u32
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
                println!("\nYour cards are ");
            }

            total = get_total(hand);
            println!("\nyour total is  {}\n",total);
        }


        //scoreing section
        let mut tot_h: i32 = get_total(house_hand);

        //draw cards untill the house has at least 17
        while  tot_h < 17 {
            house_hand.push(draw(deck));
            println!("the house draws a {} {} ",house_hand[house_hand.len() -1].get_face(), house_hand[house_hand.len() -1].get_suit());

            tot_h += house_hand[house_hand.len() -1].get_face();
        }

        println!(" you have {}", total);
        println!(" the house has  {}", tot_h);


        //Decide the winner
        if total == tot_h {
            println!("Draw!!");
        }
        else if total >= tot_h && total <=21 || total < 22 && tot_h > 21 {
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
