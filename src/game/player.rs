use crate::game::world::Item;
use crate::game::world::World;

#[derive(Debug, Clone)]
pub struct Player {
    pub current_room: usize,       
    pub inventory: Vec<Item>,     
}

impl Player {
    //player constructor (?)
    pub fn new(starting_room: usize) -> Self {
        Player {
            current_room: starting_room,
            inventory: Vec::new(),
        }
    }
    //takes an item and adds it to player inventory vec
    pub fn take_item(&mut self, item: Item) {
        self.inventory.push(item);
    }
    
    
    pub fn remove_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(index) = self.inventory.iter().position(|i| i.name == item_name) {
            Some(self.inventory.remove(index))
        } else {
            None
        }
    }

    // Move the player to an adjacent room if the direction/string key exists
    pub fn move_room(&mut self, direction: &str, world: &World) -> Result<(), String> {
        let current_room = &world.rooms[self.current_room];
        if let Some(&next_room_index) = current_room.exits.get(direction) {
            self.current_room = next_room_index;
            Ok(())
        } else {
            Err("You can't go that way.".to_string())
        }
    }
}