use crate::ship::Ship;

pub fn create_player() -> Ship {

    Ship {
        name: String::from("Player Ship"),
        hull_health: 200,
        shields: 150,
        weapons: String::from("Missiles"),
        alive: true
    }

}