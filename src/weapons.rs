// This file holds all the weapon related things. These include types and stats.

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