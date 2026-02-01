use macroquad::rand;

#[derive(Clone, Debug, PartialEq)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArmorType {
    Leather,
    Chainmail,
    Platemail,
}

#[derive(Clone, Debug)]
pub enum Item {
    Weapon(WeaponType),
    Armor(ArmorType),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Weapon(w) => w.name(),
            Item::Armor(a) => a.name(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Item::Weapon(WeaponType::Sword) => "Damage: 1-10".to_string(),
            Item::Weapon(WeaponType::Axe) => "Damage: 5-8".to_string(),
            Item::Weapon(WeaponType::Mace) => "Damage: 7".to_string(),
            Item::Armor(ArmorType::Leather) => "Reduces damage by 1".to_string(),
            Item::Armor(ArmorType::Chainmail) => "Reduces damage by 2".to_string(),
            Item::Armor(ArmorType::Platemail) => "Reduces damage by 4".to_string(),
        }
    }

    pub fn random() -> Item {
        if rand::gen_range(0.0, 1.0) < 0.5 {
            // Weapon
            match rand::gen_range(0, 3) {
                0 => Item::Weapon(WeaponType::Sword),
                1 => Item::Weapon(WeaponType::Axe),
                _ => Item::Weapon(WeaponType::Mace),
            }
        } else {
            // Armor
            match rand::gen_range(0, 3) {
                0 => Item::Armor(ArmorType::Leather),
                1 => Item::Armor(ArmorType::Chainmail),
                _ => Item::Armor(ArmorType::Platemail),
            }
        }
    }
}

impl WeaponType {
    pub fn name(&self) -> &str {
        match self {
            WeaponType::Sword => "Sword",
            WeaponType::Axe => "Axe",
            WeaponType::Mace => "Mace",
        }
    }

    pub fn roll_damage(&self) -> i32 {
        match self {
            WeaponType::Sword => rand::gen_range(1, 11), // 1-10 inclusive
            WeaponType::Axe => rand::gen_range(5, 9),    // 5-8 inclusive
            WeaponType::Mace => 7,
        }
    }
}

impl ArmorType {
    pub fn name(&self) -> &str {
        match self {
            ArmorType::Leather => "Leather Armor",
            ArmorType::Chainmail => "Chainmail",
            ArmorType::Platemail => "Platemail",
        }
    }

    pub fn damage_reduction(&self) -> i32 {
        match self {
            ArmorType::Leather => 1,
            ArmorType::Chainmail => 2,
            ArmorType::Platemail => 4,
        }
    }
}

pub fn calculate_damage(base_damage: i32, armor: Option<&ArmorType>) -> i32 {
    let reduction = armor.map(|a| a.damage_reduction()).unwrap_or(0);
    // Minimum damage is always 1 - armor can never reduce damage to zero
    (base_damage - reduction).max(1)
}
