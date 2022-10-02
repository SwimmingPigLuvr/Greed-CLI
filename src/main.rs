use owo_colors::OwoColorize;
use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::mem;
use std::borrow::Borrow;
use std::io::Read;
use std::{default, io};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// 9-29
// To Do
// implement all items

// 930 to do
// figue out how to swap two values
// use other values to buffer?
// use std::mem::swap?

// 10-1 TODO
// mario kart:
// rank items by power, make powerful items rarer
// powerful items can only be rolled by last place characters


#[derive(Debug, Clone, Default)]
pub struct Player {
    name: String,
    score: i32,
    turn_count: i32,
    evil_items: EvilItems,
    good_items: GoodItems,
}

const TARGET: i32 = 100;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Copy, Default, Ord, PartialOrd)]
pub enum EvilItems {
    /// leech points from another player
    LeechDice,
    /// subtract from everyone elses scores, you gain no points
    EvilDice,
    /// chance to double or quadruple roll
    
    ScoreSwap,
    /// yoink an item from player
    Yoink,
    /// empty
    #[default]
    Nothing,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Copy, Default, Ord, PartialOrd)]
pub enum GoodItems {
    /// roll dice values of 1-10
    MegaDice,
    /// roll 3 dice
    TripleDice,
    /// leech points from another player
    EvenOdd,
    /// trade scores with another player
    #[default]
    Nothing,
}

impl Distribution<EvilItems> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EvilItems {
        match rng.gen_range(0..=4) {
            0 => EvilItems::Yoink,
            1 => EvilItems::LeechDice,
            2 => EvilItems::EvilDice,
            _ => EvilItems::ScoreSwap,
        }
    }
}

impl Distribution<GoodItems> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GoodItems {
        match rng.gen_range(0..=3) {
            0 => GoodItems::MegaDice,
            1 => GoodItems::TripleDice,
            _ => GoodItems::EvenOdd
        }
    }
}

pub fn print_milady() -> () {
    println!("\n\n");
    println!("{}", ("      .*O@@@@@@@@@@#o*.").cyan().dimmed());
    println!("{}", ("   *#@@@@@@@@@@@@@@@@@@#o.").cyan().dimmed());
    println!("{}", ("  O@@@@@@@@@@@@@@@@@@@@@@#°").cyan().dimmed());
    println!("{}", (" *@@@@@@@@@@@@@@@@@@@@@@@@*").cyan().dimmed());
    println!("{}", (" *@@@@@@@@@@@@@@@@@@Oo.@@o").cyan().dimmed());
    println!("{}", (" °#@@@@@@@o   .@@@@@   ##.").cyan().dimmed());
    println!("{}", (" .O@@@@@@@#*  .@@@@@@O#@o").cyan().dimmed());
    println!("{}", ("   .oO@@@@@@@@@@@@@@@@@o").cyan().dimmed());
    println!("{}", ("        .*o#@@@@@@@@#*.").cyan().dimmed());
    println!("{}", ("         *O@@@@@@@@@@O°").cyan().dimmed());
    println!("{}", ("       .#@@@@@@@@@@@@@@*").cyan().dimmed());
    println!("{}", ("       o@@@@@@@@@@@@@@@@°").cyan().dimmed());
    println!("{}", ("      .#@@@@@@@@@@@@@@@@O").cyan().dimmed());
    println!("{}", ("      °@@@@@@@@@@@@@@@@@#°").cyan().dimmed());
}

pub fn print_instructions() -> () {
    println!(
        "\n       {}{}{}{}{}{}{}",
        ("🔥"),
        ("🎲"),
        ("🔥"),
        ("GREED").bright_cyan().blink(),
        ("🔥"),
        ("🎲"),
        ("🔥"),
    );
    println!("{}", ("\n\n\nHOW TO PLAY").bright_cyan().dimmed().bold());
    println!(
        "{}{}{}",
        ("type ").bright_purple().dimmed(),
        ("r").bright_purple().italic(),
        (" to roll the dice").bright_purple().dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").bright_purple().dimmed(),
        ("q").bright_purple().italic(),
        (" to end turn").bright_purple().dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").bright_purple().dimmed(),
        ("s").bright_purple().italic(),
        (" to see scoreboard").bright_purple().dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").bright_purple().dimmed(),
        ("i").bright_purple().italic(),
        (" to check item bag").bright_purple().dimmed()
    );
    println!(
        "{}{}{}",
        ("type ").bright_purple().dimmed(),
        ("c").bright_purple().italic(),
        (" to see commands\n").bright_purple().dimmed()
    );
    println!(
        "{}{}{}{}",
        ("First player to reach ").bright_green().dimmed(),
        TARGET.bold().bright_green(),
        ("pts").bright_green(),
        (" wins.\n").bright_green().dimmed(),
    );
    println!(
        "{}{}",
        ("On your turn, roll the dice as many times as you want,")
            .red()
            .dimmed(),
        (" but").red()
    );
    println!(
        "{}{}{}{}{}{}",
        ("If you roll a ").red().dimmed(),
        ("1").red(),
        (" you get ").red().dimmed(),
        ("0").red(),
        ("pts").red(),
        (" points and your turn is over").red().dimmed()
    );
    println!(
        "{}{}{}",
        ("If you roll ").cyan().dimmed(),
        ("DOUBLES").cyan().italic(),
        (" those dice are doubles in value").cyan().dimmed()
    );
    println!(
        "{}{}{}",
        ("If you roll ").red().dimmed(),
        ("SNAKE EYES").red().italic(),
        (", you lose all of your points").red().dimmed()
    );
    println!("\n{}", ("GOOD LUCK!").bright_green().bold());
    println!("{}", ("DON'T BE GREEDY").dimmed().bright_green().bold());
    println!("{}", ("\nHow many players?").bright_blue());
}

