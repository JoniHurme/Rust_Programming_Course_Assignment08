use crate::ship::Ship;

pub fn create_enemy_ship() -> Ship {


    Ship {
        name: String::from("Hostile ship 1"),
        hull_health: 100,
        shields: 50,
        weapons: String::from("Missiles"),
        alive: true,
    }
}