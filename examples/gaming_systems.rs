//! Gaming system examples for the dice-nom library.
//!
//! This example demonstrates how to use dice-nom for various tabletop RPG systems,
//! showing real-world applications of the library's features.

use dice_nom::parse;

fn main() {
    let mut rng = rand::rng();

    println!("=== D&D 5th Edition Examples ===");

    // Character creation - roll stats
    println!("Rolling ability scores (4d6, drop lowest):");
    for ability in ["STR", "DEX", "CON", "INT", "WIS", "CHA"] {
        if let Ok(generator) = parse("4d6^3") {
            let result = generator.generate(&mut rng);
            println!("{}: {}", ability, result);
        }
    }

    // Combat rolls
    println!("\nCombat examples:");
    let combat_rolls = vec![
        ("Attack roll (d20+5)", "1d20+5"),
        ("Damage (longsword)", "1d8+3"),
        ("Sneak attack", "1d8+3+3d6"),
        ("Fireball damage", "8d6"),
        ("Healing potion", "2d4+2"),
    ];

    for (description, expr) in combat_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", description, result);
        }
    }

    // Advantage/Disadvantage
    println!("\nAdvantage and Disadvantage:");
    let adv_dis_rolls = vec![
        ("Attack with Advantage", "1d20ADV+5"),
        ("Stealth with Disadvantage", "1d20DIS+2"),
    ];

    for (description, expr) in adv_dis_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", description, result);
        }
    }

    println!("\n=== Call of Cthulhu  ===");

    // Character creation - roll stats
    println!("Generate statistics:");
    let character_stats = vec![
        ("STR (Strength)", "3d6x5"),
        ("CON (Constitution)", "3d6x5"),
        ("SIZ (Size)", "2d6+6x5"),
        ("DEX (Dexterity)", "3d6x5"),
        ("APP (Appearance)", "3d6x5"),
        ("INT (Intelligence)", "2d6+6x5"),
        ("POW (Power)", "3d6x5"),
        ("EDU (Education)", "2d6+6x5"),
        ("Luck", "3d6x5"),
    ];

    for (description, expr) in character_stats {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", description, result);
        }
    }

    println!("\n=== World of Darkness Examples ===");

    // Success counting system
    let wod_rolls = vec![
        ("Dexterity + Athletics (6 dice)", "6d10[6]"),
        ("Manipulation + Subterfuge (4 dice)", "4d10[6]"),
        ("Intelligence + Academics (7 dice)", "7d10[6]"),
    ];

    for (description, expr) in wod_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {} successes", description, result.lhs.hits());
        }
    }

    println!("\n=== Shadowrun Examples ===");

    // Edge cases and exploding dice
    let shadowrun_rolls = vec![
        ("Firearms skill test", "8d6[5]"),
        ("Edge-enhanced roll", "8d6**[5]"),
        ("Damage resistance", "6d6[5]"),
    ];

    for (description, expr) in shadowrun_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {} hits", description, result.lhs.hits());
        }
    }

    println!("\n=== GURPS Examples ===");

    // 3d6 roll-under system
    let gurps_rolls = vec![
        ("Skill check vs 12", "3d6 <= 12"),
        ("Attribute check vs 14", "3d6 <= 14"),
        ("Hard skill check vs 10", "3d6 <= 10"),
    ];

    for (description, expr) in gurps_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            let success = result.sum() == 1;
            println!(
                "{}: {} ({})",
                description,
                result.lhs.sum(),
                if success { "Success" } else { "Failure" }
            );
        }
    }

    println!("\n=== Fate/Fudge Examples ===");

    // Fudge dice simulation using d3 (1=-, 2=blank, 3=+)
    println!("Fate dice (simulated with d3s where 1=-, 2=0, 3=+):");
    for i in 1..=3 {
        if let Ok(generator) = parse("4d3--2") {
            let result = generator.generate(&mut rng);
            let fate_result = result.sum();
            println!(
                "Roll {}: {} ({})",
                i,
                result,
                match fate_result {
                    -4..=-3 => "Terrible",
                    -2..=-1 => "Poor",
                    0 => "Mediocre",
                    1..=2 => "Good",
                    3..=4 => "Great",
                    _ => "Exceptional",
                }
            );
        }
    }

    println!("\n=== Savage Worlds Examples ===");

    // Exploding dice and Aces
    let savage_worlds_rolls = vec![
        ("Fighting with d8", "1d8!"),
        ("Shooting with d10", "1d10!"),
        ("Wild die + trait", "1d6! + 1d8!"),
        ("Damage with d6+2", "1d6!+2"),
    ];

    for (description, expr) in savage_worlds_rolls {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            let total = result.sum();
            println!(
                "{}: {} ({})",
                description,
                result,
                if total >= 4 { "Success" } else { "Failure" }
            );
        }
    }
}