pub fn set_player(name: String) -> Player {
    Player {
        name,
        score: 0,
        turn_count: 0,
        evil_items: EvilItems::default(),
        good_items: GoodItems::default()
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
    let mut players: Vec<Player> = Vec::new();
    // high score
    let mut high_score: Vec<Player> = vec![set_player("Tony".to_string())];

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

        match new_name.trim() {
            "Iman" => println!("{}", ("Hello Pooti").bright_cyan().italic()),
            " " => println!("{}", ("Please Enter a Name").dimmed()),
            _ => (),
        }

        // push each player in
        players.push(player);

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
        thread_rng().gen_range(1..13)
    }

    fn item_roll () -> i32 {
        thread_rng().gen_range(1..101)
    }
    
    // vector of random prompts to spice it up
    let random_prompts: Vec<String> = vec![
        String::from(", ⌚ TIME TO ROLL"),
        String::from(", 😎 IT WOULD BE COOL IF YOU ROLLED"),
        String::from(", 🍀 GOOD LUCK!"),
        String::from(", 🐌 TAKE YOUR TIME"),
    ];

    // random messages after rolling 1s
    let random_ones: Vec<String> = vec![
        String::from("👹👹👹👹👹👹👹👹👹"),
        String::from("🪦🤡"),
        String::from("🤣😹😂😹🤣"),
        String::from("🕷️🪲🪰🦗🪱🦟🪳🐜"),
    ];

    // snake eyes
    // doubles
    let dubs_msg: Vec<String> = vec![
        String::from("🤠🎉DOUBLES🎉🤠"),
        String::from("👽👾🌌🛸🌕🛸🌌👾👽"),
        String::from("🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔🦔"),
        String::from("🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄🦄"),
    ];

    let endgame: Vec<String> = vec![
        (String::from(", GOOD LUCK")),
        (String::from(", DON'T CHOKE")),
        (String::from(", YOU CAN DO IT")),
        (String::from(", DON'T LOSE")),
    ];

    // if i make the same number of prompts for each case
    // then i only need one function
    fn gen_prompt() -> usize {
        thread_rng().gen_range(0..3)
    }

    // create vector of people who get a final turn
    fn last_turns(players: Vec<Player>, turn_count: i32) -> Vec<Player> {
        players
            .into_iter()
            .filter(|p| p.turn_count == turn_count)
            .collect()
    }
    
    println!("{}{}", ("ENABLE ITEMS?").cyan(), (" y / n").cyan().dimmed());
    
    let mut item_toggle = 0;
    loop {
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("cant");
        match response.trim() {
            "y" => {
                item_toggle = 1;
                break}
            "n" => {break}
            _ => println!("{}", ("please answer y / n").cyan().dimmed())
        }
        
    }

    match item_toggle {
        1 => println!("\n{}", ("ITEMS ENABLED").italic().cyan()),
        0 => println!("\n{}", ("items off").italic().cyan()),
        _ => println!("{}", ("wackydacky").italic().cyan()),
    }

    let mut i: usize = 0;
    'game: loop {
        // call gen functions
        let index: usize = gen_prompt();
        // try_into().unwrap() changes p_num from i32 to usize
        let mut turn_scores: Vec<i32> = vec![0; p_num.try_into().unwrap()];
        // roll message
        println!(
            "\n\n{}{}\n",
            players[i]
                .name
                .to_ascii_uppercase()
                .bold()
                .italic()
                .bright_green(),
            random_prompts[index].cyan()
        );

        'turn: loop {
            // intialize roll values
            let r1 = dice_roll();
            let r2 = dice_roll();
            let r3 = dice_roll();
            let m1 = mega_dice_roll();
            let m2 = mega_dice_roll();
            
            let item_picker = item_roll();

            // get input
            let mut keyboard_roll = String::new();
            io::stdin()
                .read_line(&mut keyboard_roll)
                .expect("cant read that");

            // match input values to commands
            
            match (keyboard_roll.trim(), players[i].evil_items, players[i].good_items, item_toggle) {
                ("c", _, _, 1) => {

                }
                ("q", _, _, _) => {
                    println!(
                        "{}{}",
                        players[i].name.bright_blue(),
                        (" ended their turn").italic().dimmed()
                    );
                    println!(
                        "{}{}{}",
                        ("+").italic().bright_green(),
                        turn_scores[i].italic().bright_green(),
                        ("pts").italic().bright_green()
                    );
                    players[i].turn_count += 1;
                    players[i].score += turn_scores[i];
                    break 'turn;
                }
                //auto roll
                ("auto", _, _, _) => {
                    println!("{}", ("set auto roll amount").dimmed().italic());
                    let mut auto_roll = String::new();
                    io::stdin().read_line(&mut auto_roll).expect("cant");
                    let auto_roll = auto_roll.trim();
                    let amount: i32 = auto_roll.parse().unwrap();
                    println!(
                        "{}",
                        ("\ncool thumbs up, we'll get back to you with this feature")
                            .dimmed()
                            .italic()
                    );
                }
                //check items + descriptions
                ("i", _, _, 1) => {
                    println!(
                        "\n{}{}",
                        players[i].name.cyan().dimmed(),
                        ("'s BAG").cyan().dimmed()
                    );
                    print!("{} {:?}", ("evil").red(), players[i].evil_items.bright_red());
                    print!("\n{} {:?}\n", ("good").blue(), players[i].good_items.bright_blue());
                    match (players[i].evil_items, players[i].good_items) {
                        (EvilItems::Nothing, GoodItems::Nothing) => println!("{}", ("1 in 4 chance to earn an item everytime you roll").cyan().dimmed()),
                        (_, GoodItems::MegaDice) => println!(
                            "\n{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("mega\n").bright_magenta(),
                            ("roll two 10-sided dice").cyan().dimmed()
                        ),
                        (EvilItems::LeechDice, _) => println!(
                            "\n{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("leech\n").bright_magenta(),
                            ("dice values are subtracted from the score of a chosen opponent\n& given to the roller").cyan().dimmed()
                        ),
                        (_, GoodItems::EvenOdd)=> println!(
                            "\n{}{}{}{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("even\n").bright_magenta(),
                            ("guess if first roll is even/odd").cyan().dimmed(),
                            ("\nguess if second roll is higher/lower than first roll").cyan().dimmed(),
                            ("\nget 1 right => dice values are doubled").cyan().dimmed(),
                            ("\nget 2 right => dice values are quadrupled").cyan().dimmed()
                        ),
                        (EvilItems::Yoink, _) => println!(
                            "\n{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("yoink\n").bright_magenta(),
                            ("steal another player's item").cyan().dimmed()
                        ),
                        (EvilItems::EvilDice, _) => println!(
                            "\n{}{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("evil\n").bright_magenta(),
                            ("dice values are multiplied by 2").cyan().dimmed(),
                            ("\n& subtracted from all other players' scores").cyan().dimmed()
                        ),
                        (EvilItems::ScoreSwap, _) => println!(
                            "\n{}{}{}\n",
                            ("command ").bright_magenta().dimmed(),
                            ("score swap\n").bright_magenta(),
                            ("trade scores with an opponent").cyan().dimmed()
                        ),
                        (_, GoodItems::TripleDice) => println!(
                            "\n{}{}{}{}{}{}{}",
                            ("command ").bright_magenta().dimmed(),
                            ("triple\n").bright_magenta(),
                            ("roll 3 dice").cyan().dimmed(),
                            ("\nif you roll the same number 3 times, those numbers will be tripled").cyan().dimmed(),
                            ("\nif you roll the same number twice, those numbers are doubled").cyan().dimmed(),
                            ("\nif you roll two 1's, your turn ends and you get no points").cyan().dimmed(),
                            ("\nif you roll three 1's you lose all of your points").cyan().dimmed(),
                        ),
                    }
                    println!("\n{}", ("OTHER ITEMS").bright_magenta().dimmed());

                    let Good_equipped = players[i].good_items;
                    let equipped = players[i].evil_items;
                    for i in EvilItems::iter() {
                        if i != equipped {
                            println!("{:?}", (i).bright_magenta().bold())
                        }
                    }
                    for i in GoodItems::iter() {
                        if i != Good_equipped {
                            println!("{:?}", (i).bright_magenta().bold())
                        }
                    }
                    println!("");
                }
                //items off
                ("i", _, _, _) => {
                    println!("{}", ("ITEMS TURNED OFF").cyan().italic())
                }
                //show scoreboard
                ("s", _, _, _) => {

                    // sort players by score
                    let mut ranking = players.clone();
                    // init last place index as usize
                    let last: usize = ranking.len() - 1;
                    ranking.sort_by(|a, b| b.score.cmp(&a.score));
                    for i in ranking.iter() {
                        match i.score {
                            x if x == ranking[last].score => {
                                    println!(
                                        "{} {} {}", 
                                        (" LAST! ").white().on_red().blink(), 
                                        i.name.to_ascii_uppercase().bright_cyan(),
                                        i.score.bright_green().bold(),
                                    );
                                    
                            }
                            x if x == ranking[0].score => {
                                    println!("{} {} {}", ("1st").yellow().blink(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[1].score => {
                                    println!("{} {} {}", ("2nd").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            
                            x if x == ranking[2].score => {
                                    println!("{} {} {}", ("3rd").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[3].score => {
                                    println!("{} {} {}", ("4th").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[4].score => {
                                    println!("{} {} {}", ("5th").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[5].score => {
                                    println!("{} {} {}", ("6th").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[6].score => {
                                    println!("{} {} {}", ("7th ").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x == ranking[7].score => {
                                    println!("{} {} {}", ("8th").yellow(), i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                            x if x < 0 => {
                                    println!("{} {} {}", i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold(), ("SAD").truecolor(233, 94, 70));
                                    
                            }
                            _ => {
                                    println!("{} {}", i.name.to_ascii_uppercase().bright_cyan(), i.score.bright_green().bold());
                                    
                            }
                        }
                        
                    }
                    println!("\n")
                }
                //normal roll
                ("r", _, _, 1) => {
                    // 🎲🎲 print roll
                    println!(
                        "\n{} + {} = {}\n",
                        r1.red().on_white().bold(),
                        r2.red().on_white().bold(),
                        (r1 + r2).bright_green()
                    );
                    // match dice
                    match (r1, r2) {
                        //snake eyes
                        (1, 1) => {
                            println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                            players[i].score *= 0;
                            println!("{}", ("TOTAL SCORE 0").red());
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        //roll a 1
                        (x, y) if x == 1 || y == 1 => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED A 1!").dimmed());
                            println!("{}", ("TURN COMPLETE").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        // roll doubles
                        (x, y) if x == y => {
                            println!("{}", dubs_msg[index]);
                            turn_scores[i] += r1 * 4;
                            println!(
                                "\n{} x2 = {}🎉",
                                (r1 * 2).green(),
                                (r1 * 4).bright_green().bold().blink()
                            );
                            println!("{} {}", ("TURN SCORE").dimmed(), turn_scores[i].green(),);
                        }
                        
                        //normal roll
                        _ => {
                            turn_scores[i] += r1 + r2;
                            println!("{}{}\n", ("turn score:").dimmed(), turn_scores[i].green(),);
                        }
                    }
                    
                    // sort players by score
                    let mut placement = players.clone();
                    // init last place index as usize
                    let last_place: usize = placement.len() - 1;
                    let second_to_last: usize = placement.len() - 2;
                    placement.sort_by(|a, b| b.score.cmp(&a.score));
                    match (item_picker, players[i].name.to_owned()) {
                        // first place had 1 in four chance to get weak item
                        (76..=100, first) if first == placement[0].name => {
                            // soft items
                            let random_item: GoodItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].good_items = random_item;
                        }
                        // 2nd place has 33% chance to get a good item
                        (66..=100, second) if second == placement[1].name => {
                            // soft items
                            let random_item: GoodItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].good_items = random_item;
                        }

                        // last place has 100% chance of getting item
                        // last place has 50/50 chance to get weak items or strong items
                        (1..=50, last) if last == placement[last_place].name => {
                            // soft items
                            let random_item: GoodItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].good_items = random_item;
                        }

                        (51..=100, last) if last == placement[last_place].name => {
                            // powerful items
                            let random_item: EvilItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].evil_items = random_item;
                        }
                        // 2nd to last has 50% chance to get good item
                        (1..=50, second2last) if second2last == placement[second_to_last].name => {
                            // soft items
                            let random_item: GoodItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].good_items = random_item;
                        }
                        // 2nd to last has 33% chance of getting evil items
                        (66..=100, second2last) if second2last == placement[second_to_last].name => {
                            // powerful items
                            let random_item: EvilItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].evil_items = random_item;
                        }
                        
                        // everyone else
                        // 10% evil / 33% good
                        (1..=10, _) => {
                            // powerful items
                            let random_item: EvilItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].evil_items = random_item;
                        }
                        (66..=100, _) => {
// soft items
                            let random_item: GoodItems = rand::random();
                            println!("{}", ("found item!").italic().dimmed());
                            println!(
                                "{}{}{:?}",
                                players[i].name.to_ascii_uppercase().bright_green().bold(),
                                (" picked up ").bright_cyan(),
                                random_item.bright_magenta().bold()
                            );
                            players[i].good_items = random_item;
                        }

                        _ => {}
                    }
                }
                // no items normal roll
                ("r", _, _, 0) => {
                    // 🎲🎲 print roll
                    println!(
                        "\n{} + {} = {}\n",
                        r1.red().on_white().bold(),
                        r2.red().on_white().bold(),
                        (r1 + r2).bright_green()
                    );
                    // match dice
                    match (r1, r2) {
                        //snake eyes
                        (1, 1) => {
                            println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                            players[i].score *= 0;
                            println!("{}", ("TOTAL SCORE 0").red());
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        //roll a 1
                        (x, y) if x == 1 || y == 1 => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED A 1!").dimmed());
                            println!("{}", ("TURN COMPLETE").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        // roll doubles
                        (x, y) if x == y => {
                            println!("{}", dubs_msg[index]);
                            turn_scores[i] += r1 * 4;
                            println!(
                                "\n{} x2 = {}🎉",
                                (r1 * 2).green(),
                                (r1 * 4).bright_green().bold().blink()
                            );
                            println!("{} {}", ("TURN SCORE").dimmed(), turn_scores[i].green(),);
                        }
                        
                        //normal roll
                        _ => {
                            turn_scores[i] += r1 + r2;
                            println!("{}{}\n", ("turn score:").dimmed(), turn_scores[i].green(),);
                        }
                    }
                    
                    
                }

                //give items
                ("e", _, _, 1) => players[i].evil_items = EvilItems::EvilDice,
                ("eo", _, _, 1) => players[i].good_items = GoodItems::EvenOdd,
                ("l", _, _, 1) => players[i].evil_items = EvilItems::LeechDice,
                ("m", _, _, 1) => players[i].good_items = GoodItems::MegaDice,
                ("ss", _, _, 1) => players[i].evil_items = EvilItems::ScoreSwap,
                ("t", _, _, 1) => players[i].good_items = GoodItems::TripleDice,
                ("y", _, _, 1) => players[i].evil_items = EvilItems::Yoink,

                //dont have items
                ("evil", item, _, 1) if item != EvilItems::EvilDice => {
                    println!("{}{:?}", ("don't have ").dimmed().italic(), EvilItems::EvilDice)
                }
                ("even", _, item, 1) if item != GoodItems::EvenOdd => {
                    println!("{}{:?}", ("don't have ").dimmed().italic(), GoodItems::EvenOdd)
                }
                ("leech", item, _, 1) if item != EvilItems::LeechDice => println!(
                    "{}{:?}",
                    ("don't have ").dimmed().italic(),
                    EvilItems::LeechDice
                ),
                ("mega", _, item, 1) if item != GoodItems::MegaDice => {
                    println!("{}{:?}", ("don't have ").dimmed().italic(), GoodItems::MegaDice)
                }
                ("score swap", item, _, 1) if item != EvilItems::ScoreSwap => println!(
                    "{}{:?}",
                    ("don't have ").dimmed().italic(),
                    EvilItems::ScoreSwap
                ),
                ("triple", _, item, 1) if item != GoodItems::TripleDice => println!(
                    "{}{:?}",
                    ("don't have ").dimmed().italic(),
                    GoodItems::TripleDice
                ),
                ("yoink", item, _, 1) if item != EvilItems::Yoink => {
                    println!("{}{:?}", ("don't have ").dimmed().italic(), EvilItems::Yoink)
                }
                //have item

                // implement all items !!!
                ("triple", _, GoodItems::TripleDice, 1) => {
                    // use tripleDice
                    players[i].good_items = GoodItems::Nothing;
                    // 🎲🎲 print roll
                    println!(
                        "\n{} + {} + {} = {}\n",
                        r1.white().on_green().bold(),
                        r2.white().on_green().bold(),
                        r3.white().on_green().bold(),
                        (r1 + r2 + r3).bright_green()
                    );
                    match (r1, r2, r3) {
                        // triple ones
                        (1, 1, 1) => {
                            print!("{}", ("WOW. UNLUCKY. SAD."));
                            print!(
                                "{}{}",
                                ("\nYou rolled the mythical"),
                                (" THREE EYED SNAKE ")
                            );
                            players[i].score *= 0;
                            print!("{}{}{}", ("you now have "), players[i].score, ("pts"));
                            players[i].turn_count += 1
                        }
                        // two ones
                        (1, 1, _) => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED TWO 1's").dimmed());
                            println!("{}", ("TURN COMPLETE").red().dimmed());
                            println!("{}", ("+0pts").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        // two ones
                        (1, _, 1) => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED TWO 1's").dimmed());
                            println!("{}", ("TURN COMPLETE").red().dimmed());
                            println!("{}", ("+0pts").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        // two ones
                        (_, 1, 1) => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED TWO 1's").dimmed());
                            println!("{}", ("TURN COMPLETE").red().dimmed());
                            println!("{}", ("+0pts").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn;
                        }
                        // triples
                        (x, y, z) if x == y && x == z => {
                            // insert emojis
                            print!("triples");
                            println!(
                                "\n{} x3 = {}🎉",
                                (r1 * 3).green(),
                                (r1 * 9).bright_green().bold().blink()
                            );
                            turn_scores[i] += r1 * 9;
                            println!(
                                "{}{}",
                                ("TURN SCORE").bright_green().dimmed(),
                                turn_scores[i].bright_green()
                            )
                        }
                        // doubles
                        (x, y, _) if x == y => {
                            // insert emojis
                            print!("doubles");
                            println!(
                                "\n{} + {} x2 = {}🎉",
                                r1.green(),
                                r2.green(),
                                (r1 * 4).bright_green().bold().blink()
                            );
                            println!(
                                "{} + {} = {}",
                                (r1 * 4).green(),
                                r3.green(),
                                ((r1 * 4) + r3).green()
                            );
                            turn_scores[i] += (r1 * 4) + r3;
                            println!(
                                "{}{}",
                                ("TURN SCORE ").bright_green().dimmed(),
                                turn_scores[i].bright_green()
                            )
                        }
                        // doubles
                        (x, _, z) if x == z => {
                            // insert emojis
                            print!("doubles");
                            println!(
                                "\n{} + {} x2 = {}🎉",
                                r1.green(),
                                r3.green(),
                                (r1 * 4).bright_green().bold().blink()
                            );
                            println!(
                                "{} + {} = {}",
                                (r1 * 4).green(),
                                r2.green(),
                                ((r1 * 4) + r2).green()
                            );
                            turn_scores[i] += (r1 * 4) + r2;
                            println!(
                                "{}{}",
                                ("TURN SCORE ").bright_green().dimmed(),
                                turn_scores[i].bright_green()
                            )
                        }
                        // doubles
                        (_, y, z) if z == y => {
                            // insert emojis
                            print!("doubles");
                            println!(
                                "\n{} + {} x2 = {}🎉",
                                r2.green(),
                                r3.green(),
                                (r2 * 4).bright_green().bold().blink()
                            );
                            println!(
                                "{} + {} = {}",
                                (r2 * 4).green(),
                                r1.green(),
                                ((r2 * 4) + r1).green()
                            );
                            turn_scores[i] += (r2 * 4) + r1;
                            println!(
                                "{}{}",
                                ("TURN SCORE ").bright_green().dimmed(),
                                turn_scores[i].bright_green()
                            )
                        }
                        (_, _, _) => {
                            turn_scores[i] += r1 + r2 + r3;
                            println!(
                                "{}{}",
                                ("TURN SCORE ").green().dimmed(),
                                turn_scores[i].green()
                            )
                        }
                    }
                }
                ("evil", EvilItems::EvilDice, _, 1) => {
                    println!(
                        "\n{}{}{}",
                        players[i].name.to_ascii_uppercase().on_red().bold(),
                        (" rolled the ").red(),
                        ("EVIL DICE").red().bold().blink()
                    );
                    println!(
                        "\n{} {} {} {} {}\n",
                        r1.white().on_red().bold(),
                        ("+").red().dimmed(),
                        r2.white().on_red().bold(),
                        ("=").red().dimmed(),
                        (r1 + r2).bright_red()
                    );
                    // use item
                    players[i].evil_items = EvilItems::Nothing;
                    let evil_score = r1 + r2 + r1 + r2;
                    println!(
                        "{}{}{}{}{}",
                        ("-").red().bold().blink(),
                        evil_score.red().blink().bold(),
                        ("pts").red().bold().blink(),
                        (" for everyone except ").red(),
                        players[i].name.to_ascii_uppercase().on_red().bold()
                    );
                    turn_scores[i] += evil_score;
                    // pnum - 1 so the index wont go out of bounds

                    let mut e: usize = 0;
                    loop {
                        players[e].score -= evil_score;
                        if e == (p_num - 1).try_into().unwrap() {
                            break;
                        }
                        e += 1
                    }
                }
                ("mega", _, GoodItems::MegaDice, 1) => {
                    {
                        println!(
                            "{}{}{}",
                            players[i].name.to_ascii_uppercase().on_bright_cyan().bold(),
                            (" rolled the ").bright_cyan(),
                            ("MEGA DICE").bright_cyan().bold().blink()
                        );
                        // use MEGADICE
                        players[i].good_items = GoodItems::Nothing;
                        // 🎲🎲 print roll
                        println!(
                            "\n{} + {} = {}\n",
                            m1.blue().on_white().bold(),
                            m2.blue().on_white().bold(),
                            (m1 + m2).bright_green()
                        );
                        match (m1, m2) {
                        //snake eyes
                        (1, 1) => {
                            println!("\n{}", ("  SNAKE EYES  ").on_bright_magenta());
                            players[i].score *= 0;
                            println!("{}", ("TOTAL SCORE 0").red());
                            players[i].turn_count += 1;
                            break 'turn
                            },
                        //roll 1

                        (a, b) if a == 1 || b == 1 => {
                            println!("\n{}", random_ones[index]);
                            println!("{}", ("ROLLED A 1!").dimmed());
                            println!("{}", ("TURN COMPLETE").red());
                            println!("{} {}", ("TOTAL SCORE").blue(), players[i].score);
                            players[i].turn_count += 1;
                            break 'turn
                        }
                        (j, k) if j == k => {
                            println!("{}", dubs_msg[index]);
                            turn_scores[i] += m1 * 4;
                            println!("\n{} x2 = {}🎉", (m1 * 2).green(), (m1 * 4).bright_green().bold().blink());
                            println!(
                                "{} {}",
                                ("TURN SCORE").dimmed(),
                                turn_scores[i].green(),
                            );

                        },
                        (6, 9) => {println!("{}", ("good_items sunglasses emoji"));
                            turn_scores[i] += m1 + m2
                    },
                        (3, 11) => {println!("{}", ("woah amber is the color of your energy")); turn_scores[i] += m1 + m2
                    },
                        (7, 11) => {println!("{}", ("711 bonus! free slurpees for everyone")); turn_scores[i] += m1 + m2},
                        (9, 11) => {println!("{}", ("plane building emojis 911 in rememberance of building 7, each player is awarded +7pts!")); turn_scores[i] += m1 + m2},
                        //normal roll
                        _ => {turn_scores[i] += m1 + m2;
                        println!(
                            "{}{}",
                            ("turn score:").dimmed(),
                            turn_scores[i].green(),
                        );

                        }}
                    }
                }
                ("leech", EvilItems::LeechDice, _, 1) => {
                    // use item
                    players[i].evil_items = EvilItems::Nothing;

                    // ask player for answer
                    // compare that answer with every player name
                    // if it matches then we activate the leech dice
                    // if it doesn't match we tell them to answer with a valid name

                    println!("\n{}\n", ("SELECT A PLAYER TO LEECH POINTS FROM").purple());

                    'outer: loop {
                        // take input
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("cant");
                        let victim = input.trim();

                        // loop to see if input matches any players
                        let mut n = 0;
                        'inner: loop {
                            let cur = players[n].name.to_owned();
                            match victim {
                                // valid leech victim
                                x if x == cur => {
                                    println!(
                                        "\n{}{}{}",
                                        players[i].name.to_ascii_uppercase().purple(),
                                        (" chose ").purple().dimmed(),
                                        cur.to_ascii_uppercase().purple()
                                    );
                                    // ask to multiply
                                    println!(
                                        "\n{}{}\n",
                                        ("ROLL MULTIPLIER? ").purple(),
                                        ("y / n").purple().dimmed().italic()
                                    );
                                    println!(
                                        "{}",
                                        ("roll 1-3 to double the score that you leech")
                                            .purple()
                                            .dimmed()
                                            .italic()
                                    );
                                    println!("{}\n", ("roll 4-6 and the score will be doubled and leeched from you!").purple().dimmed().italic());

                                    loop {
                                        let mut multi = String::new();
                                        io::stdin().read_line(&mut multi).expect("cant");
                                        match multi.trim() {
                                            "y" => {
                                                // 🎲🎲 print roll
                                                println!(
                                                    "\n{} {} {} {} {}\n",
                                                    r1.white().on_purple().bold(),
                                                    ("+").purple(),
                                                    r2.white().on_purple().bold(),
                                                    ("=").purple(),
                                                    (r1 + r2).purple().blink()
                                                );
                                                let pts_stolen = (r1 + r2) * 2;
                                                println!(
                                                    "{}{}",
                                                    ("and the multiplier roll is...").purple(),
                                                    r3.white().on_purple().bold()
                                                );
                                                match r3 {
                                                    x if x >= 4 => {
                                                        println!(
                                                            "{}{}{}{}{}{}\n",
                                                            pts_stolen.purple().bold(),
                                                            ("pts ").purple().bold(),
                                                            ("will be leeched from ")
                                                                .purple()
                                                                .dimmed(),
                                                            players[i].name.to_ascii_uppercase().purple().bold(),
                                                            (" and given to ").purple().dimmed(),
                                                            cur.to_ascii_uppercase().purple().bold()
                                                        );
                                                        players[i].score -= pts_stolen;
                                                        players[n].score += pts_stolen;
                                                    }
                                                    x if x <= 3 => {
                                                        println!(
                                                            "{}{}{}{}{}{}\n",
                                                            pts_stolen.purple().bold(),
                                                            ("pts ").purple().bold(),
                                                            ("will be leeched from ")
                                                                .purple()
                                                                .dimmed(),
                                                            cur.to_ascii_uppercase().purple().bold(),
                                                            (" and given to ").purple().dimmed(),
                                                            players[i].name.to_ascii_uppercase().purple().bold()
                                                        );
                                                        players[i].score += pts_stolen;
                                                        players[n].score -= pts_stolen;
                                                    }
                                                    _ => (),
                                                }
                                                break;
                                            }
                                            "n" => {
                                                // 🎲🎲 print roll
                                                println!(
                                                    "\n{} {} {} {} {}\n",
                                                    r1.white().on_purple().bold(),
                                                    ("+").purple(),
                                                    r2.white().on_purple().bold(),
                                                    ("=").purple(),
                                                    (r1 + r2).purple().blink()
                                                );
                                                println!(
                                                    "{}",
                                                    ("no multiplier").purple().dimmed().italic()
                                                );
                                                println!(
                                                    "{}{}{}{}{}{}\n",
                                                    players[i]
                                                        .name
                                                        .to_ascii_uppercase()
                                                        .purple()
                                                        .bold(),
                                                    (" leeched ").purple(),
                                                    (r1 + r2).purple().bold().blink(),
                                                    ("pts").purple().bold().blink(),
                                                    (" from ").purple(),
                                                    cur.to_ascii_uppercase().purple().bold()
                                                );
                                                players[i].score += r1 + r2;
                                                players[n].score -= r1 + r2;
                                                break;
                                            }
                                            _ => println!(
                                                "{}",
                                                ("please make a decision! y / n")
                                                    .purple()
                                                    .dimmed()
                                                    .italic()
                                            ),
                                        }
                                    }
                                    break 'outer;
                                }
                                _ => n += 1,
                            }

                            if n == p_num.try_into().unwrap() {
                                println!(
                                    "{}",
                                    ("please choose a valid victim").dimmed().italic().purple()
                                );
                                break 'inner;
                            }
                        }// inner end
                    } /* end loop */
                }
                // add emojis in vim!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                ("even", _, GoodItems::EvenOdd, 1) => {
                    let mut pts: i32 = 0;
                    println!(
                        "{}",
                        (" Lets Play Even/Odd High/Low ").bright_yellow().on_cyan()
                    );

                    // first roll
                    println!("\n{}", ("first roll").italic().dimmed());
                    println!("{}", ("EVEN or ODD?").cyan());
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("cant");
                    println!(
                        "{}{}",
                        ("first roll is ").cyan(),
                        r1.bright_yellow().on_cyan()
                    );
                    match (answer.trim(), r1) {
                        ("even", _) => {
                            if r1 % 2 == 0 {
                                pts += 1;
                                println!(
                                    "{} {}",
                                    ("Correct!").bright_blue(),
                                    ("+1").bright_green().bold()
                                );
                            } else if r1 % 2 != 0 {
                                println!("wrong")
                            }
                        }
                        ("odd", _) => {
                            if r1 % 2 != 0 {
                                pts += 1;
                                println!(
                                    "{} {}",
                                    ("Correct!").bright_blue(),
                                    ("+1").bright_green().bold()
                                );
                                println!("correct")
                            } else if r1 % 2 == 0 {
                                println!("wrong")
                            }
                        }
                        _ => println!("error"),
                    }
                    // second roll
                    println!("\n{}", ("second roll").italic().dimmed());
                    println!("{}", ("HIGH or LOW?").cyan());
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer).expect("cant");
                    println!(
                        "{}{}",
                        ("second roll is ").cyan(),
                        r2.bright_yellow().on_cyan()
                    );
                    match (answer.trim(), r2) {
                        ("high", _) => {
                            if r2 > r1 {
                                pts += 1;
                                println!(
                                    "{} {}",
                                    ("Correct!").bright_blue(),
                                    ("+1").bright_green().bold()
                                );
                                println!("correct")
                            } else if r2 < r1 {
                                println!("wrong")
                            }
                        }
                        ("low", _) => {
                            if r2 < r1 {
                                pts += 1;
                                println!(
                                    "{} {}",
                                    ("Correct!").bright_blue(),
                                    ("+1").bright_green().bold()
                                );
                                println!("correct")
                            } else if r2 > r1 {
                                println!("wrong")
                            }
                        }
                        _ => println!("error"),
                    }
                    println!("{} {}", ("correct answers").blue(), pts.bright_cyan());
                    let multi: i32 = (pts * 2) * (r1 + r2);
                    println!(
                        "{} {}",
                        (" YOU EARNED ").bright_yellow().on_bright_blue(),
                        multi.bright_green().blink_fast().bold()
                    );
                    turn_scores[i] += multi;
                    players[i].good_items = GoodItems::Nothing;
                    println!("{}", ("Keep Rolling\n").dimmed().italic())
                }
                ("swap", EvilItems::ScoreSwap, _, 1) => {
                    println!("{}", (" UNDER CONSTRUCTION check back later ").on_bright_red().bold());
                    // use item
                    players[i].evil_items = EvilItems::Nothing;
                    // println!("{}", (" CHOOSE A PLAYER TO SWAP SCORES WITH ").black().on_truecolor(255, 95, 31));
                    
                    // 'outer: loop {
                    //     // take input
                    //     let mut input = String::new();
                    //     io::stdin().read_line(&mut input).expect("cannot");
                    //     let swap_victim = input.trim();

                    //     //see if input matches any player names
                    //     let mut s = 0;
                    //     'inner: loop {
                    //         // init variable to hold current player
                    //         let cur = players[s].name.to_owned();
                    //         match swap_victim {
                    //             // valid victim
                    //             x if x == cur => {
                    //                 println!("{}{}{}", players[i].name.to_ascii_uppercase().truecolor(255, 95, 23).bold(), (" swapped scores with ").truecolor(255, 95, 23).dimmed(), players[s].name.to_ascii_uppercase().truecolor(255, 95, 23).bold());
                    //                 println!("{}", ("check scoreboard :)"));

                    //                 //swap scores | score swap
                    //                 let (mut a, mut b) = (players[i].score, players[s].score);

                    //                 mem::swap(&mut a, &mut b);
                    //                 println!("{}{}{}{}", players[i].name, (" now has "), players[i].score, ("pts"));
                    //                 println!("{}{}{}{}", players[s].name, (" now has "), players[s].score, ("pts"));
                                    
                    //                 break 'outer
                    //             }
                    //             _ => s += 1,
                    //         }
                    //         if s == p_num.try_into().unwrap() {
                    //             println!("{}", ("please choose valid target").dimmed().italic().truecolor(255, 95, 23));
                    //             break 'inner;
                    //         }
                            
                            
                    //     } // inner end
                        

                    // } // outer end
                }
                ("yoink", EvilItems::Yoink, _, 1) => {
                    println!("{}{}", (" UNDER CONSTRUCTION ").white().on_yellow().bold(), (" check back later").dimmed().italic());
                    players[i].evil_items = EvilItems::Nothing;
                    // println!("{}{}{}", players[i].name.to_ascii_uppercase().on_truecolor(251, 72, 196).bold(), (" used ").truecolor(251, 72, 196), ("YOINK").truecolor(251, 72, 196).bold().blink());
                    // println!("{}", (" SELECT A PLAYER TO STEAL AN ITEM FROM ").on_truecolor(251, 72, 196).bold());
                    // 'outer: loop {
                    //     let mut input = String::new();
                    //     io::stdin().read_line(&mut input).expect("cant");
                    //     let target = input.trim();

                    //     let mut y = 0;
                    //     'target: loop {
                    //         let cur = players[y].name.to_owned();
                    //         let cur_item = players[y].items.to_owned();
                    //         match (target, cur_item) {
                    //             ("none", _) => {println!("{}", ("no one was yoinked").dimmed().italic().truecolor(251, 72, 196));break 'outer}
                    //             (x, Items::Nothing) if x == cur => {
                    //                 println!("{}", ("no items to steal"));
                    //                 break 'target
                    //             }
                    //             (x, _) if x == cur => {
                    //                 println!("get yoinked son");
                    //                 break 'outer
                    //             }
                    //             _ => y += 1
                    //         }
                    //         if y == p_num.try_into().unwrap() {
                    //             println!("{}", ("please choose valid target or 'none'").dimmed().italic());
                    //             break 'target
                    //         }
                    //     } //end target loop
                    // } //end loop
                }
                (_, _, _, _) => println!("{}", ("invalid command").dimmed().italic()),
            }
        }

        // end of turn loop

        // check if anyone has won

        // temp changed to 30 so i can test the end game in terminal

        // if last player wins then this loop is unessecary
        if players[i].score >= TARGET {
            println!("\nCONGRATS {}!", (players[i].name).bright_green());
            println!("\n🏆🥇{}🥇🏆", (" YOU WON ").yellow().blink());
            println!("{}{}{}", ("with ").yellow().dimmed(), players[i].score.yellow(), ("pts").yellow());

            // if it is the last player the game ends
            // beacuse everyone has had an equal number of turns
            if i == (p_num - 1).try_into().unwrap() {
                break 'game;
            } else {
                println!("\n{}\n", ("...almost!").bright_magenta());
                println!(
                    "{}{}{}",
                    ("You won in ").bright_cyan().dimmed(),
                    (players[i].turn_count).bright_cyan(),
                    (" turns.").bright_cyan().dimmed()
                );
                println!(
                    "{}{}{}",
                    ("all players who have not had ").bright_cyan().dimmed(),
                    (players[i].turn_count).bright_cyan(),
                    (" turns get one last chance to beat your score!")
                        .bright_cyan()
                        .dimmed()
                );

                // create a new vector for the final players
                let total_turns_minus_one = players[i].turn_count - 1;
                let cloned_players = players.clone();
                let mut final_round_players = last_turns(cloned_players, total_turns_minus_one);
                for i in final_round_players.iter() {
                    println!("{}", i.name.trim().bright_red())
                }

                // create a vec of the player w the highest score
                let mut high_scores: Vec<Player> = vec![players[i].clone()];
                let mut v: usize = 0;
                loop {
                    'final_turn: loop {
                        let pts_away = high_scores[0].score - final_round_players[v].score;

                        // if a player surpasses the current highscore
                        // it the old high score is popped out of the vec
                        // and the new high score is pushed in
                        if final_round_players[v].score > high_scores[0].score {
                            println!(
                                "{} you've passed {}'s score",
                                final_round_players[v].name.trim().bold().on_cyan(),
                                players[i].name.trim().bold().on_bright_magenta()
                            );
                            high_scores.pop();
                            high_scores.push(final_round_players[v].clone());
                            println!("You set the new high score!");
                        }

                        // prompt roll / show how many points away they are
                        println!(
                            "\n\n{}{}\n{}{}{}{}",
                            final_round_players[v].name.bright_green().bold(),
                            endgame[index].blue().bold(),
                            ("you are only ").blue().dimmed(),
                            pts_away.bright_red(),
                            ("pts").bright_red(),
                            (" away").blue().dimmed()
                        );

                        let r1 = dice_roll();
                        let r2 = dice_roll();
                        //use normal turn functions!
                        let mut endroll = String::new();
                        io::stdin().read_line(&mut endroll).expect("cant read");

                        match endroll.trim() {
                            "r" => {
                                println!(
                                    "\n\n{} + {} = {}",
                                    r1.red().on_white().bold(),
                                    r2.red().on_white().bold(),
                                    (r1 + r2).bright_green()
                                );
                                match (r1, r2) {
                                    (x, y) if x == 1 || y == 1 => {
                                        println!("{}", ("YOU LOSE"));
                                        v += 1;
                                        break 'final_turn;
                                    }
                                    (x, y) if x == 1 && y == 1 => {
                                        println!(
                                            "{}",
                                            ("of all the times you could rolled SNAKE EYES")
                                                .dimmed()
                                                .italic()
                                        );
                                        v += 1;
                                        break 'final_turn;
                                    }
                                    (x, y) if x == y => {
                                        println!("{}", dubs_msg[index]);
                                        println!(
                                            "\n{} x2 = {}🎉",
                                            (r1 * 2).green(),
                                            (r1 * 4).bright_green().bold().blink()
                                        );
                                        final_round_players[v].score += r1 * 4;
                                        continue 'final_turn;
                                    }
                                    (_, _) => {
                                        final_round_players[v].score += r1 + r2;
                                        continue 'final_turn;
                                    }
                                }
                            }
                            "q" => println!("{}", ("cannot quit final round").italic().dimmed()),
                            "even" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "evil" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "mega" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "triple" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "yoink" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "leech" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            "swap" => {
                                println!("{}", ("cannot use item on final round").dimmed().italic())
                            }
                            _ => println!(" "),
                        }
                        if endroll.trim().contains("roll") {
                            let roll1 = dice_roll();
                            let roll2 = dice_roll();
                            // 🎲🎲 print roll

                            if roll1 == roll2 {
                                println!("fuck yeah lets fucking go thats good thats real good keep doing that");
                                final_round_players[v].score += roll1 * 4;
                                println!("keep rolling? \ny or n");
                                let mut yon = String::new();
                                io::stdin()
                                    .read_line(&mut yon)
                                    .expect("error can't read that");
                                let noy: bool = yon.contains("y");
                                if noy {
                                    continue 'final_turn;
                                } else {
                                    v += 1;
                                    break 'final_turn;
                                }
                                // remember to let players win on last round
                            } else {
                                final_round_players[v].score += roll1 + roll2;
                                if final_round_players[v].score > players[i].score {
                                    println!(
                                        "{} you have surpassed {}'s score",
                                        final_round_players[v].name.trim().bold().on_cyan(),
                                        players[i].name.trim().bold().on_bright_magenta()
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

        // check high score
        match (players[i].score, high_score[0].score) {
            (x, y) if x > y => {
                high_score.pop();
                high_score.push(players[i].clone());
                println!(
                    "{}{}{}",
                    players[i].name.bright_cyan(),
                    (" has set the new high score at ").cyan().dimmed(),
                    players[i].score.bright_cyan().bold()
                )
            }
            _ => (),
        }

        // turn loop goes to next player's turn
        // unless the index = number of players - 1
        // then it resets: i *= 0
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
