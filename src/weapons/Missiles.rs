use crate::weapons::{Damage, Weapon};

pub struct Missiles {
    pub ammo: u32,
    pub hull_damage: u32,
    pub shield_damage: u32,
}

impl Weapon for Missiles {
    fn ammo_count(&self) -> u32 {
        todo!()
    }


    fn fire(&mut self) -> Option<Damage> {
        todo!()
    }

    fn hull_damage(&self) -> u32 {
        todo!()
    }

    fn shield_damage(&self) -> u32 {
        todo!()
    }
}