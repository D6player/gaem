use rand::prelude::*;

const INFLUENCE_PRICE: i32 = 55;
const GOLD_PRICE: i32 = 5;
fn specialty_point_cost(i: i32) -> i32 {
    100 * (i + 1).pow(2) + 150
}

/// Game instance.
pub struct Game {
    b_player: Player,
    r_player: Player,
    towns: Vec<Town>,
    pub rng: ThreadRng,

    pub over: bool,
    pub team: Team,
    pub rounds: usize,
}

impl Game {
    pub fn init() -> Game {
        let mut rng = rand::thread_rng();

        let mut towns: Vec<Town> = vec![];
        for id in 0..16usize {
            towns.push(Town::new(&mut rng, id));
        }

        Game {
            b_player: Player::new(Team::Blue, &towns),
            r_player: Player::new(Team::Red, &towns),
            towns: towns,
            rng: rng,
            
            over: false,
            team: Team::Blue,
            rounds: 0,
        }
    }

    pub fn get_towns_id(&self, team: Team) -> Vec<usize> {
        match team {
            Team::Blue => self.b_player.get_towns_id(),
            Team::Red => self.r_player.get_towns_id(),
        }
    }

    pub fn get_capital_id(&self, team: Team) -> usize {
        match team {
            Team::Blue => self.b_player.get_capital_id(),
            Team::Red => self.r_player.get_capital_id(),
        }
    }

    fn pass_turn(&mut self) -> () {
        // Win condition
        if !(self.b_player.towns.contains(&self.b_player.capital))
        || !(self.r_player.towns.contains(&self.r_player.capital))
        {
            println!("GG\'s");
            self.over = true;
        }

        // Pass turn normally otherwise
        match self.team {
            Team::Blue => self.b_player.end_turn(),
            Team::Red => self.r_player.end_turn(),
        }

        self.team = self.team.rival();
        self.rounds += 1;
    }

    pub fn print_map(&self) -> () {
        for x in 0..4 {
            let mut v = [("", 0, 0); 4];
            for i in 0..4 {
                v[i] = (
                    match self.towns[x*4 + i].perk {
                        Perk::Flagellation => "F",
                        Perk::Communion => "C",
                        Perk::Display => "D",
                    },
                    self.towns[x*4 + i].grade,
                    self.towns[x*4 + i].size,
                );
            }
            println!("{:?}", v);
        }
    }

    pub fn print_stats(&self, team: Team) -> () {
        let stats: (i32, i32, i32, [i32; 3]) = match team {
            Team::Blue => self.b_player.get_stats(),
            Team::Red => self.r_player.get_stats(),
        };

        println!("gold: {}. Influence: {}. Available: {}. Points: {:?}", stats.0, stats.1, stats.2, stats.3);
    }

    pub fn get_available_actions(&self, player: Team) -> Vec<Action> {
        match player {
            Team::Blue => self.b_player.get_available_actions(&self.towns, &self.r_player),
            Team::Red => self.r_player.get_available_actions(&self.towns, &self.b_player),
        }
    }

    pub fn do_action(&mut self, action: Action, team: Team) -> () {
        if match team {
            Team::Blue => self.b_player.do_action(action, &mut self.r_player, &mut self.rng),
            Team::Red => self.r_player.do_action(action, &mut self.b_player, &mut self.rng),
        } { self.pass_turn() }
    }
}

/// Player instance.
struct Player {
    gold: i32,
    influence: i32,
    specialty: [i32; 3],
    available_specialty_points: i32,
    towns: Vec<Town>,
    capital: Town,

    zealous: i32,
    specialty_points_bought: i32,
}

impl Player {
    pub fn new(team: Team, towns: &Vec<Town>) -> Player {
        let starting_town = match team {
            Team::Blue => towns[12],
            Team::Red => towns[3],
        };

        Player {
            gold: 0,
            influence: 0,
            specialty: [0; 3],
            available_specialty_points: 3,
            towns: vec![starting_town],
            capital: starting_town,

            zealous: -1,
            specialty_points_bought: 0,
        }
    }

