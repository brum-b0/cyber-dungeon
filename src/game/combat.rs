use crate::game::{player::Player, world::{World, Npc}};
use std::io::{self, Write};

pub enum CombatAction {
    Attack,
    Eat,
    Skill,
    Retreat,
}

pub enum CombatState {
    PlayerTurn,
    NPCTurn,
    Defeat,
    Victory,
    Fled,
}

pub struct Combat {
    pub state: CombatState,
    pub npc_index: usize,
    pub turn_count: u32,
}

impl Combat {
    pub fn new(npc_index: usize) -> Self {
        Combat {
            state: CombatState::PlayerTurn,
            npc_index,
            turn_count: 0,
        }
    }

    pub fn start_combat(player: &mut Player, world: &mut World, npc_index: usize) -> String {
        let npc = &world.npcs[npc_index];
        println!("\n=== COMBAT INITIATED ===");
        println!("You are fighting: {}", npc.name);
        println!("Enemy HP: {} | Enemy AP: {}", npc.health_points, npc.attack_power);
        println!("Your HP: {} | Your AP: {}", player.health_points, player.attack_power);
        println!("========================\n");

        let mut combat = Combat::new(npc_index);
        combat.combat_loop(player, world)
    }

    fn combat_loop(&mut self, player: &mut Player, world: &mut World) -> String {
        loop {
            match self.state {
                CombatState::PlayerTurn => {
                    let action = self.get_player_action(player, world);
                    match action {
                        CombatAction::Attack => {
                            let damage = player.attack_power;
                            world.npcs[self.npc_index].health_points -= damage;
                            println!("You attack for {} damage!", damage);
                            
                            if world.npcs[self.npc_index].health_points <= 0 {
                                self.state = CombatState::Victory;
                                continue;
                            }
                        }
                        CombatAction::Eat => {
                            // See get_player_action
                        }
                        CombatAction::Retreat => {
                            self.state = CombatState::Fled;
                            continue;
                        }
                        CombatAction::Skill => {
                            println!("You don't know any skills yet!");// Need to add skills and experience later
                            continue; 
                        }
                    }
                    self.state = CombatState::NPCTurn;
                }
                CombatState::NPCTurn => {
                    let npc = &world.npcs[self.npc_index];
                    let damage = npc.attack_power;
                    player.health_points -= damage;
                    println!("{} attacks you for {} damage!", npc.name, damage);
                    
                    if player.health_points <= 0 {
                        self.state = CombatState::Defeat;
                        continue;
                    }
                    
                    self.state = CombatState::PlayerTurn;
                    self.turn_count += 1;
                }
                CombatState::Victory => {
                    println!("\n=== VICTORY! ===");
                    println!("You defeated the {}!", world.npcs[self.npc_index].name);
                    
                    // Remove defeated NPC from the world
                    world.npcs.remove(self.npc_index);
                    
                    return "You emerge victorious from combat!".to_string();
                }
                CombatState::Defeat => {
                    println!("\n=== DEFEAT ===");
                    println!("You have been defeated...");
                    return "GAME OVER - You died in combat!".to_string();
                }
                CombatState::Fled => {
                    println!("\n=== RETREAT ===");
                    println!("You successfully fled from combat!");
                    return "You escaped from the fight.".to_string();
                }
            }
            
            // Show status after each round
            let npc = &world.npcs[self.npc_index];
            println!("\n--- Status ---");
            println!("Your HP: {} | Enemy HP: {}", player.health_points, npc.health_points);
            println!("--------------\n");
        }
    }

    fn get_player_action(&self, player: &mut Player, world: &World) -> CombatAction {
        loop {
            print!("Choose action: [A]ttack, [E]at, [S]kill, [R]etreat > ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "a" | "attack" => return CombatAction::Attack,
                "e" | "eat" => {
                    if self.handle_eat_action(player) {
                        return CombatAction::Eat;
                    }
                    
                }
                "s" | "skill" => return CombatAction::Skill,
                "r" | "retreat" => return CombatAction::Retreat,
                _ => println!("Invalid action! Please choose A, E, S, or R."),
            }
        }
    }

    fn handle_eat_action(&self, player: &mut Player) -> bool {
        let edible_items: Vec<_> = player.inventory.iter()
            .filter(|item| item.can_eat)
            .collect();
        
        if edible_items.is_empty() {
            println!("You have no edible items!");
            return false;
        }
        
        println!("Edible items:");
        for (i, item) in edible_items.iter().enumerate() {
            println!("  {}: {} (heals {})", i + 1, item.name, item.heal_amount);
        }
        
        print!("Choose item to eat (number) or 'cancel': ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "cancel" {
            return false;
        }
        
        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= edible_items.len() {
                let item_name = edible_items[choice - 1].name.clone();
                let heal_amount = edible_items[choice - 1].heal_amount;
                
                player.health_points += heal_amount;
                player.remove_item(&item_name);
                
                println!("You ate {} and {} {} HP!", item_name, 
                    if heal_amount >= 0 { "gained" } else { "lost" }, 
                    heal_amount.abs());
                return true;
            }
        }
        
        println!("Invalid choice!");
        false
    }
}