#![deny(clippy::pedantic)]
// use std::borrow::Borrow;
// use std::cmp;
use owo_colors::OwoColorize;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use std::io;

// TO DO
// evil dice: if triples is rolled then all other players are set back to 0,
// if one is rolled then turn score is 0, if triple 1's then player is banished

// check score: checks all player scores

#[derive(Debug, Clone)]
struct Player {
    name: String,
    score: i32,
    turn_count: i32,
}

// this takes the user input and creates a new Player type
fn set_player(name: String) -> Player {
    Player {
        name,
        score: 0,
        turn_count: 0,
    }
}

// dice simulation
fn dice_roll() -> i32 {
    let mut rng = thread_rng();
    let roll: i32 = rng.gen_range(1..7);
    roll
}

// this generates a random number 0-14 because there are 15 random prompts
fn gen_prompt_num() -> usize {
    let mut prompt: ThreadRng = thread_rng();
    let prompt_num: usize = prompt.gen_range(0..14);
    prompt_num
}
// R.O. means random One
fn gen_ro_prompt() -> usize {
    let mut ro: ThreadRng = thread_rng();
    let ro_num: usize = ro.gen_range(0..4);
    ro_num
}

// fn gen_snake_msg
// fn gen_doubles_msg

// Using Closures that Capture Their Environment
// https://doc.rust-lang.org/book/ch13-02-iterators.html

// this function creates a vec of Players who have a certain turn_count left
// use .filter method to find players who still have to have a final turn
fn last_turns(players: Vec<Player>, turn_count: i32) -> Vec<Player> {
    players
        .into_iter()
        .filter(|p| p.turn_count == turn_count)
        .collect()
}

