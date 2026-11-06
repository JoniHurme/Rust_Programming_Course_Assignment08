use crate::enemy::create_enemy_ship;
use crate::player::create_player;

mod ship;
mod enemy;
mod weapons;
mod player;
mod sectors;

fn main() {

    println!("WELCOME TO SOLAR COMMANDER");

    let enemy1 = create_enemy_ship();
    println!("{}", enemy1.name);
    
    let player = create_player();
    println!("{}", player.name);


}
