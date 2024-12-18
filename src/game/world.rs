use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<Item>, //items in room
    pub exits: HashMap<String, usize>, // Direction/string to world vec index map
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub can_take: bool,//some objects cannot/should not be picked up
}

pub struct World {
    pub rooms: Vec<Room>, // List of all rooms in the game world
}

impl World {
    pub fn new() -> Self {
        World { rooms: Vec::new() }
    }

    // Creates and adds a room to the world, returning its index so we can map adjacent rooms to the world room vec
    pub fn create_room(
        &mut self,
        name: &str,
        description: &str,
        items: Vec<Item>,
        exits: HashMap<String, usize>,
    ) -> usize {
        let room = Room {
            name: name.to_string(),
            description: description.to_string(),
            items,
            exits,
        };
        self.rooms.push(room);
        self.rooms.len() - 1 // Return the index of the newly created room
    }

    /*
     * Initializes all the rooms.
     * Indices must be planned ahead of time. It would help if you drew a map and numbered them.
     */
    pub fn init_rooms(&mut self) {
        // Example rooms for testing, all rooms for your game need to be placed in here.
        let room1 = self.create_room(
            "Starting Room",
            "You are in the starting room. There's an exit to the north.",
            vec![Item {
                name: "Key".to_string(),
                description: "A small rusty key.".to_string(),
                can_take: true,
            }],
            HashMap::from([("north".to_string(), 1)]),
        );

        let room2 = self.create_room(
            "Maze Entrance",
            "You stand at the entrance of a dark maze. Exits lead in all directions.",
            vec![],
            HashMap::from([
                ("south".to_string(), room1),
                ("north".to_string(), 2),
                ("east".to_string(), 3),
            ]),
        );

        self.create_room(
            "Maze Room 1",
            "The walls here are identical, and you feel disoriented.",
            vec![],
            HashMap::from([("south".to_string(), room2)]),
        );

        self.create_room(
            "Maze Room 2",
            "A narrow corridor with a faint breeze.",
            vec![],
            HashMap::from([("west".to_string(), room2)]),
        );
    }
}