use std::io;
use rand::Rng;

struct Character {
    name: String,
    level: u32,
    health: u32,
    max_health: u32,
    magic_points: u32,
    max_magic_points: u32,
}

impl Character {
    fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            level: 1,
            health: 100,
            max_health: 100,
            magic_points: 50,
            max_magic_points: 50,
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 20;
        self.health = self.max_health;
        self.max_magic_points += 10;
        self.magic_points = self.max_magic_points;
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

    fn take_damage(&mut self, damage: u32) {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
            println!("{} has been defeated!", self.name);
        }
    }
}

fn main() {
    println!("Welcome to the RPG Game!");

    println!("Enter your character's name:");
    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    let mut player = Character::new(player_name.trim());

    loop {
        println!("Options:");
        println!("1. Level Up");
        println!("2. Cast Spell");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                player.level_up();
                println!("{} leveled up!", player.name);
            }
            "2" => {
                player.cast_spell();
            }
            "3" => {
                println!("Exiting the game.");
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }

        let enemy_damage = rand::thread_rng().gen_range(10..=30);
        player.take_damage(enemy_damage);

        println!("{}'s Health: {}/{}", player.name, player.health, player.max_health);
        println!("{}'s Magic Points: {}/{}", player.name, player.magic_points, player.max_magic_points);
        println!("{}'s Level: {}", player.name, player.level);
    }
}
