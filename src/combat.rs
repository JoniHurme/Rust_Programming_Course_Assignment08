use crate::ship::Ship;
use crate::weapons::{Damage, Weapon};


pub fn attack(attacker: &mut Ship, defender: &mut Ship) {
    let weapon = attacker.weapons.get_mut(0).unwrap();

    match weapon.fire() {
        Some(damage) => {

            defender.hull_health = defender.hull_health - damage.hull_damage;
            defender.shields = defender.shields - damage.shield_damage;

        }
        None => {
            println!("Can not fire. Out of ammo.");
            return;
        }
    }

}