    pub fn get_stats(&self) -> (i32, i32, i32, [i32; 3]) {
        (
            self.gold,
            self.influence,
            self.available_specialty_points,
            self.specialty
        )
    }

    fn get_specialty_index(perk: Perk) -> usize {
        match perk {
            Perk::Flagellation => 0,
            Perk::Communion => 1,
            Perk::Display => 2,
        }
    }

    fn get_specialty_points(&self, perk: Perk) -> i32 {
        self.specialty[Player::get_specialty_index(perk)]
    }

    pub fn get_towns_id(&self) -> Vec<usize> {
        let mut ids = vec![];
        for t in &self.towns {
            ids.push(t.id);
        }

        return ids;
    }

    pub fn get_capital_id(&self) -> usize {
        self.capital.id
    }

    fn get_neighbors(&self, towns: &Vec<Town>) -> Vec<Town> {
        let mut neighbors = vec![];
        for town in &self.towns {
            for neighbor_id in town.get_neighbors() {
                let neighbor = towns[neighbor_id];
                if !(neighbors.contains(&neighbor)) && !(self.towns.contains(&neighbor)) {neighbors.push(neighbor)}
            }
        }

        return neighbors;
    }

    fn can_aford(&self, price: i32, resource: Resource) -> bool {
        price <= 
        match resource {
            Resource::Gold => self.gold,
            Resource::Influence => self.influence,
            Resource::SpecialtyPoint => self.available_specialty_points,
        }
    }

    /// Stuff that happens at the end of every turn
    fn end_turn(&mut self) -> () {
        let mut gains: (i32, i32) = (0, 0);
        for town in &self.towns {
            gains.0 += self.capital.size * town.size * 150;
            gains.1 += self.capital.grade * town.grade * self.get_specialty_points(town.perk);
        }
        self.gold += gains.0;
        self.influence += gains.1 - if self.zealous == 0 {170} else {0};
        self.zealous -= 1;
    }

    /// Returns a list of actions a player is able to do.
    pub fn get_available_actions(&self, towns: &Vec<Town>, rival: &Player) -> Vec<Action> {
        // Empty list of actions.
        let mut actions: Vec<Action> = vec![];

        // Action::Convert(Town)
        // Action::Capture(Town)
        for neighbor in self.get_neighbors(towns) {
            if rival.towns.contains(&neighbor) {
                actions.push(Action::Capture(neighbor));
            } else if neighbor.grade <= self.get_specialty_points(neighbor.perk) {
                actions.push(Action::Convert(neighbor));
            }
        }

        // Action::ChangeCapitalTo(Town)
        for town in &self.towns {
            if *town != self.capital {
                actions.push(Action::ChangeCapitalTo(*town));
            }
        }

        // Action::AddSpecialtyPoint(Perk)
        if self.can_aford(1, Resource::SpecialtyPoint) {
            actions.push(Action::AddSpecialtyPoint(Perk::Flagellation));
            actions.push(Action::AddSpecialtyPoint(Perk::Communion));
            actions.push(Action::AddSpecialtyPoint(Perk::Display));
        }

        // Action::BuySpecialtyPoint
        let price = specialty_point_cost(self.specialty_points_bought);
        if self.can_aford(price, Resource::Gold) {
            actions.push(Action::BuySpecialtyPoint);
        }

        // Action::BuyInfluence
        if self.can_aford(INFLUENCE_PRICE, Resource::Gold) {
            actions.push(Action::BuyInfluence);
        }

        // Action::BuyGold
        if self.can_aford(GOLD_PRICE, Resource::Influence) {
            actions.push(Action::BuyGold);
        }

        // Action::Pass
        actions.push(Action::Pass);

        return actions;
    }

