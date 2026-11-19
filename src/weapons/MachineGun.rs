use crate::weapons::{Damage, Weapon};

pub struct MachineGun {
    pub ammo: u32,
    pub hull_damage: u32,
    pub shield_damage: u32,
}

impl Weapon for MachineGun {
    fn ammo_count(&self) -> u32 {
        self.ammo
    }

    fn fire(&mut self) -> Option<Damage> {
        if self.ammo == 0 {
            return None
        }
        self.ammo -= 20;
        Some(Damage {hull_damage: self.hull_damage, shield_damage: self.shield_damage})

    }

    fn hull_damage(&self) -> u32 {
        self.hull_damage
    }

    fn shield_damage(&self) -> u32 {
        self.shield_damage
    }
}