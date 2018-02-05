extern crate rand;

use std::io;
use rand::Rng;

fn main() {

    println!("\n** Russian Roulette **\n");

    // number of chambers in the revolver
    const CHAMBERS_TOTAL: i32 = 6;

    // figure out which chamber is loaded
    let chamber_loaded = rand::thread_rng().gen_range(0, CHAMBERS_TOTAL);

    // we always start at chamber 0
    let mut chamber_next = 0;

    // decide who goes first
    let mut player_next = rand::thread_rng().gen_range(0, 2);

    println!("We sit on opposite sides of the table, with the unloaded gun between us.");

    loop {

        println!("\nRound {}:", chamber_next + 1);

        // load the gun only once
        if chamber_next == 0 {
            if player_next == 0 {
                println!("I place one bullet in the revolver and spin the chamber. I snap the chamber into place...");
            } else {
                println!("You place one bullet in the revolver and spin the chamber. You snap the chamber into place...");
            }
        }

        // each player picks up the gun
        if player_next == 0 {
            println!("I put the gun to my head...");
            println!("...And pull the trigger... ");
        } else {
            println!("You put the gun to your head...");
            println!("Press the [RETURN] key to pull the trigger.");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
        }

        // the game ends when the loaded chamber is fired
        if chamber_next == chamber_loaded {
            if player_next == 0 {
                println!("The gun fires and my brains are sprayed all over the wall. It is a grisly scene.");
                println!("You WIN!");
                break;
            } else {
                println!("The gun fires and YOUR brains are sprayed all over the wall. It is a grisly scene.");
                println!("You LOSE!");
                break;
            }
        } else {
            println!("We hear a dry click, there was no round in the chamber.");
        }

        // switch to the next player
        if player_next == 0 {
            player_next = 1;
        } else {
            player_next = 0;
        }

        // increment the chamber
        chamber_next += 1;
    }
}
