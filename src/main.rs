mod game_handler;

use game_handler::GameHandler;
use game_handler::game::{Player, Town, Action};

type Score = i32;

const AN_AWFUL_LOT: i32 = 99999999;
const TURNS_AHEAD: i32 = 4;

fn max<T: Ord>(a: T, b: T) -> T {
    if a > b {a} else {b}
}

fn max_out_of_list<T: Ord + Copy>(array: &Vec<T>) -> T {
    let mut greatest_value = array[0];
    for value in array {
        greatest_value = max(greatest_value, *value);
    }

    greatest_value
}

/// Pure function, static. Gets a list of actions, returns most profitable action.
fn ia(mut actions: Vec<Action>, towns: &Vec<Town>, player: &Player, rival: &Player) -> Action {
    let sorting_key = |action: &Action| {
        if let Action::Capture(_town) = action {
            let c = |g: f32, i: f32| -> f32 {
                let g = g * 11.0;
                let i = (i+0.001) * 250.0;
    
                1.0 - ((g/i) + 1.0).powf(-1.0)
            };
            let chance = c(player.gold as f32, rival.influence as f32);
            if chance >= 0.5 { AN_AWFUL_LOT } else { 0 }
        } else {
            get_score(action, towns, player, rival, TURNS_AHEAD)
        }
    };

    // Pick the action with the highest score
    actions.sort_by_cached_key(sorting_key);
    actions[actions.len()-1]
}

fn get_score(action: &Action, _towns: &Vec<Town>, player: &Player, _rival: &Player, turns: i32) -> Score {
    let mut player2 = player.clone();
    player2.do_action2(*action);
    player2.end_turn();
    if turns == 0 {
        get_player_score(&player2)
    } else if let Action::Capture(_) = action {
        0
    } else {
        let mut scores = vec![];
        for action in player2.get_available_actions(_towns, _rival) {
            scores.push(get_score(&action, _towns, &player2, _rival, turns-1));
        }
        max_out_of_list(&scores)
    }
}

fn get_player_score(player: &Player) -> i32 {
    ((player.gold as f32 / 150.0 + player.influence as f32 * 3.0 / 11.0) * 1000.0) as i32
}

fn main() {
    let mut gh = GameHandler::new(&ia);
    gh.simulate();
}
