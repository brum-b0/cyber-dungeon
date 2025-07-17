use std::{collections::HashMap, hash::Hash};

use serde::de::value::UnitDeserializer;

pub struct World {
    pub rooms: Vec<Room>, // List of all rooms in the game world
    pub npcs: Vec<Npc>,   // list of all npcs in the game world
    pub flags: HashMap<String, bool>
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<Item>,              //items in room
    pub exits: HashMap<String, usize>, // Direction/string to world vec index map
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub can_take: bool, //some objects cannot/should not be picked up
    pub can_eat: bool,
    pub can_equip: bool,
    pub heal_amount: i32,
    pub attack_increase_amount: i32
    
}

#[derive(Debug, Clone)]
pub struct Npc {
    pub name: String,
    pub current_dialogue: Vec<String>, //current dialogue is a vec of string sentences
    pub current_room: usize,
    pub all_dialogues: HashMap<usize, Vec<String>>, //dialogue vector based on room index
    pub dialogue_counter: usize,                    // set this to zero
    pub health_points: i32, // basic hp
    pub attack_power: i32, // basic attack
    pub hostile: bool, // hostility state, monsters can be considered npcs as well as hostile npcs like in dark souls
}



impl Npc {
    pub fn speak_dialogue(&mut self) -> String {
        //if end of vec, return current index
        let resp = self.current_dialogue[self.dialogue_counter].clone();
        if self.dialogue_counter < self.current_dialogue.len() - 1 {
            self.dialogue_counter += 1;
        }
        resp.to_string()
    }

    pub fn update_dialogue(&mut self, room: usize) {
        self.current_dialogue = self.all_dialogues[&room].clone();
        self.dialogue_counter = 0;
    }

    pub fn move_to_room(&mut self, room: usize) {
        self.current_room = room;
        self.update_dialogue(room);
    }
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: Vec::new(),
            npcs: Vec::new(),
            flags: HashMap::new(),
        }
    }

    //event flag setter and getter
    pub fn set_flag(&mut self, key: &str, value: bool) {
        self.flags.insert(key.to_string(), value);
    }

    pub fn get_flag(&self, key: &str) -> bool {
        *self.flags.get(key)
            .unwrap_or(&false)
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

    pub fn create_npc(
        &mut self,
        name: &str,
        current_dialogue: Vec<String>,
        current_room: usize,
        all_dialogues: HashMap<usize, Vec<String>>,
        hp: i32,
        ap: i32,
        hostility: bool
    ) -> usize {
        let npc = Npc {
            name: name.to_string(),
            current_dialogue,
            current_room,
            all_dialogues,
            dialogue_counter: 0,
            health_points: hp,
            attack_power: ap,
            hostile: hostility

        };
        self.npcs.push(npc);
        self.npcs.len() - 1
    }

    /* INIT FN'S BELOW
     * ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
     * ==================================================================================================================
     */


    /* create all world npcs and place them in rooms accordingly.
     * Npcs, like players are not truly inside of a room, they are just have a current room they can look at.
     * Monsters, creatures and bosses are all Npcs.
     *
     */
    pub fn init_npcs(&mut self) {
        //init npc dialogue first
        let merlin_dialogues = HashMap::from([
            (
                1,
                vec![
                    "Greetings Traveler. I hope you found that key I left in the previous room.".to_string(),
                    "You will find that you may need it here or there.".to_string(),
                    "Good luck on your quest, perhaps we shall meet again soon.".to_string(),
                ]
            ),
            (
                3, //dialogue for room 3. make sure to move npc after skeleton is defeated
                vec![
                    "Hello again traveler.".to_string(),
                    "Thanks for getting rid of that skeleton in the other room.".to_string(),
                    "I must be going soon. Take care.".to_string()

                ]
            ),
        ]);

        //dialogue/quest npc
        self.create_npc(
            "Merlin",
            merlin_dialogues[&1].clone(),
            1,
            merlin_dialogues,
            20,
            20,
            false
        );
        self.create_npc("Skeleton", 
        Vec::new(), 
        2, 
        HashMap::new(), 
        50, 
        10, 
        true);
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
                can_eat: true,
                can_equip: false,
                heal_amount: -10,
                attack_increase_amount: 0
            }],
            HashMap::from([("north".to_string(), 1)]),
        );

        let room2 = self.create_room(
            "Maze Entrance",
            "You stand at the entrance of a dark maze. Exits lead in all directions.",
            vec![Item {
                name: "Broken_Sword".to_string(),
                description: "Not much use unless you're out of options.".to_string(),
                can_take: true,
                can_eat: false,
                can_equip: true,
                heal_amount: 0,
                attack_increase_amount: 10
                },
                Item {
                    name: "Healing_Grass".to_string(),
                    description: "Heals a small amount of health.".to_string(),
                    can_take: true,
                    can_eat: true,
                    can_equip: false,
                    heal_amount: 20,
                    attack_increase_amount: 0
                }
            ],
            HashMap::from([
                ("south".to_string(), room1), //either strategy here works
                ("north".to_string(), 2), // but I prefer direct index mapping using a drawing of a world map
                ("east".to_string(), 3),
                ("stairs".to_string(), 4),
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
