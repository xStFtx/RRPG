use std::io;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Clone, PartialEq)]
enum ItemRarity {
    Common,
    Rare,
    Epic,
}

#[derive(Clone, PartialEq)]
enum ItemType {
    HealthPotion,
    MagicPotion,
    DamageBoost,
}

#[derive(Clone, PartialEq)]
enum ItemEffect {
    Heal(u32),
    RestoreMagicPoints(u32),
    DamageIncrease(u32),
}

struct Item {
    name: String,
    rarity: ItemRarity,
    item_type: ItemType,
    effect: ItemEffect,
}

struct Character {
    name: String,
    level: u32,
    health: u32,
    max_health: u32,
    magic_points: u32,
    max_magic_points: u32,
    experience_points: u32,
    experience_to_level_up: u32,
    class: CharacterClass,
    inventory: Vec<Item>,
}

struct QuestObjective {
    description: String,
    is_completed: bool,
}

struct Quest {
    name: String,
    objectives: Vec<QuestObjective>,
    rewards: Vec<Item>,
    completed: bool,
}

struct Enemy {
    name: String,
    health: u32,
    damage: u32,
}

impl Character {
    fn new(name: &str, class: CharacterClass) -> Self {
        let (max_health, max_magic_points) = match class {
            CharacterClass::Warrior => (120, 40),
            CharacterClass::Mage => (80, 80),
            CharacterClass::Rogue => (100, 60),
        };

        Character {
            name: name.to_string(),
            level: 1,
            health: max_health,
            max_health,
            magic_points: max_magic_points,
            max_magic_points,
            experience_points: 0,
            experience_to_level_up: 100, // Adjust as needed
            class,
            inventory: vec![],
        }
    }

    fn cast_spell(&mut self) {
        if self.magic_points >= 10 {
            let damage = rand::thread_rng().gen_range(10..=20);
            println!("{} casts a spell and deals {} damage!", self.name, damage);
            self.magic_points -= 10;
        } else {
            println!("Not enough magic points!");
        }
    }

    fn attack(&self) -> u32 {
        match self.class {
            CharacterClass::Warrior => rand::thread_rng().gen_range(10..=20),
            CharacterClass::Mage => rand::thread_rng().gen_range(15..=25),
            CharacterClass::Rogue => rand::thread_rng().gen_range(12..=22),
        }
    }

    fn take_damage(&mut self, damage: u32) {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
            println!("{} has been defeated!", self.name);
        }
    }

    fn use_item(&mut self, item: &Item) {
        match &item.effect {
            ItemEffect::Heal(heal_amount) => {
                self.health += heal_amount;
                if self.health > self.max_health {
                    self.health = self.max_health;
                }
            }
            ItemEffect::RestoreMagicPoints(mp_amount) => {
                self.magic_points += mp_amount;
                if self.magic_points > self.max_magic_points {
                    self.magic_points = self.max_magic_points;
                }
            }
            ItemEffect::DamageIncrease(damage_increase) => {
                // Implement logic to increase character's damage here.
            }
        }
        // Remove the used item from the inventory.
        self.inventory.retain(|item_in_inventory| item_in_inventory != item);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.item_type == other.item_type && self.effect == other.effect
    }
}

impl Enemy {
    fn attack(&self) -> u32 {
        self.damage
    }

    fn take_damage(&mut self, damage: u32) {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
            println!("{} has been defeated!", self.name);
        }
    }
}

fn fight(player: &mut Character, enemy: &mut Enemy) {
    let player_damage = player.attack();
    let enemy_damage = enemy.attack();

    println!(
        "{} attacks {} for {} damage!",
        player.name, enemy.name, player_damage
    );
    enemy.take_damage(player_damage);

    if enemy.health == 0 {
        println!("{} defeats the {}!", player.name, enemy.name);
        return;
    }

    println!(
        "{} attacks {} for {} damage!",
        enemy.name, player.name, enemy_damage
    );
    player.take_damage(enemy_damage);

    if player.health == 0 {
        println!("{} has been defeated.", player.name);
    }
}

fn battle(player: &mut Character) {
    let mut enemies: Vec<Enemy> = vec![
        Enemy {
            name: "Orc".to_string(),
            health: 80,
            damage: 10,
        },
        Enemy {
            name: "Dragon".to_string(),
            health: 150,
            damage: 20,
        },
    ];

    println!("A battle begins!");

    while let Some(enemy) = enemies.first_mut() {
        println!(
            "{} encounters a {} with {} health!",
            player.name, enemy.name, enemy.health
        );

        fight(player, enemy);

        if enemy.health == 0 {
            enemies.remove(0);
        }

        if player.health == 0 {
            println!("{} has been defeated.", player.name);
            return;
        }
    }

    println!("The battle is over!");
}

fn main() {
    println!("Welcome to the Advanced RPG Game!");

    println!("Enter your character's name:");
    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    let mut player = Character::new(player_name.trim(), CharacterClass::Warrior);

    loop {
        println!("Options:");
        println!("1. Cast Spell");
        println!("2. Use Item");
        println!("3. Battle");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                player.cast_spell();
            }
            "2" => {
                println!("Inventory:");
                for (index, item) in player.inventory.iter().enumerate() {
                    println!("{}. {}", index + 1, item.name);
                }
                println!("Enter the number of the item to use:");
                let mut item_choice = String::new();
                io::stdin()
                    .read_line(&mut item_choice)
                    .expect("Failed to read line");
                if let Ok(item_index) = item_choice.trim().parse::<usize>() {
                    if item_index > 0 && item_index <= player.inventory.len() {
                        // Clone the item before borrowing the player mutably
                        let chosen_item = player.inventory[item_index - 1].clone();
                        player.use_item(&chosen_item);
                        println!("{} uses {}.", player.name, chosen_item.name);
                    } else {
                        println!("Invalid item choice.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }
            "3" => {
                battle(&mut player);
            }
            "4" => {
                println!("Goodbye!");
                return;
            }
            _ => {
                println!("Invalid choice.");
            }
        }

        println!(
            "{}'s Health: {}/{}",
            player.name, player.health, player.max_health
        );
        println!(
            "{}'s Magic Points: {}/{}",
            player.name, player.magic_points, player.max_magic_points
        );
        println!("{}'s Level: {}", player.name, player.level);
    }
}


impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            name: self.name.clone(),
            rarity: self.rarity.clone(),
            item_type: self.item_type.clone(),
            effect: self.effect.clone(),
        }
    }
}