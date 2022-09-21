// greed functions go here
use owo_colors::OwoColorize;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use std::io;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    score: i32,
    turn_count: i32,
    items: Items,
}

pub fn set_player(name: String) -> Player {
    Player {
        name,
        score: 0,
        turn_count: 0,
        items: Items::Empty,
    }
}

pub fn get_player_names() -> () {
    let mut p_string = String::new();
    io::stdin().read_line(&mut p_string).expect("cant read");
    let p_string = p_string.trim();
    // change p string into and i32 so we can see how many players to create
    let p_num: i32 = p_string.parse().unwrap();
    // create empty vec to hold players
    let mut pvec: Vec<Player> = Vec::new();

    // ⚠️TO DO⚠️
    // refactor this!
    // make it better. smarter. quicker.
    // find easiest laziest way to write it
    // change if loops to match statements

    // loop through every number in 1 - pnum
    let mut i = 1;
    loop {
        // prompt players to enter their names
        println!(
            "{}{}{}",
            ("PLAYER ").on_bright_green().bold(),
            (i).on_bright_green().bold(),
            (" ENTER YOUR NAME").bright_green()
        );
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name).expect("cant read");

        if new_name.contains("Bob") || new_name.contains("bob") {
            println!("{}", ("~ B A N A N A ~").yellow());
        } else if new_name.contains("Kevin") || new_name.contains("kevin") {
            println!("{}", ("~ B A N A N A ~").yellow());
        } else if new_name.contains("Stuart")
            || new_name.contains("stuart")
            || new_name.contains("Stu")
            || new_name.contains("stu")
        {
            println!("{}", ("~ B A N A N A ~").yellow());
        }

        // ❗main focus point
        // use set_player function with user input as the parameter
        let player: Player = set_player(new_name);
        // push each player in
        pvec.push(player);

        if i >= p_num {
            break;
        }
        i += 1;
    }
    // ⚠️⚠️⚠️
}

#[derive(Debug, Clone)]
pub enum Items {
    /// roll dice values of 1-10
    MegaDice,
    /// roll 3 dice
    TripleDice,
    /// leech points from another player
    LeechDice,
    /// subtract from everyone elses scores, you gain no points
    EvilDice,
    /// chance to double or quadruple roll
    EvenOddHighLow,
    /// trade scores with another player
    ScoreSwap,
    /// yoink an item from player
    Yoink,
    /// empty
    Empty,
}

pub fn dice_roll() -> i32 {
    let mut rng = thread_rng();
    let roll: i32 = rng.gen_range(1..7);
    roll
}

pub fn print_milady() -> () {
    println!("\n\n");
    println!("{}", ("         .*O@@@@@@@@@@#o*.").bold());
    println!("{}", ("      *#@@@@@@@@@@@@@@@@@@#o.").bold());
    println!("{}", ("     O@@@@@@@@@@@@@@@@@@@@@@#°").bold());
    println!("{}", ("    *@@@@@@@@@@@@@@@@@@@@@@@@*").bold());
    println!("{}", ("    *@@@@@@@@@@@@@@@@@@Oo.@@o").bold());
    println!("{}", ("    °#@@@@@@@o   .@@@@@   ##.").bold());
    println!("{}", ("    .O@@@@@@@#*  .@@@@@@O#@o").bold());
    println!("{}", ("      .oO@@@@@@@@@@@@@@@@@o").bold());
    println!("{}", ("           .*o#@@@@@@@@#*.").bold());
    println!("{}", ("            *O@@@@@@@@@@O°").bold());
    println!("{}", ("          .#@@@@@@@@@@@@@@*").bold());
    println!("{}", ("          o@@@@@@@@@@@@@@@@°").bold());
    println!("{}", ("         .#@@@@@@@@@@@@@@@@O").bold());
    println!("{}", ("         °@@@@@@@@@@@@@@@@@#°").bold());
}

pub fn print_instructions() -> () {
    println!(
        "\n\t{} {} {} {} {}",
        ("$").blink().green(),
        ("$").blink_fast().bright_green(),
        ("  G R E E D  ").bold().blink().bright_cyan(),
        ("$").blink_fast().bright_green(),
        ("$").blink().green()
    );
    println!("{}", ("\n\n\nHOW TO PLAY").dimmed().bold());
    println!(
        "{}",
        ("\nRoll the dice as many times as you want.").on_truecolor(23, 23, 23)
    );
    println!(
        "Once you choose to end your turn then, your turn score is added to your total score."
    );
    println!(
        "{}",
        ("If you roll a 1, your turn ends and no points are added to your total score.")
            .on_truecolor(23, 23, 23)
    );
    println!("If you roll doubles, those dice are doubled in value.");
    println!(
        "{}",
        ("If you roll SNAKE EYES, your turn ends and your total score is now 0.")
            .on_truecolor(23, 23, 23)
    );
    println!("{}", ("Win by reaching 100 points.").italic().yellow());
    println!("\n{}", ("All players get an equal amount of turns."));
    println!("{}", ("If player 2 reaches 100 in 4 turns, then all following players get one last turn to get a higher score.").on_truecolor(23, 23, 23));
    println!("Player 1 does not get to go again because they already had their 4th turn.");
    println!(
        "{}",
        ("If you reach 100 points, you may continue rolling to set a higher score to beat.")
            .on_truecolor(23, 23, 23)
    );
    println!("\nGOOD LUCK, HAVE FUN");
    println!("{}", ("\ntype 'roll' to roll").dimmed());
    println!("{}", ("type 'scoreboard' to see score").dimmed());
    println!("{}", ("\nHow many players will be playing?").bright_blue());
}
