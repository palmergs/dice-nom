//! Basic usage examples for the dice-nom library.
//!
//! This example demonstrates the core functionality of the dice-nom library,
//! including simple dice rolling, complex expressions, and various operators.

use dice_nom::{parse, roller};

fn main() {
    let mut rng = rand::rng();

    println!("=== Basic Dice Rolling ===");

    // Simple dice rolls using the roller function
    let simple_d6 = roller(1, 6, None);
    let result = simple_d6.generate(&mut rng);
    println!("1d6: {}", result);

    let multiple_dice = roller(3, 6, None);
    let result = multiple_dice.generate(&mut rng);
    println!("3d6: {}", result);

    println!("\n=== Parsing Complex Expressions ===");

    // Parse and roll various dice expressions
    let expressions = vec![
        "3d6+4",   // Basic arithmetic
        "2d8-1",   // Subtraction
        "1d20+5",  // Common D&D roll
        "4d6^3",   // Keep highest 3 of 4 dice
        "2d20ADV", // Advantage (roll twice, keep higher)
        "2d20DIS", // Disadvantage (roll twice, keep lower)
    ];

    for expr in expressions {
        match parse(expr) {
            Ok(generator) => {
                let result = generator.generate(&mut rng);
                println!("{}: {}", expr, result);
            }
            Err(_) => println!("Failed to parse: {}", expr),
        }
    }

    println!("\n=== Exploding Dice ===");

    // Exploding dice examples
    let exploding_expressions = vec![
        "3d6!",   // Explode on max (reroll once if all dice are 6)
        "2d8!!",  // Explode until (keep rerolling while all dice are max)
        "4d6*",   // Explode each (reroll each die that's max)
        "3d10**", // Explode each until (keep rerolling each max die)
    ];

    for expr in exploding_expressions {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", expr, result);
        }
    }

    println!("\n=== Target Numbers and Success Counting ===");

    // Target-based rolling
    let target_expressions = vec![
        "6d6[4]",     // Count dice >= 4 as successes
        "4d10(3)",    // Count dice <= 3 as successes
        "3d6{12}",    // Success if total >= 12
        "2d10{15,5}", // 1 success at 15, +1 for every 5 above
    ];

    for expr in target_expressions {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", expr, result);
        }
    }

    println!("\n=== Pool Manipulation ===");

    // Pool operations
    let pool_expressions = vec![
        "5d6^3", // Keep highest 3
        "5d6`2", // Keep lowest 2
        "6d6~4", // Keep middle 4
        "5d6Y",  // Keep largest group of matching dice
    ];

    for expr in pool_expressions {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!("{}: {}", expr, result);
        }
    }

    println!("\n=== Comparisons ===");

    // Comparison operations
    let comparison_expressions = vec![
        "3d6 > 2d8",    // Left vs right comparison
        "1d20+5 >= 15", // Skill check vs DC
        "2d6 = 7",      // Exact match
        "3d6 <=> 3d6",  // Three-way comparison (-1, 0, or 1)
    ];

    for expr in comparison_expressions {
        if let Ok(generator) = parse(expr) {
            let result = generator.generate(&mut rng);
            println!(
                "{}: {} ({})",
                expr,
                result,
                if result.sum() == 1 {
                    "Success"
                } else if result.sum() == -1 {
                    "Failure"
                } else if result.sum() == 0 {
                    "Tie/False"
                } else {
                    "Unknown"
                }
            );
        }
    }

    println!("\n=== Multiple Rolls ===");

    // Demonstrate multiple rolls of the same expression
    if let Ok(generator) = parse("3d6") {
        println!("Rolling 3d6 five times:");
        for i in 1..=5 {
            let result = generator.generate(&mut rng);
            println!("Roll {}: {}", i, result);
        }
    }

    println!("\n=== Error Handling ===");

    // Show error handling for invalid expressions
    let invalid_expressions = vec!["attack the goblin", "3d6 + invalid", "d", ""];

    for expr in invalid_expressions {
        match parse(expr) {
            Ok(_) => println!("{}: Parsed successfully (unexpected!)", expr),
            Err(remaining) => println!("{}: Failed to parse, remaining: '{}'", expr, remaining),
        }
    }
}
