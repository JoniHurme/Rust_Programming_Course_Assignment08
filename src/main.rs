use ship::{EnemyKind};
use crate::ship::{create_enemy_ship};
use crate::weapons::{Weapon};

mod ship;
mod enemy;
mod weapons;
mod player;
mod sectors;
mod combat;

fn main() {


    println!("WELCOME TO SOLAR COMMANDER");




    let mut ship2 = create_enemy_ship(EnemyKind::Patrol);
    // println!("{}", ship2.name);
    let ship3 = create_enemy_ship(EnemyKind::HeavyPatrol);
    // print!("{}", ship3.hull_health);


    if let Some(a) = ship2.weapons.get_mut(0) {
        println!("{} ammo left", a.ammo_count());
    }


    println!("{}", ship3.hull_health);


    let damage = ship2.weapons.get_mut(0).unwrap().fire();

    println!("{}", damage);

    println!("{}", ship3.hull_health);

    if let Some(a) = &ship2.weapons.get(0) {
        println!("{} ammo left", a.ammo_count())
    }


}
