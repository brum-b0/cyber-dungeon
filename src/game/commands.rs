use super::{player::Player, world::World};

pub fn process_command(command: &str, player: &mut Player, world: &mut World) -> String {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    if parts.is_empty() {
        return "I don't understand that command.".to_string();
    }

    match parts[0] {
        "go" => {
            if parts.len() < 2 {
                "Go where?".to_string()
            } else {
                match player.move_room(parts[1], world) {
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
            let current_room = &world.rooms[player.current_room];
            format!(
                "{}\nItems here: {:?}\nExits: {:?}",
                current_room.description,
                current_room
                    .items
                    .iter()
                    .map(|i| &i.name)
                    .collect::<Vec<_>>(),
                current_room.exits.keys().collect::<Vec<_>>()
            )
        }
        "inventory" => {
            let items = player.inventory.iter().map(|i| &i.name).collect::<Vec<_>>();
            if items.is_empty() {
                "You are not carrying anything.".to_string()
            } else {
                format!("You are carrying: {:?}", items)
            }
        }
        "take" => {
            if parts.len() < 2 {
                "Take what?".to_string()
            } else {
                let item_name = parts[1];

                // Check if the item exists in the current room
                if let Some(room_item) = world.rooms[player.current_room]
                    .items
                    .iter_mut()
                    .find(|i| i.name == item_name)
                {
                    if room_item.can_take {
                        // If the item can be taken, take it from the room and add it to the player's inventory
                        let item = room_item.clone(); // Clone it to own the item
                        player.take_item(item);
                        // Remove the item from the room
                        world.rooms[player.current_room]
                            .items
                            .retain(|i| i.name != item_name);
                        format!("You picked up the {}.", item_name)
                    } else {
                        "That item cannot be taken.".to_string()
                    }
                } else {
                    "There's no such item here.".to_string()
                }
            }
        }
        _ => "Unknown command.".to_string(),
    }
}
