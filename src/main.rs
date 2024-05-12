//! Testing purpose CLI

mod game;
mod renderer;

use game::{Action, Game, Perk, Team};
use image::imageops::overlay;
use image::{open, RgbaImage};
use renderer::Renderer;

fn print_action(action: Action) -> () {
    match action {
        Action::Pass => println!("You can pass the turn."),
        Action::Convert(town) => println!("You can convert town {}", town.id),
        Action::ChangeCapitalTo(town) => println!("You can change capital to town {}", town.id),
        Action::AddSpecialtyPoint(perk) => println!(
            "You can add a point to the perk {}",
            match perk {
                Perk::Flagellation => "Flagellation",
                Perk::Communion => "Communion",
                Perk::Display => "Display",
            }
        ),
        Action::BuySpecialtyPoint => println!("You can buy a specialty point"),
        Action::Capture(town) => println!("You can capture town {}", town.id),
        Action::BuyGold => println!("You can trade influence for gold"),
        Action::BuyInfluence => println!("You can trade gold for influence"),
    }
}

fn parse_action(args: Vec<&str>, actions: &Vec<Action>, game: &mut Game, team: Team) {
    let n = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    game.do_action(actions[n - 1], team);
}

fn main() {
    let mut renderer = Renderer::init();

    let mut game = Game::init();

    let town_spr = match open("src/assets/town.png") {
        Ok(spr) => spr,
        Err(_) => panic!(),
    };

    println!("It\'s BLUE\'s turn. Round 0.");
    loop {
        // Game handling
        let team = game.team;
        let rounds = game.rounds;
        let actions = game.get_available_actions(team);

        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("failed to read input");
        let args: Vec<&str> = command.as_str().split_whitespace().collect();
        let args = if args.len() > 0 { args } else { vec![""] };
        match args[0] {
            "action" => parse_action(args, &actions, &mut game, team),
            "map" => game.print_map(),
            "stats" => game.print_stats(team),
            "quit" => break,
            "help" => {
                for action in actions {
                    print_action(action)
                }
            }
            "" => println!("Type a command, type help to see all available actions."),
            _ => println!("Bad command, type help to see all available actions."),
        }
        if rounds != game.rounds {
            println!("It\'s {}\'s turn. Round {}.", game.team, game.rounds);

            // Render frame every turn
            renderer.render(
                game.get_towns_id(Team::Blue),
                game.get_capital_id(Team::Blue),
                game.get_towns_id(Team::Red),
                game.get_capital_id(Team::Red),
            );

            if let Some(mut image) = RgbaImage::from_raw(1000, 1000, renderer.im_buff.clone()) {
                for i in 0..(4 * 4) {
                    let t = renderer.towns.get_town(i as usize);
                    let pt = renderer.cache.get_pixel(t.x, t.y);
                    overlay(
                        &mut image,
                        &town_spr,
                        (pt.x - 25) as i64,
                        (pt.y - 25) as i64,
                    );
                }
                let _res = image.save("src/assets/out.png");
            }
        }

        if game.over {
            break;
        }
    }
}
