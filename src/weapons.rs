// This file holds all the weapon related things. These include types and stats.

pub(crate) mod MachineGun;
pub(crate) mod PlasmaCannon;
mod Missiles;

// The weapon is a trait which gets implemented on the different weapon types.
pub trait Weapon {
    fn ammo_count(&self) -> u32;
    
    fn fire(&mut self) -> Option<Damage>;

    fn hull_damage(&self) -> u32;
    fn shield_damage(&self) -> u32;
}

pub struct Damage {

    pub hull_damage: u32,
    pub shield_damage: u32,
}
