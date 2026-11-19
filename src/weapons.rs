// This file holds all the weapon related things. These include types and stats.

// The weapon is a trait which gets implemented on the different weapon types.
pub trait Weapon {
    fn ammo_count(&self) -> u32;
    
    fn fire(&mut self) -> bool;

}
// Weapon types and their structs
pub struct MachineGun {
    pub ammo: u32,
    pub hull_damage: u32,
    pub shield_damage: u32,
}

pub struct PlasmaCannon {
    pub ammo: u32,
    pub hull_damage: u32,
    pub shield_damage: u32,
}

pub struct Missiles {
    pub ammo: u32,
    pub hull_damage: u32,
    pub shield_damage: u32,
}

// Weapon impl blocks for the different weapon types.
impl Weapon for PlasmaCannon {
    fn ammo_count(&self) -> u32 {
        todo!()
    }

    fn fire(&mut self) -> bool{
        todo!()
    }
}

impl Weapon for MachineGun {
    fn ammo_count(&self) -> u32 {
        self.ammo
    }
    fn fire(&mut self) -> bool {

        self.ammo -= 20;
        true

    }
}

impl Weapon for Missiles {
    fn ammo_count(&self) -> u32 {
        todo!()
    }

    fn fire(&mut self) -> bool {
        todo!()
    }
}