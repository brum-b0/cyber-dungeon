mod game;

use game::{commands::process_command, player::Player, world::World};
use std::io::{self, Write};

fn main() {
    // Initialize the game world
    let mut world = World::new();
    world.init_rooms(); 
    
    //init the player
    let mut player = Player::new(0); // Start in the first room, but you can start wherever you want I guess
    let mut player_name = String::new();

    // this is more for my dungeon theme, can be whatever
    print!("Username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.trim().to_string();//fix broken prompt

    // this is more for my dungeon theme, can be whatever you want or none of it
    println!("User {} logged in successfully.", player_name);
    println!("Welcome to the Cyber Dungeon!");
    println!("Type 'look' to see your surroundings, 'go <direction>' to move, or 'inventory' to check your items.");

    //main control loop: read input -> process command -> respond appropriately. see process_command fn for details.
    loop {

        print!("{}@C-D > ", player_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //For Cli version only. not sure how this would work in wasm build
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