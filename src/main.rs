mod greed;
use greed::{mega_roll, roll};

use owo_colors::OwoColorize;
use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// implement items!
// add emojis on vim
// update scoreboard

// better way to rewrite if loops:
// use match statements!
// match keyboard_roll {
//     "roll" => normal_roll();
// }

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    score: i32,
    turn_count: i32,
    items: Items,
}

impl Player {
    fn print_scores(&self) {
        println!("{} {}", self.name, self.score)
    }
}

const TARGET: i32 = 100;

#[derive(Debug, Clone, PartialEq, EnumIter)]
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

impl Distribution<Items> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Items {
        match rng.gen_range(0..=7) {
            0 => Items::MegaDice,
            1 => Items::TripleDice,
            2 => Items::LeechDice,
            3 => Items::EvilDice,
            4 => Items::EvenOddHighLow,
            5 => Items::ScoreSwap,
            _ => Items::Yoink,
        }
    }
}

pub fn print_milady() -> () {
    println!("\n\n");
    println!("{}", ("      .*O@@@@@@@@@@#o*.").bold().dimmed());
    println!("{}", ("   *#@@@@@@@@@@@@@@@@@@#o.").bold().dimmed());
    println!("{}", ("  O@@@@@@@@@@@@@@@@@@@@@@#°").bold().dimmed());
    println!("{}", (" *@@@@@@@@@@@@@@@@@@@@@@@@*").bold().dimmed());
    println!("{}", (" *@@@@@@@@@@@@@@@@@@Oo.@@o").bold().dimmed());
    println!("{}", (" °#@@@@@@@o   .@@@@@   ##.").bold().dimmed());
    println!("{}", (" .O@@@@@@@#*  .@@@@@@O#@o").bold().dimmed());
    println!("{}", ("   .oO@@@@@@@@@@@@@@@@@o").bold().dimmed());
    println!("{}", ("        .*o#@@@@@@@@#*.").bold().dimmed());
    println!("{}", ("         *O@@@@@@@@@@O°").bold().dimmed());
    println!("{}", ("       .#@@@@@@@@@@@@@@*").bold().dimmed());
    println!("{}", ("       o@@@@@@@@@@@@@@@@°").bold().dimmed());
    println!("{}", ("      .#@@@@@@@@@@@@@@@@O").bold().dimmed());
    println!("{}", ("      °@@@@@@@@@@@@@@@@@#°").bold().dimmed());
}

pub fn print_instructions() -> () {
    println!(
        "\n       {}{}{}{}{}{}{}",
        ("🔥"),
        ("🎲"),
        ("🔥"),
        ("GREED").green().blink(),
        ("🔥"),
        ("🎲"),
        ("🔥"),
    );
    println!("{}", ("\n\n\nHOW TO PLAY").bright_cyan().bold());
    println!(
        "{}{}{}",
        ("\ntype ").dimmed(),
        ("r").bright_purple().italic(),
        (" to roll the dice").dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").dimmed(),
        ("s").bright_purple().italic(),
        (" to see scoreboard").dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").dimmed(),
        ("i").bright_purple().italic(),
        (" to check item bag\n").dimmed()
    );
    println!(
        "First player to reach {} wins",
        TARGET.bold().bright_green()
    );
    println!(
        "{}",
        ("Players take turn rolling the dice as many times as they want.\n")
    );
    println!(
        "{}{}{}",
        ("If you roll "),
        ("SNAKE EYES").cyan().italic(),
        (" (two ones), you lose all of your points.")
    );
    println!(
        "{}{}{}",
        ("If you roll "),
        ("DOUBLES").cyan().italic(),
        (" those dice are doubles in value.")
    );
    println!(
        "{}{}{}{}{}",
        ("If you roll a "),
        ("1").cyan(),
        (" you get "),
        ("0").red(),
        (" points and your turn is over.")
    );
    println!("\n{}", ("GOOD LUCK").bright_green().bold());
    println!("{}", ("\nHow many players?").bright_blue());
}

pub fn set_player(name: String) -> Player {
    Player {
        name,
        score: 0,
        turn_count: 0,
        items: Items::Empty,
    }
}

