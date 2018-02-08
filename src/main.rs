extern crate rand;

use std::io;
use rand::Rng;

///
/// Provides a simple game of ["Russian Roulette"][0].
///
/// Bootstrapping function for the application, this initializes our variables
/// and starts the game.
///
/// [0]: https://en.wikipedia.org/wiki/Russian_roulette
///
fn main() {

    println!("\n** Russian Roulette **\n");

    // number of chambers in the revolver
    const CHAMBERS_TOTAL: i32 = 6;

    // figure out which chamber is loaded
    let chamber_loaded = rand::thread_rng().gen_range(0, CHAMBERS_TOTAL);

    // decide who goes first
    let mut player_next = rand::thread_rng().gen_range(0, 2);

    println!("We sit on opposite sides of the table, with the unloaded gun between us.\n");

    // loop through the chambers of the gun
    for chamber_next in 0..chamber_loaded + 1 {

        println!("Round {}:", chamber_next + 1);

        // load the gun only once
        if chamber_next == 0 {
            load_gun(player_next);
        }

        // each player picks up the gun and pulls the trigger
        pull_trigger(player_next);

        // the game ends when the loaded chamber is fired
        if chamber_next == chamber_loaded {
            shoot_player(player_next);
        } else {
            println!("We hear a dry click, there was no round in the chamber.\n");
        }

        // switch to the next player
        if player_next == 0 {
            player_next = 1;
        } else {
            player_next = 0;
        }
    }
}

///
/// Loads the gun for the provided player.
///
fn load_gun(player: i32) {

    if player == 0 {
        println!("I place one bullet in the revolver and spin the chamber. I snap the chamber into place...");
    } else {
        println!("You place one bullet in the revolver and spin the chamber. You snap the chamber into place...");
    }
}

///
/// Pulls the trigger for the provided player.
///
/// If the provided player is the human, they are prompted before the gun is
/// fired.
///
fn pull_trigger(player: i32) {

    if player == 0 {
        println!("I put the gun to my head...");
        println!("...And pull the trigger... ");
    } else {
        println!("You put the gun to your head...");
        println!("Press the [RETURN] key to pull the trigger.");
        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).expect("Failed to read line");
    }
}

///
/// Shoots the provided player.
///
fn shoot_player(player: i32) {

    if player == 0 {
        println!("The gun fires and my brains are sprayed all over the wall. It is a grisly scene.");
        println!("You WIN!");
    } else {
        println!("The gun fires and YOUR brains are sprayed all over the wall. It is a grisly scene.");
        println!("You LOSE!");
    }
}
