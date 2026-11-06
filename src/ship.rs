
pub struct Ship {
    // Will add a level or a type of the ship as well,
    // which will determinate some of the characteristics of the enemy ship.
    // Such as shield and hull_health values.

    pub name: String,
    pub hull_health: u32,
    pub shields: u32,
    pub alive: bool,
    
    // Will put weapons in a vec or something like that.
    // They are in their own file and have their own stats.
    pub weapons: String,

}

pub fn calculate_damage_hull() {

}