    /// Makes a player do and action.
    pub fn do_action(&mut self, action: Action, rival: &mut Player, rng: &mut ThreadRng) -> bool {
        match action {
            Action::Pass => true,
            Action::Convert(town) => self.convert(town),
            Action::ChangeCapitalTo(town) => self.change_capital_to(town),
            Action::AddSpecialtyPoint(perk) => self.add_specialty_point(perk),
            Action::BuySpecialtyPoint => self.buy_specialty_point(),
            Action::Capture(town) => self.capture(town, rival, rng),
            Action::BuyInfluence => self.buy_influence(),
            Action::BuyGold => self.buy_gold(),
        }
    }

    fn convert(&mut self, town: Town) -> bool {
        self.towns.push(town);

        true
    }

    fn change_capital_to(&mut self, town: Town) -> bool {
        self.capital = town;

        false
    }

    fn add_specialty_point(&mut self, perk: Perk) -> bool {
        self.specialty[Player::get_specialty_index(perk)] += 1;
        self.available_specialty_points -= 1;

        false
    }

    fn buy_specialty_point(&mut self) -> bool {
        let price = specialty_point_cost(self.specialty_points_bought);
        self.available_specialty_points += 1;
        self.gold -= price;

        false
    }

    fn capture(&mut self, town: Town, rival: &mut Player, rng: &mut ThreadRng) -> bool {
        let c = |g: f32, i: f32| -> f32 {
            let g = g * 11.0;
            let i = (i+0.001) * 450.0;

            1.0 - ((g/i) + 1.0).powf(-1.0)
        };
        let d6: f32 = rng.gen();

        if d6 < c(self.gold as f32, rival.influence as f32) {
            rival.towns.retain(|&t| t != town);
            self.towns.push(town);

            rival.influence = 200;
            rival.zealous = 2;
        }

        self.gold -= rival.influence * 11 / 3 * 150;
        self.gold = if self.gold < 0 {0} else {self.gold};

        true
    }

    fn buy_influence(&mut self) -> bool {
        self.gold -= INFLUENCE_PRICE;
        self.influence += 1;

        false
    }

    fn buy_gold(&mut self) -> bool {
        self.influence -= GOLD_PRICE;
        self.gold += 190;
        
        false
    }
}

/// Town instance.
#[derive(Copy, Clone)]
pub struct Town {
    perk: Perk,
    grade: i32,
    size: i32,
    pub id: usize,
}

impl PartialEq for Town {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Town {
    pub fn new(rng: &mut ThreadRng, id: usize) -> Town {
        let mut p = [Perk::Flagellation, Perk::Communion, Perk::Display];
        let mut s = [1, 1, 1, 2, 2, 3];
        let mut g = [1, 1, 2];
        s.shuffle(rng);
        g.shuffle(rng);
        p.shuffle(rng);

        Town {
            perk: p[0],
            size: s[0],
            grade: g[0],
            id: id,
        }
    }

    pub fn get_neighbors(&self) -> Vec<usize> {
        let mut neighbors = vec![];
        let id = self.id;
        if id%4 != 0 {neighbors.push(id-1)}
        if id%4 != 3 {neighbors.push(id+1)}
        if !(id < 4) {neighbors.push(id-4)}
        if !(id > 11) {neighbors.push(id+4)}

        return neighbors;
    }
}

// GLOBAL ENUMS

#[derive(Copy, Clone)]
pub enum Team {
    Red,
    Blue,
}

enum Resource {
    Gold,
    Influence,
    SpecialtyPoint,
}

/// Critical enum. Handles actions.
#[derive(Copy, Clone)]
pub enum Action {
    Pass,
    Convert(Town),
    ChangeCapitalTo(Town),
    AddSpecialtyPoint(Perk),
    BuySpecialtyPoint,
    Capture(Town),
    BuyInfluence,
    BuyGold,
    //TODO
}

#[derive(Copy, Clone, PartialEq)]
pub enum Perk {
    Flagellation,
    Communion,
    Display,
}

// misc
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Team::Blue => "BLUE",
            Team::Red => "RED",
        })
    }
}

impl Team {
    pub fn rival(&self) -> Team {
        match self {
            Team::Blue => Team::Red,
            _ => Team::Blue,
        }
    }
}
