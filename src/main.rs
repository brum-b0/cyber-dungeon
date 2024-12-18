mod game;

use game::{commands::process_command, player::Player, world::World};
use std::io::{self, Write};

fn main() {
    let mut world = World::new();
    world.init_rooms(); // Initialize the game world

    let mut player = Player::new(0); // Start in the first room
    let mut player_name = String::new();

    //fix broken prompt
    print!("Username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.trim().to_string();

    println!("User {} logged in successfully.", player_name);
    println!("Welcome to the Cyber Dungeon!");
    println!("Type 'look' to see your surroundings, 'go <direction>' to move, or 'inventory' to check your items.");

    loop {
        print!("{}@C-D > ", player_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //For Cli version only 
        if input.trim().eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            println!("User {} logged out successfully.", player_name);
            break;
        }
         

        let response = process_command(&input, &mut player, &mut world);
        println!();
        println!("{}", response);
        println!();
    }
}