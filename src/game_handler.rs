//! Moderates the game between IA's

mod renderer;
pub mod game;
extern crate image;

use renderer::Renderer;
use game::{Game, Action, Town, Player, Team};
use image::{RgbaImage, open};
use image::imageops::overlay;

type Ia<'a> = &'a dyn Fn(Vec<Action>, &Vec<Town>, &Player, &Player) -> Action;

pub struct GameHandler {
    town_spr: image::DynamicImage,
    renderer: Renderer,
    game: Game,
    ia: Ia<'static>,
}

impl GameHandler {
    pub fn new(ia: Ia<'static>) -> GameHandler {
        println!("Initializing renderer [1/2]");
        let renderer = Renderer::init();
        println!("Initializing game instance [2/2]");
        let game = Game::init();

        let town_spr = match open("src/assets/town.png") {
            Ok(spr) => spr,
            Err(_) => panic!()
        };

        GameHandler {
            town_spr: town_spr,
            renderer: renderer,
            game: game,
            ia: ia,
        }
    }

    fn render(&mut self, round: usize) -> () {
        self.renderer.render(
            self.game.get_towns_id(Team::Blue),
            self.game.get_capital_id(Team::Blue),
            self.game.get_towns_id(Team::Red),
            self.game.get_capital_id(Team::Red),
        );

        if let Some(mut image) = RgbaImage::from_raw(1000, 1000, self.renderer.im_buff.clone()) {
            for i in 0..(4*4) {
                let t = self.renderer.towns.get_town(i as usize);
                let pt = self.renderer.cache.get_pixel(t.x, t.y);
                overlay(&mut image, &self.town_spr, (pt.x-25) as i64, (pt.y-25) as i64);
            }
            let _res = image.save(format!("src/assets/{round}.png"));
        }
    }

    pub fn simulate(&mut self) -> () {
        self.game.print_map(); // Debugging purposes
        loop {
            let team = self.game.team;
            let round = self.game.rounds;

            let actions = self.game.get_available_actions(team);
            let action = (self.ia)(
                actions,
                self.game.get_towns(),
                self.game.get_player(team),
                self.game.get_player(team.rival()),
            );
            self.game.do_action(action, team);
            if self.game.rounds != round {

                println!("Round {}, team {}", round, team); // Debug
                self.game.print_stats(team);                // purposes

                // Render frame
                self.render(round);
            }

            // Win condition
            if self.game.over { break; }
        }
    }
}
