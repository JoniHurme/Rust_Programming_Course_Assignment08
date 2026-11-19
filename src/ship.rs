use crate::weapons::{MachineGun, PlasmaCannon, Weapon};

pub struct Ship {
    // Will add a level or a type of the ship as well,
    // which will determinate some of the characteristics of the enemy ship.
    // Such as shield and hull_health values.

    pub name: String,
    pub hull_health: u32,
    pub shields: u32,
    pub alive: bool,
    
    // They are in their own file and have their own stats and behavior.
    // We need the Box and dyn keywords because the compiler does not know the size of these things at compile time, nor does it know what type the vec will ultimately have.
    pub weapons: Vec<Box<dyn Weapon>>,

}

impl Ship {
    pub fn new(name: &str, hull_health: u32, shields: u32 , weapons: Vec<Box<dyn Weapon>>) -> Ship {
        Self {
            name: name.to_string(),
            hull_health,
            shields,
            alive: true,
            weapons,
        }
    }

    // Function to create an enemy of the type Patrol
    pub fn enemy_patrol() -> Self {
        Self::new(
            "Enemy1",
            150, 50,
            vec![
                Box::new(MachineGun{
                    ammo: 50,
                    hull_damage: 50,
                    shield_damage: 25
                }),
                Box::new(
                    PlasmaCannon{
                        ammo: 50,
                        hull_damage: 25,
                        shield_damage: 50,
                    })
            ],
        )
    }

    // Function to create an enemy of the type Heavy Patrol
    pub fn enemy_heavy_patrol() -> Self {
        Self::new(
            "Enemy Heavy Patrol",
            200, 100,
            vec![
                Box::new(
                    PlasmaCannon{
                        ammo: 50,
                        hull_damage: 25,
                        shield_damage: 50
                    })
            ]
        )
    }
    

}
// Enum to hold the possible enemy variants. The enum is also used for the create_enemy_ship "factory"
pub enum EnemyKind {
    Patrol,
    HeavyPatrol,

}


// function to spawn enemy ships.
pub fn create_enemy_ship(kind: EnemyKind) -> Ship {
    match kind {
        EnemyKind::Patrol => Ship::enemy_patrol(),
        EnemyKind::HeavyPatrol => Ship::enemy_heavy_patrol(),
    }
}
