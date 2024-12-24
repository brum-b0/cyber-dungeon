use super::{
    player::{self, Player},
    world::World,
};

pub fn process_command(command: &str, player: &mut Player, world: &mut World) -> String {
    //figured it would work best like argv
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    if parts.is_empty() {
        return "I don't understand that command.".to_string();
    }
    //match on main command
    match parts[0] {
        "go" | "move" => {
            //moves rooms
            if parts.len() < 2 {
                //moving needs a direction
                "Go where?".to_string()
            } else {
                match player.move_room(parts[1], world) {
                    //if we get an Ok for a move, we make it happen
                    Ok(_) => {
                        let current_room = &world.rooms[player.current_room];
                        format!(
                            "You moved to: {}.\n{}",
                            current_room.name, current_room.description
                        )
                    }
                    Err(err) => err,
                }
            }
        }
        "look" => {
            //comment update test commit fix
            //shows room description and debug print room items and exits
            let current_room = &world.rooms[player.current_room];
            format!(
                "{}\n\
                Items here: {:?}\n\
                Exits: {:?}\n\
                NPCs: {:?}",
                current_room.description,
                current_room
                    .items
                    .iter()
                    .map(|i| &i.name) //we only need the names, not the entire item struct
                    .collect::<Vec<_>>(),
                current_room.exits.keys().collect::<Vec<_>>(),
                world
                    .npcs
                    .iter()
                    .filter(|i| i.current_room == player.current_room)
                    .map(|i| &i.name)
                    .collect::<Vec<_>>()
            )
        }
        "inventory" => {
            //grabs player inventory and shows it. we only need the item names displayed
            let items = player.inventory.iter().map(|i| &i.name).collect::<Vec<_>>();
            if items.is_empty() {
                "You are not carrying anything.".to_string()
            } else {
                format!("You are carrying: {:?}", items)
            }
        }
        "take" => {
            //takes item from the room, clones it to player inventory and removes item from room
            if parts.len() < 2 {
                //needs an item to take
                "Take what?".to_string()
            } else {
                let item_name = parts[1];

                // first, does this item exist in the room?
                if let Some(room_item) = world.rooms[player.current_room]
                    .items
                    .iter_mut()
                    .find(|i| i.name == item_name)
                {
                    if room_item.can_take {
                        // next, can the item be taken?
                        let item = room_item.clone(); // clone it
                        player.take_item(item); // to own it :)
                                                // Remove the item from the room's items
                        world.rooms[player.current_room]
                            .items
                            .retain(|i| i.name != item_name); //need to modify room items in place, rather than filter
                        format!("You picked up the {}.", item_name)
                    } else {
                        "That item cannot be taken.".to_string()
                    }
                } else {
                    "There's no such item here.".to_string()
                }
            }
        }
        "drop" => {
            if parts.len() < 2 {
                "Drop what?".to_string()
            } else {
                let item_name = parts[1];

                if let Some(drop_item) = player.inventory.iter_mut().find(|i| i.name == item_name) {
                    let item = drop_item.clone();
                    world.rooms[player.current_room].items.push(item);
                    player.remove_item(item_name);
                    format!("You dropped the {}.", item_name)
                } else {
                    "You don't have that.".to_string()
                }
            }
        }
        "talk" => {
            if parts.len() < 2 {
                "Talk to whom?".to_string()
            } else {
                let npc_name = parts[1];

                if parts[1] == "self" {
                    "Why are you talking to yourself?".to_string()
                } else if let Some(npc) = world
                    .npcs
                    .iter_mut() // we need mutable references here
                    .find(|i| i.name == npc_name)
                {
                    if npc.current_room == player.current_room {
                        npc.speak_dialogue()
                    } else {
                        "They aren't here.".to_string()
                    }
                } else {
                    "They aren't here".to_string()
                }
            }
        }
        "help" => {
            // end each line as \n\ for readability for both dev and user
            "this is the help page\n\
            you can view it like this and\n\
            it stays in place\n\
            test newline\
            "
            .to_string()
        }
        "eat" => {
            if parts.len() < 2 {
                "Eat what?".to_string()
            } else {
                let item_name = parts[1];

                if let Some(drop_item) = player.inventory.iter_mut().find(|i| i.name == item_name) {
                    if drop_item.can_eat == true {
                        player.health_points += drop_item.heal_amount;
                        player.remove_item(parts[1]);
                        format!("You ate the {}.", item_name)
                    } else {
                        "You can't eat that.".to_string()
                    }
                } else {
                    "You don't have that.".to_string()
                }
            }
        }
        //status command display hp, ap, and equipment
        "status" => {
            let gear = player.equipment.iter().map(|i| &i.name).collect::<Vec<_>>();
            format!(
                "Equipment: {:?}\n\
                HP: {}      AP: {}
            ", gear, player.health_points, player.attack_power)
        }
        //equip an item
        "equip" => {
            if parts.len() < 2 {
                "Equip what?".to_string()
            } else {
                
                let item_name = parts[1];
                //search equipment for item name
                if let Some(equip_item) = player.equipment.iter().find(|i| i.name == item_name) {
                    "You've already equipped that.".to_string()
                } else {
                    //find item in inventory
                    if let Some(equip_item) = player.inventory.iter().find(|i| i.name == item_name) {
                        let item = equip_item.clone();
                        player.attack_power += item.attack_increase_amount;
                        player.equip_item(item); 
                        format!("You equipped the {}.", item_name)

                    } else {
                        "You don't have that.".to_string()
                    }
                }
                
            }
        }
        //unequip an item
        "unequip" => {
            if parts.len() < 2 {
                "Unequip what?".to_string()
            } else {
                let item_name = parts[1];
                //search equipment for item name
                if let Some(equip_item) = player.equipment.iter().find(|i| i.name == item_name) {
                    player.attack_power -= equip_item.attack_increase_amount;
                    player.unequip_item(item_name);

                    format!("You unequipped the {}", item_name)
                } else {
                    "You don't have that equipped.".to_string()
                }

            }
        }
        //fight an npc -> if hostile jump into fight -> if not hostile ask if sure
            // fight creates a new combat loop

        _ => "Unknown command.".to_string(), //generic response to things we dont' recognize :)
    }
}