fn main() {
    print_milady();
    print_instructions();

    // How many players?
    let mut p_string = String::new();
    io::stdin().read_line(&mut p_string).expect("cant read");
    let p_string = p_string.trim();
    // change p string into and i32 so we can see how many players to create
    let p_num: i32 = p_string.parse().unwrap();
    // create empty vec to hold players
    let mut pvec: Vec<Player> = Vec::new();

    // create players
    let mut i = 1;
    loop {
        // get names
        println!(
            "{}{}{}",
            ("PLAYER ").on_bright_green().bold(),
            (i).on_bright_green().bold(),
            (" ENTER YOUR NAME").bright_green()
        );
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name).expect("cant read");
        let trimmed_name: String = new_name.trim().to_string();
        let player: Player = set_player(trimmed_name);
        // push each player in
        pvec.push(player);

        if i >= p_num {
            break;
        }
        i += 1;
    }

    // dice simulation
    fn dice_roll() -> i32 {
        thread_rng().gen_range(1..7)
    }

    fn mega_dice_roll() -> i32 {
        thread_rng().gen_range(1..11)
    }

    // vector of random prompts to spice it up
    let mut random_prompts: Vec<String> = Vec::new();
    random_prompts.push(String::from(", ⌚ TIME TO ROLL"));
    random_prompts.push(String::from(", 😎 IT WOULD BE COOL IF YOU ROLLED"));
    random_prompts.push(String::from(", 🍀 GOOD LUCK!"));
    random_prompts.push(String::from(", 🐌 TAKE YOUR TIME"));

    // random messages after rolling 1s
    let mut random_ones: Vec<String> = Vec::new();
    random_ones.push(String::from("👹👹👹👹👹👹👹👹👹"));
    random_ones.push(String::from("🪦🤡"));
    random_ones.push(String::from("🤣😹😂😹🤣"));
    random_ones.push(String::from("🕷️🪲🪰🦗🪱🦟🪳🐜"));

    // snake eyes
    // doubles
    let mut dubs_msg: Vec<String> = Vec::new();
    dubs_msg.push(String::from("🤠🎉DOUBLES🎉🤠"));
    dubs_msg.push(String::from("👽👾🌌🛸🌕🛸🌌👾👽"));
    dubs_msg.push(String::from("🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔"));
    dubs_msg.push(String::from("🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄"));

    let mut endgame: Vec<String> = Vec::new();
    endgame.push(String::from("GOOD LUCK"));
    endgame.push(String::from("DON'T CHOKE"));
    endgame.push(String::from("YOU CAN DO IT"));
    endgame.push(String::from("DON'T LOSE"));

    // if i make the same number of prompts for each case
    // then i only need one function
    fn gen_prompt() -> usize {
        thread_rng().gen_range(0..3)
    }

    // collects players who get one last turn
    fn last_turns(players: Vec<Player>, turn_count: i32) -> Vec<Player> {
        players
            .into_iter()
            .filter(|p| p.turn_count == turn_count)
            .collect()
    }

    let mut i: usize = 0;
    'game: loop {
        // call gen functions
        let index: usize = gen_prompt();
        // try_into().unwrap() changes p_num from i32 to usize
        let mut turn_scores: Vec<i32> = vec![0; p_num.try_into().unwrap()];
        // roll msg
        println!(
            "\n\n{}{}\n",
            pvec[i]
                .name
                .to_ascii_uppercase()
                .bold()
                .italic()
                .bright_green(),
            random_prompts[index].cyan()
        );

        'turn: loop {
            let r1 = dice_roll();
            let r2 = dice_roll();
            let m1 = mega_dice_roll();
            let m2 = mega_dice_roll();
            let secret_num = dice_roll();

            let mega1 = mega_dice_roll();
            let mega2 = mega_dice_roll();

            let mut keyboard_roll = String::new();
            io::stdin()
                .read_line(&mut keyboard_roll)
                .expect("cant read that");

            // construction zone
            match keyboard_roll.trim() {
                "s" => {
                    println!("\n{}", ("SCOREBOARD").cyan().bold());
                    for i in pvec.iter() {
                        println!(
                            "{} {}",
                            i.name.to_ascii_uppercase().bright_magenta(),
                            i.score.bright_green().bold()
                        )
                    }
                },
                // normal roll
                "r" => {
                    // 🎲🎲 print roll
                    println!(
                        "\n{} + {} = {}\n",
                        r1.red().on_white().bold(),
                        r2.red().on_white().bold(),
                        (r1 + r2).bright_green()
                    );

                    // dice options
                    // rolling snake eyes
                    if r1 == 1 && r2 == 1 {
                        println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                        pvec[i].score *= 0;
                        println!("{}", ("TOTAL SCORE 0").red());
                        pvec[i].turn_count += 1;
                        break 'turn;

                    // rolling a 1
                    } else if r1 == 1 || r2 == 1 {
                        println!("\n{}", random_ones[index]);
                        println!("{}", ("ROLLED A 1!").dimmed());
                        println!("{}", ("TURN COMPLETE").red());
                        println!("{} {}", ("TOTAL SCORE").blue(), pvec[i].score);
                        pvec[i].turn_count += 1;
                        break 'turn;

                    // rolling secret num
                    } else if r1 == secret_num || r2 == secret_num {
                        // insert emojis NVIM
                        let random_item: Items = rand::random();
                        println!("{}", ("secret number rolled!").italic().dimmed());
                        println!(
                            "{}{}{:?}\n",
                            pvec[i].name.to_ascii_uppercase().bright_green().bold(),
                            (" picked up ").bright_cyan(),
                            random_item.bright_magenta().bold()
                        );
                        pvec[i].items = random_item;

                    // rolling doubles
                    } else if r1 == r2 {
                        /* 👽 */
                        println!("{}", dubs_msg[index]);
                        turn_scores[i] += r1 * 4;
                        println!("\nx2 = {}🎉", (r1 * 4).bright_green());
                        println!(
                            "{} {},{} {}",
                            ("TURN SCORE").dimmed(),
                            turn_scores[i].green(),
                            (" ROLL AGAIN?").bright_blue(),
                            ("y / n").dimmed()
                        );

                        // go again?
                        let mut response = String::new();
                        io::stdin().read_line(&mut response).expect("can't read");
                        let binary = response.contains("y");

                        // yes
                        if binary == true {
                            continue 'turn;
                        }
                        // no
                        else {
                            pvec[i].score += turn_scores[i];
                            println!(
                                "{} {}",
                                ("TOTAL SCORE").blue(),
                                pvec[i].score.bright_green()
                            );
                            pvec[i].turn_count += 1;
                            break 'turn;
                        }
                    }

                    // normal roll
                    // go again?
                    turn_scores[i] += r1 + r2;
                    println!(
                        "{}{},{} {}",
                        ("turn score:").dimmed(),
                        turn_scores[i].green(),
                        (" ROLL AGAIN?").bright_blue(),
                        ("y / n").dimmed()
                    );
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).expect("can't read");
                    let binary = response.contains("y");

                    // yes
                    if binary == true {
                        continue 'turn;
                    }
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!(
                            "{} {}",
                            ("TOTAL SCORE").blue(),
                            pvec[i].score.bright_green()
                        );
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
                },
                "mega" => if pvec[i].items == Items::MegaDice {
                    // 🎲🎲 print roll
                    println!(
                        "\n{} + {} = {}\n",
                        m1.red().on_white().bold(),
                        m2.red().on_white().bold(),
                        (m1 + m2).bright_green()
                    );

                    // dice options
                    // rolling snake eyes
                    if m1 == 1 && m2 == 1 {
                        println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                        pvec[i].score *= 0;
                        println!("{}", ("TOTAL SCORE 0").red());
                        pvec[i].turn_count += 1;
                        break 'turn;

                    // rolling a 1
                    } else if m1 == 1 || m2 == 1 {
                        println!("\n{}", random_ones[index]);
                        println!("{}", ("ROLLED A 1!").dimmed());
                        println!("{}", ("TURN COMPLETE").red());
                        println!("{} {}", ("TOTAL SCORE").blue(), pvec[i].score);
                        pvec[i].turn_count += 1;
                        break 'turn;

                    // rolling secret num
                    } else if m1 == secret_num || m2 == secret_num {
                        // insert emojis NVIM
                        let random_item: Items = rand::random();
                        println!("{}", ("secret number rolled!").italic().dimmed());
                        println!(
                            "{}{}{:?}\n",
                            pvec[i].name.to_ascii_uppercase().bright_green().bold(),
                            (" picked up ").bright_cyan(),
                            random_item.bright_magenta().bold()
                        );
                        pvec[i].items = random_item;

                    // rolling doubles
                    } else if m1 == m2 {
                        /* 👽 */
                        println!("{}", dubs_msg[index]);
                        turn_scores[i] += m1 * 4;
                        println!("\nx2 = {}🎉", (m1 * 4).bright_green());
                        println!(
                            "{} {},{} {}",
                            ("TURN SCORE").dimmed(),
                            turn_scores[i].green(),
                            (" ROLL AGAIN?").bright_blue(),
                            ("y / n").dimmed()
                        );

                        // go again?
                        let mut response = String::new();
                        io::stdin().read_line(&mut response).expect("can't read");
                        let binary = response.contains("y");

                        // yes
                        if binary == true {
                            continue 'turn;
                        }
                        // no
                        else {
                            pvec[i].score += turn_scores[i];
                            println!(
                                "{} {}",
                                ("TOTAL SCORE").blue(),
                                pvec[i].score.bright_green()
                            );
                            pvec[i].turn_count += 1;
                            break 'turn;
                        }
                    }

                    // normal roll
                    // go again?
                    turn_scores[i] += m1 + m2;
                    println!(
                        "{}{},{} {}",
                        ("turn score:").dimmed(),
                        turn_scores[i].green(),
                        (" ROLL AGAIN?").bright_blue(),
                        ("y / n").dimmed()
                    );
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).expect("can't read");
                    let binary = response.contains("y");

                    // yes
                    if binary == true {
                        continue 'turn;
                    }
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!(
                            "{} {}",
                            ("TOTAL SCORE").blue(),
                            pvec[i].score.bright_green()
                        );
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
                    
                },
                _ => println!("please enter valid command"),
            }
            // construction zone

            if keyboard_roll.trim().contains("121212") {
                // 🎲🎲 print roll
                println!(
                    "\n{} + {} = {}\n",
                    r1.red().on_white().bold(),
                    r2.red().on_white().bold(),
                    (r1 + r2).bright_green()
                );

                // dice options
                // rolling snake eyes
                if r1 == 1 && r2 == 1 {
                    println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                    pvec[i].score *= 0;
                    println!("{}", ("TOTAL SCORE 0").red());
                    pvec[i].turn_count += 1;
                    break 'turn;

                // rolling a 1
                } else if r1 == 1 || r2 == 1 {
                    println!("\n{}", random_ones[index]);
                    println!("{}", ("ROLLED A 1!").dimmed());
                    println!("{}", ("TURN COMPLETE").red());
                    println!("{} {}", ("TOTAL SCORE").blue(), pvec[i].score);
                    pvec[i].turn_count += 1;
                    break 'turn;

                // rolling secret num
                } else if r1 == secret_num || r2 == secret_num {
                    // insert emojis NVIM
                    let random_item: Items = rand::random();
                    println!("{}", ("secret number rolled!").italic().dimmed());
                    println!(
                        "{}{}{:?}\n",
                        pvec[i].name.to_ascii_uppercase().bright_green().bold(),
                        (" picked up ").bright_cyan(),
                        random_item.bright_magenta().bold()
                    );
                    pvec[i].items = random_item;

                // rolling doubles
                } else if r1 == r2 {
                    /* 👽 */
                    println!("{}", dubs_msg[index]);
                    turn_scores[i] += r1 * 4;
                    println!("\nx2 = {}🎉", (r1 * 4).bright_green());
                    println!(
                        "{} {},{} {}",
                        ("TURN SCORE").dimmed(),
                        turn_scores[i].green(),
                        (" ROLL AGAIN?").bright_blue(),
                        ("y / n").dimmed()
                    );

                    // go again?
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).expect("can't read");
                    let binary = response.contains("y");

                    // yes
                    if binary == true {
                        continue 'turn;
                    }
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!(
                            "{} {}",
                            ("TOTAL SCORE").blue(),
                            pvec[i].score.bright_green()
                        );
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
                }

                // normal roll
                // go again?
                turn_scores[i] += r1 + r2;
                println!(
                    "{}{},{} {}",
                    ("turn score:").dimmed(),
                    turn_scores[i].green(),
                    (" ROLL AGAIN?").bright_blue(),
                    ("y / n").dimmed()
                );
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("can't read");
                let binary = response.contains("y");

                // yes
                if binary == true {
                    continue 'turn;
                }
                // no
                else {
                    pvec[i].score += turn_scores[i];
                    println!(
                        "{} {}",
                        ("TOTAL SCORE").blue(),
                        pvec[i].score.bright_green()
                    );
                    pvec[i].turn_count += 1;
                    break 'turn;
                }
            } else if keyboard_roll.contains("banana") {
                println!("{}", ("~ B A N A N A ~").yellow());
            } else if keyboard_roll.contains("triples is best") {
                println!("you rolled: \n {} {} {}", r1, r2, secret_num);
                turn_scores[i] += r1 + r2 + secret_num;
                println!("turn score: {}", turn_scores[i].bright_magenta())
            }
            // 💯💯💯💯💯 scoreboard
            else if keyboard_roll.contains("999") {
                let name = &pvec[i].name;
                let score = &pvec[i].score;

                println!("\n{} {}\n", name.cyan(), score.bright_green())
            } else if keyboard_roll.trim().contains("i") {
                println!("\n~{:?}~\n", pvec[i].items.bright_blue());
                match pvec[i].items {
                    Items::Empty => println!(" "),
                    Items::MegaDice => println!(
                        "{}{}{}\n",
                        ("type ").dimmed(),
                        ("mega").bright_magenta(),
                        (" to roll 2 ten-sided-dice.").dimmed()
                    ),
                    Items::LeechDice => println!(
                        "{}{}{}\n",
                        ("type ").dimmed(),
                        ("leech").bright_magenta(),
                        (" to leech off the score of another player").dimmed()
                    ),
                    Items::EvenOddHighLow => println!(
                        "{}{}{}\n{}{}\n",
                        ("type ").dimmed(),
                        ("even").bright_magenta(),
                        (" to guess even/odd, then guess higher/lower.").dimmed(),
                        ("If you get 1 right your, roll is doubled.").dimmed(),
                        ("If you get both right, your roll is quadrupled").dimmed()
                    ),
                    Items::Yoink => println!(
                        "{}{}{}\n",
                        ("type ").dimmed(),
                        ("yoink").bright_magenta(),
                        (" to yoink an item from another player").dimmed()
                    ),
                    Items::EvilDice => println!(
                        "{}{}{}{}{}\n",
                        ("type ").dimmed(),
                        ("evil").bright_magenta(),
                        (" to roll the Evil Dice.").dimmed(),
                        ("\nDice value will be doubled and subtracted from opponents' scores.")
                            .dimmed(),
                        ("You will not gain any points.").dimmed()
                    ),
                    Items::ScoreSwap => println!(
                        "{}{}{}\n",
                        ("type ").dimmed(),
                        ("swap").bright_magenta(),
                        (" to swap scores with any opponent you choose.").dimmed()
                    ),
                    Items::TripleDice => println!(
                        "{}{}{}\n",
                        ("type ").dimmed(),
                        ("triple").bright_magenta(),
                        (" to roll 3 dice.").dimmed()
                    ),
                }
            } else if keyboard_roll.trim().contains("mega") && pvec[i].items != Items::MegaDice {
                println!("{}", ("YOU DON'T HAVE ANY MEGADICE").red())
            } else if keyboard_roll.trim().contains("mega") && pvec[i].items == Items::MegaDice {
                // 🎲🎲 print roll
                println!(
                    "\n{} + {} = {}\n",
                    mega1.red().on_white().bold(),
                    mega2.red().on_white().bold(),
                    (mega1 + mega2).bright_green()
                );
                turn_scores[i] += mega1 + mega2;

                println!(
                    "{}{},{} {}",
                    ("turn score:").dimmed(),
                    turn_scores[i].green(),
                    (" ROLL AGAIN?").bright_blue(),
                    ("y / n").dimmed()
                );
                pvec[i].items = Items::Empty;
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("can't read");
                let binary = response.contains("y");

                // yes
                if binary == true {
                    continue 'turn;
                }
                // no
                else {
                    pvec[i].score += turn_scores[i];
                    println!(
                        "{} {}",
                        ("TOTAL SCORE").blue(),
                        pvec[i].score.bright_green()
                    );
                    pvec[i].turn_count += 1;
                    break 'turn;
                }
            }

            //💯💯💯💯💯💯💯
        }
        // end of turn loop

        // check if anyone has won

        // temp changed to 30 so i can test the end game in terminal

        // if last player wins then this loop is unessecary
        if pvec[i].score >= TARGET {
            println!("\nCONGRATS {}!", (pvec[i].name).bright_green());
            println!("\n🏆🥇YOU WON🥇🏆");
            if i == (p_num - 1).try_into().unwrap() {
                break 'game;
            } else {
                println!(
                    "\n...but not so fast. You won in {} turns,\n",
                    (pvec[i].turn_count).cyan()
                );
                println!(
                    "players who have not had {} turns get to go again.",
                    (pvec[i].turn_count).cyan()
                );

                let total_turns_minus_one = pvec[i].turn_count - 1;
                let cloned_pvec = pvec.clone();
                let mut final_round_players = last_turns(cloned_pvec, total_turns_minus_one);
                println!("{:?}", final_round_players);

                let mut v: usize = 0;
                let mut high_scores: Vec<Player> = vec![pvec[i].clone()];
                loop {
                    'final_turn: loop {
                        if final_round_players[v].score > high_scores[0].score {
                            println!(
                                "{} you've passed {}'s score",
                                final_round_players[v].name.trim().bold().on_cyan(),
                                pvec[i].name.trim().bold().on_bright_magenta()
                            );
                            high_scores.pop();
                            high_scores.push(final_round_players[v].clone());
                            println!("You set the new high score!");
                        }
                        println!(
                            "\n\n{}, {}",
                            final_round_players[v].name.bright_green().bold(),
                            endgame[index].bold()
                        );
                        println!(
                            "score:{}, {} points away!",
                            final_round_players[v].score.cyan().bold(),
                            (high_scores[0].score - final_round_players[v].score).red()
                        );

                        //use normal turn functions!
                        let mut endroll = String::new();
                        io::stdin().read_line(&mut endroll).expect("cant read");
                        if endroll.trim().contains("roll") {
                            let roll1 = dice_roll();
                            let roll2 = dice_roll();
                            // 🎲🎲 print roll
                            println!(
                                "\n\n{} + {} = {}",
                                roll1.red().on_white().bold(),
                                roll2.red().on_white().bold(),
                                (roll1 + roll2).bright_green()
                            );
                            if roll1 == 1 && roll2 == 1 {
                                println!(
                                    "sorry {}, u lose",
                                    final_round_players[v].name.trim().bold().bright_green()
                                );
                                v += 1;
                                break 'final_turn;
                            } else if roll1 == 1 || roll2 == 1 {
                                println!(
                                    "better luck next time. thanks for playing {}",
                                    final_round_players[v]
                                        .name
                                        .trim()
                                        .bright_yellow()
                                        .on_bright_purple()
                                );
                                v += 1;
                                break 'final_turn;
                            } else if roll1 == roll2 {
                                println!("fuck yeah lets fucking go thats good thats real good keep doing that");
                                final_round_players[v].score += roll1 * 4;
                                println!("keep rolling? \ny or n");
                                let mut yon = String::new();
                                io::stdin()
                                    .read_line(&mut yon)
                                    .expect("error can't read that");
                                let noy: bool = yon.contains("y");
                                if noy == true {
                                    continue 'final_turn;
                                } else {
                                    v += 1;
                                    break 'final_turn;
                                }
                                // remember to let players win on last round
                            } else {
                                final_round_players[v].score += roll1 + roll2;
                                if final_round_players[v].score > pvec[i].score {
                                    println!(
                                        "{} you have surpassed {}'s score",
                                        final_round_players[v].name.trim().bold().on_cyan(),
                                        pvec[i].name.trim().bold().on_bright_magenta()
                                    );
                                    high_scores.pop();
                                    high_scores.push(final_round_players[v].clone());
                                    println!("You set the new high score!");
                                    println!("keep rolling? \ny or n");
                                    let mut yon = String::new();
                                    io::stdin()
                                        .read_line(&mut yon)
                                        .expect("error can't read that");
                                    let noy: bool = yon.contains("y");
                                    if noy == true {
                                        continue 'final_turn;
                                    } else {
                                        v += 1;
                                        break 'final_turn;
                                    }
                                }
                                continue 'final_turn;
                            }
                        }
                        // end final turn loop

                        if v == final_round_players.len() - 1 {
                            println!("{:?} is the high score", high_scores);
                            break 'game;
                        } else {
                            v += 1;
                        }
                    }
                    // end final game loop
                }
            }
            // end turn loop
        }

        let p_num_minus_one = p_num - 1;
        if i == p_num_minus_one.try_into().unwrap() {
            i *= 0
        } else {
            i += 1;
        }
    }
}
// end game loop

