mod cache;
mod towns_render;

use towns_render::TownsR;
use cache::Cache;
use perlin2d::PerlinNoise2D;
use glm::{mat4, vec3};
use glm::ext::{rotate, scale};
use std::f32::consts::{FRAC_PI_4, PI};
use geo::{LineString, Polygon, ConvexHull};

type Color = [u8; 4];
const BLUE: Color = [0, 0, 255, 255];
const RED: Color = [255, 0, 0, 255];

pub struct Renderer {
    pub cache: Cache, // TEMPORALLY PUBLIC
    pub towns: TownsR, // TEMPORAL
    pub im_buff: Vec<u8>,
}

impl Renderer {
    fn set_pixel(&mut self, x: i32, y: i32, c: Color) {
        if x < 1000 && x > 0 && y < 1000 && y > 0 {
            let i0 = ((x + y*1000)*4) as usize;
            for i in 0..4usize {
                self.im_buff[i0 + i] = c[i];
            }
        }
    }

    fn draw_circle(&mut self, x0: i32, y0: i32, r: i32, col: Color) {
        for x in 0..r {
            for y in 0..r {
                let x = x0 + x;
                let y = y0 + y;
                if self.cache.is_visible(x, y) {
                    let pt = self.cache.get_pixel(x, y);
                    self.set_pixel(pt.x, pt.y, col);
                }
            }
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: Color) {
        for t in 0..100 {
            let t = (t as f32)/100.0;
            let x = x1 + (t*(x2 - x1) as f32) as i32;
            let y = y1 + (t*(y2 - y1) as f32) as i32;
    
            self.draw_circle(x, y, 7, col);
        }
    }

    fn draw_town(&mut self, towns: Vec<usize>, cap: usize, col: Color) {
        let mut tmp: Vec<(i32, i32)> = vec![];
        for i in &towns {
            let t = self.towns.get_town(*i);
            for a in 0..100 {
                let a = (a as f32)*2.0*PI / 100.0;
                tmp.push((t.x + (a.cos()*50.0) as i32, t.y + (a.sin()*50.0) as i32));
            }
        }

        let poly = Polygon::new(
            LineString::from(tmp),
            vec![]
        );

        let poly = poly.convex_hull();

        for i in 1..poly.exterior().points().len() {
            let pt1 = poly.exterior()[i-1];
            let pt2 = poly.exterior()[i];
            self.draw_line(pt1.x, pt1.y, pt2.x, pt2.y, col);
        }

        let cap = self.towns.get_town(cap);
        let x2 = cap.x;
        let y2 = cap.y;
        for i in towns {
            let t = self.towns.get_town(i);
            let x1 = t.x;
            let y1 = t.y;
            self.draw_line(x1, y1, x2, y2, [255, 255, 0, 255]);
        }
    }

    pub fn init() -> Renderer {
        // create perlin noise
        let perlin = PerlinNoise2D::new(8, 2.5, 0.5, 1.0, 2.05, (100.0, 100.0), 2.0, 101);

        // setup transformation matrix
        let m = mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let m = scale(&m, vec3(0.7, 0.7, 0.7));
        let m = rotate(&m, PI/3.0, vec3(1.0, 0.0, 0.0));
        let m = rotate(&m, FRAC_PI_4, vec3(0.0, 0.0, 1.0));

        // load cache
        let cache = Cache::load(perlin, m);

        // return renderer object with empty buffer
        Renderer {
            cache: cache,
            towns: TownsR::gen(),
            im_buff: vec![0; 1000*1000*4],
        }
    }

    pub fn render(&mut self, b_towns: Vec<usize>, b_cap: usize, r_towns: Vec<usize>, r_cap: usize) {
        // draw pixels on canvas
        for x in 0..1000 {
            for y in 0..1000 {
                if self.cache.is_visible(x, y) {
                    let pt = self.cache.get_pixel(x, y);
                    self.set_pixel(pt.x, pt.y, pt.c);
                }
            }
        }

        // draw towns
        self.draw_town(b_towns, b_cap, BLUE);
        self.draw_town(r_towns, r_cap, RED);
    }
}
