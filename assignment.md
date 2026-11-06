Overview

Design and implement a small text-based game that runs in the terminal. This is the final assignment of the course: it combines everything the course has covered into one larger program.

You have freedom in the theme, the world, and the exact rules. The constraints below are about structure and Rust features.

This assignment describes a dungeon crawler -type game, but you are allowed to come up with your own idea as long as you think the scale of the project is similar and your submission fullfills the requirements.
Project Structure

Create a Cargo project. Your project must use modules in separate source files, do not put everything in main.rs. A suggested layout is:

dungeon-crawler/
├── Cargo.toml
└── src/
├── main.rs        // game loop, ties everything together
├── game.rs        // game state
├── player.rs      // player struct
├── world.rs       // rooms / "world map"
├── combat.rs      // combat rules
└── ui.rs          // terminal input and output

You may choose different module names and split things differently, as long as the responsibilities stay clear and there are at least three modules in separate files besides main.rs.
Requirements

    Modules: The project must be organized into modules declared with mod and placed in separate files (see demos/rust-0604-separate-files). At least three modules besides main.rs.

    Structs with methods: Model the game's core entities (player, enemy, item, room, ...) as structs with impl blocks and methods (demos/rust-0401, rust-0402). Use &self, &mut self, and associated functions (new) where they fit.

    Enums and match: Use at least one enum with data-carrying variants for something that has several distinct states or kinds, for example a Command, an Action, an enemy kind, or an item type (demos/rust-0501, rust-0502). Handle it with an exhaustive match.

    Collections: Use Vec and/or HashMap for the game's data - the world, the inventory, the enemies (demos/rust-0701..0703).

    Terminal interaction: The game is played by reading commands from the terminal and printing text. Reading input was shown in the demos (demos/rust-0201-variables, demos/rust-0203-control-flow) and in assignments/02_calculator.md. You may reuse that pattern:

    use std::io;

    fn read_command() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    Error handling: Invalid user input (unknown command, invalid number) must be handled gracefully with match / if let on parse() results and the game must not crash on bad input. If your game reads or writes any files (optional), use Result and handle errors with match or ? (demos/rust-0802, assignments/07_dungeon_supply_chest).

    Game loop with a clear end: The game runs in a loop until the player wins, loses, or explicitly quits. Print the game state (location, health, inventory) before each command.

    Ownership: Try not to clone() everything everything away from the borrow checker. Use references and borrowing sensibly (demos/rust-0301..0303).

By doing this assignment you should naturally fullfill these constraints without having to think of these too much. But in the end do check that your submission tries to fullfill these at least for the most part. These are the grading criteria.
Ideas if you need a starting point

    A couple rooms connected in a map. You could store these in a HashMap<String, Room>
    Commands like go north, look, take sword.
    A player with health, an inventory (Vec<String> or HashMap<String, u32>), and simple stats.
    Enemies with health and attack; turn-based combat with attack / defend / use potion commands.
    Items that can be picked up and used.
    A win condition (e.g. defeat the boss / reach the exit) and a lose condition (health reaches 0).

Libraries

Third-party crates are allowed but not required. The standard library is enough. If you add a crate (e.g. for random numbers or colored output), add it with cargo add and keep the game playable with cargo run.
Scope

    One player, terminal only, no graphics (unless you feel adventurous).
    The world can be small (a handful of rooms and enemies), but it must be complete with playable game loop.
    The game must compile with cargo build and run with cargo run without crashing on normal or invalid input.
