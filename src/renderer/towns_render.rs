extern crate rand;

use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
pub struct TownR {
    pub x: i32,
    pub y: i32
}

pub struct TownsR {
    arr: Vec<TownR>,
    _b_frontier: Vec<usize>
}

impl TownsR {
   pub fn gen() -> TownsR {
        let mut towns = TownsR {
            arr: vec![],
            _b_frontier: vec![]
        };

        let mut rng = thread_rng();
        for i in 0..(4*4) {
            let x: f32 = rng.gen();
            let y: f32 = rng.gen();
            let x = (x*150.0) as i32;
            let y = (y*150.0) as i32;

            let x0 = (i%4)*250;
            let y0 = (i/4)*250;

            towns.arr.push(TownR {
                x: x0 + x + 50,
                y: y0 + y + 50,
            });
        }

        return towns;
    }

    pub fn get_town(&self, i: usize) -> TownR {
        self.arr[i]
    }
}