// thanks for playing
// -SwimmingPigLuvr

//   ⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀
//  ⠀⠀⠀⠀⢀⣴⠾⠛⠉⠉⠉⠉⠛⠿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⠀⠀⣀⠀⠤⠤⠤⠀⣀⠀⠀⠀⠀⠀⠀⠀
//  ⠀⠀⠀⢠⡿⠁⠀⢀⣠⣤⣤⣄⡀⠀⠈⢿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠐⢉⠴⣚⠉⣙⠢⢄⡤⢞⡂⢀⣐⢄⠀⠀
//  ⠀⠀⢀⣿⣁⣀⣠⡿⠋⠀⠀⠙⢿⣄⣀⣈⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⡤⣞⢁⠊⢀⣀⠐⢿⡄⠰⢁⡀⠈⠺⣦⢡⠀
//  ⠀⠀⢸⣿⠛⠛⢻⣧⠀⠿⠇⠀⣼⡟⠛⠛⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣗⣟⡸⠀⠻⡿⠃⢸⣇⢃⠿⠿⠀⠀⣽⢸⠀
//  ⠀⠀⢸⣿⠀⠀⠀⠙⢷⣦⣴⡾⠋⠀⠀⠀ ⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠈⠙⢷⣴⡀⠀⠠⣪⣾⣷⡄⡀⠠⣐⢕⠁⠀
//  ⠀⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  ⣿⡇⠀⠀⠀⢰⡦⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠙⠲⡖⠓⠉⠁⠈⠉⠒⠀⠈⢸⠀⠀
//  ⠀⠀⣸⣿⠀⠀⠀⠛⠷⠶⠶⠾⠛⠀⠀⠀ ⣿⣇⠀⠀⢶⣿⣷⣤⢀⣀⡀⠀⠀⣏⡑⠢⢄⠀⠀⠀⠈⠐⠀⠐⠀⠀⠀⠀⠀⡸⡀⠀
//  ⠀⣸⣿⣿⢷⣦⣀⣀⣀⣀⣀⣀⣀⣀⣴⡾⣿⣿⣇⠀⠛⠛⠛⠟⠀⠤⠤⠌⢉⠀⠈⠓⢬⣿⣦⡤⣤⣤⠤⠤⣤⣤⣤⣤⣚⣔⣄⠀
//  ⢠⣿⢸⣿⠀⣿⡏⠉⠉⠉⠉⠉⠉⢹⣿⠀⣿⡇⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⡤⠂⠀⠀⢀⠤⠤⢄⡨⠔⠒⢍⠉⢁⣯⡆
//  ⢸⡏⢸⣿⣀⣿⡇⠀⠀⠀⠀⠀⠀ ⢸⣿⣀⣿⡇⢹⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⡗⢤⡤⣬⠀⠀⠀⢇⠀⠀⠀⠁⠀⠀⡸⢰⣿⣿⡿
//  ⢸⡇⠀⢿⣏⠉⠁⠀⠀⠀⠀⠀⠀  ⠈⠉⣹⡿⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢌⡿⣽⡀⠀⠀⠈⠒⢄⡀⠀⢀⠔⠁⠈⠙⡋⠀
//  ⢸⣿⣤⣌⠛⠷⣶⣶⣶⣶⣶⣶⣶⣶⠾⠛⣡⣤⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠳⢧⣠⣤⣄⣠⣀⣈⣱⡥⠤⠴⠦⠴⠃⠀
//  ⠘⠿⠿⠇⠀⠀⠀⢿⡾⠇⠸⢷⡿⠀⠀⠀⠸⠿⠿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⠀⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀
//  ⠀⠀⠀⠀⠀⠀ ⠀⠛⠛⠁⠈⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀⠙⠉⠉⠀⠈⠉⠉⠉⠀⠀⠀⠀⠀⠀