fn main() {
    // Game rules
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

    let mut p_string = String::new();
    io::stdin().read_line(&mut p_string).expect("cant read");
    // read_line will read the user input
    // and it will also read when they press 'enter'
    // .trim() takes that off
    let p_string = p_string.trim();
    // change p string into and i32 so we can see how many players to create
    let p_num: i32 = p_string.parse().unwrap();
    // create empty vec to hold players
    let mut pvec: Vec<Player> = Vec::new();

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
        // use set_player function with user input as the parameter
        let player: Player = set_player(new_name);
        // push each player in
        pvec.push(player);

        if i >= p_num {
            break;
        }
        i += 1;
    }

    // vector of random prompts to spice it up
    let random_prompts = [
        "HURRY UP AND ROLL",
        "ROLL THEM SHITS, BITCH",
        "ROLL THE DICE!!!",
        "YOUR TURN",
        "*picks up dice...shakes vigorously...rolls them across the table",
        "GOOD LUCK, HAVE FUN :)",
        "DON'T BE GREEDY",
        "YOU CAN DO IT!!!",
        "your family is counting on you",
        "please roll",
        "your turn to roll now",
        "right now it is your turn to roll now",
        "ROLL THE VIRTUAL DICE",
        "fucking roll already",
        "please roll, milady",
    ];

    // random messages after rolling 1s
    let random_ones = [
        "you rolled a 1! no points for this turn",
        "get good nerd",
        "no points scored because you rolled a 1",
        "get rekt",
        "HAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHAHA",
    ];

    // snake eyes
    // doubles

    let mut i: usize = 0;
    'game: loop {
        // call gen functions
        let rp_index: usize = gen_prompt_num();
        let ro_index: usize = gen_ro_prompt();
        // turn score vec: returns to 0 after your turn is over
        // create new vec with vec![x; y]
        // x being the value, y being the amount
        // so we have p_num amount of 0s in our vec

        // wait... do i even need to have a turn score for each player??
        // i mean the score resets with the game loop anyways.
        // and i only ever call turn_scores[i]
        // so yes i dont need one for each player. i will change that later

        // could be useful if i wanted to record highest turn score
        // would be badass if you didnt win but you got an award for having the best run

        // try_into().unwrap() changes p_num from i32 to usize
        let mut turn_scores: Vec<i32> = vec![0; p_num.try_into().unwrap()];
        println!(
            "\n\t{} {}: {}",
            pvec[i]
                .name
                .trim()
                .to_ascii_uppercase()
                .bold()
                .on_truecolor(150, 0, 100),
            pvec[i].score.underline().truecolor(150, 0, 100),
            random_prompts[rp_index].cyan()
        );

        'turn: loop {
            let roll1 = dice_roll();
            let roll2 = dice_roll();
            let roll3 = dice_roll();
            let mut keyboard_roll = String::new();
            io::stdin()
                .read_line(&mut keyboard_roll)
                .expect("cant read that");
            if keyboard_roll.trim().contains("roll") {
                println!(
                    "\n\t{} & {}",
                    roll1.on_truecolor(0, 95, 95).bold().underline(),
                    roll2.on_truecolor(69, 0, 69).bold().underline()
                );

                // rolling snake eyes
                if roll1 == 1 && roll2 == 1 {
                    println!("\n{}", ("SNAKE EYES").on_bright_magenta());
                    pvec[i].score *= 0;
                    println!(
                        "{}, your total score is {}",
                        pvec[i].name.trim(),
                        pvec[i].score
                    );
                    pvec[i].turn_count += 1;
                    break 'turn;

                // rolling a 1
                } else if roll1 == 1 || roll2 == 1 {
                    println!("\n{}", random_ones[ro_index]);
                    println!(
                        "{}, your total score is {}",
                        pvec[i].name.trim(),
                        pvec[i].score
                    );
                    pvec[i].turn_count += 1;
                    break 'turn;

                // rolling doubles
                } else if roll1 == roll2 {
                    println!("\n{}", ("DOUBLES").bright_yellow().bold());
                    turn_scores[i] += roll1 * 4;
                    println!(
                        "{}: {} {}",
                        pvec[i].name.trim().red(),
                        ("your turn score is:").cyan(),
                        turn_scores[i].bold().on_bright_cyan()
                    );
                    println!("\nroll again? [y,n]");

                    // go again?
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).expect("can't read");
                    let binary = response.contains('y');

                    // yes
                    if binary {
                        continue 'turn;
                    }
                    // no
                    else {
                        pvec[i].score += turn_scores[i];
                        println!(
                            "\n{}, your total score is {}",
                            pvec[i].name.trim().on_truecolor(3, 4, 5),
                            pvec[i].score.bright_green()
                        );
                        pvec[i].turn_count += 1;
                        break 'turn;
                    }
                }
                // normal roll
                // go again?
                turn_scores[i] += roll1 + roll2;
                println!(
                    "{}, your turn score is {}",
                    pvec[i].name.trim().on_cyan(),
                    turn_scores[i].red()
                );
                println!("\nroll again? {}", ("[y,n]").on_bright_purple());
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("can't read");
                let binary = response.contains('y');

                // yes
                if binary {
                    continue 'turn;
                }
                // no
                else {
                    pvec[i].score += turn_scores[i];
                    println!(
                        "{}, your total score is {}",
                        pvec[i].name.trim().on_truecolor(3, 44, 55),
                        pvec[i].score
                    );
                    pvec[i].turn_count += 1;
                    break 'turn;
                }
            } else if keyboard_roll.contains("banana") {
                println!("{}", ("~ B A N A N A ~").yellow());
            } else if keyboard_roll.contains("triples is best") {
                println!("you rolled: \n {} {} {}", roll1, roll2, roll3);
                turn_scores[i] += roll1 + roll2 + roll3;
                println!("turn score: {}", turn_scores[i].bright_magenta());
            } else if keyboard_roll.contains("scoreboard") {
                let mut s = 0;
                loop {
                    println!("{:X?} {:?}", pvec[s].name.trim(), pvec[s].score);
                    if s + 1 == p_num.try_into().unwrap() {
                        break;
                    }
                    s += 1;
                }
            }
        }
        // end of turn loop

        // check if anyone has won

        // temp changed to 30 so i can test the end game in terminal

        // if last player wins then this loop is unessecary
        if pvec[i].score >= 100 {
            println!("CONGRATS {}", pvec[i].name);
            println!("***YOU WIN***");
            if i == (p_num - 1).try_into().unwrap() {
                break 'game;
            } else {
                println!("...but not so fast");
                println!("you won in {} turns, so every player who hasn't had {} turns gets to try one last time", pvec[i].turn_count, pvec[i].turn_count);

                let total_turns_minus_one = pvec[i].turn_count - 1;
                let cloned_pvec = pvec.clone();
                let mut final_round_players = last_turns(cloned_pvec, total_turns_minus_one);

                let mut v: usize = 0;
                let mut high_scores: Vec<Player> = vec![pvec[i].clone()];
                loop {
                    'final_turn: loop {
                        if final_round_players[v].score > high_scores[0].score {
                            println!(
                                "{} you have surpassed {}'s score",
                                final_round_players[v].name.trim().bold().on_cyan(),
                                pvec[i].name.trim().bold().on_bright_magenta()
                            );
                            high_scores.pop();
                            high_scores.push(final_round_players[v].clone());
                            println!("You set the new high score!");
                        }
                        println!(
                            "last chance {}",
                            final_round_players[v].name.trim().bright_green().bold()
                        );
                        println!(
                            "your score is {}, {} points away!",
                            final_round_players[v].score.yellow().bold(),
                            high_scores[0].score - final_round_players[v].score
                        );
                        println!("type 'milady' to roll for the last time");
                        let mut milady = String::new();
                        io::stdin().read_line(&mut milady).expect("cant read");
                        if milady.trim().contains("milady") {
                            let roll1 = dice_roll();
                            let roll2 = dice_roll();
                            println!("You");
                            println!("rolled");
                            println!("...");
                            println!("{} + {}", roll1.red().on_bright_white(), roll2.red().on_white());
                            if roll1 == 1 && roll2 == 1 {
                                println!(
                                    "wow {} you are very unlucky",
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
                                let noy: bool = yon.contains('y');
                                if noy {
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
                                    let noy: bool = yon.contains('y');
                                    if noy {
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

        // use filter method to create vec of structs with given attribute
        // give only the remaining players the option to roll 1 last time
        // iter through slice of the vector
        // then see if anyone has gotten a higher score
        // break loop print winner
        // reset turn loop back to player 1
        let p_num_minus_one = p_num - 1;
        if i == p_num_minus_one.try_into().unwrap() {
            i *= 0;
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
