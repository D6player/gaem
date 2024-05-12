
use perlin2d::PerlinNoise2D;
use glm::{Mat4, vec4};

type Color = [u8; 4];
const BLACK: Color = [0, 0, 0, 255];
const WHITE: Color = [255, 255, 255, 255];

const WIDTH: f32 = 0.015;
const SEPARATION: f32 = 0.15;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub c: Color
}

pub struct Cache {
    buff: Vec<Point>,
    zbuff: Vec<f32>
}
impl Cache {
    pub fn load(perlin: PerlinNoise2D, m: Mat4) -> Cache {
        // initialize buffer
        let mut cache = Cache {
            buff: vec![Point{x:0, y:0, z:0.0, c:BLACK}; 1000*1000],
            zbuff: vec![0.0; 1000*1000]
        };

        // compute surface
        for x in 0..1000 {
            for y in 0..1000 {
                // get 3d space cordinates
                let xf = (x as f32)/100.0 - 5.0;
                let yf = (y as f32)/100.0 - 5.0;
                let zf = perlin.get_noise(xf as f64, yf as f64) as f32;

                // choose color of pixel
                let col: Color = if (-zf%SEPARATION) < WIDTH {
                    WHITE
                } else { BLACK };

                // transform to project
                let v = m * vec4(xf, yf, -zf, 1.0);

                // save on buffer
                let pt = Point {
                    x: ((v.x + 5.0)*100.0) as i32,
                    y: ((v.y + 6.0)*100.0) as i32,
                    z: v.z,
                    c: col
                };
                cache.buff[x+y*1000] = pt;
                cache.zbuff[(pt.x+pt.y*1000) as usize] = v.z;
            }
        }
        return cache;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Point {
        if x < 1000 && x > 0 && y < 1000 && y > 0 {
            self.buff[(x + y*1000) as usize]
        } else {
            Point {
                x: -1,
                y: -1,
                z: 0.0,
                c: [0; 4],
            }
        }
    }

    pub fn get_z(&self, x: i32, y: i32) -> f32 {
        if x < 1000 && x > 0 && y < 1000 && y > 0 {
            self.zbuff[(x + y*1000) as usize]
        } else {
            0.0
        }
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        let pt = self.get_pixel(x, y);
        pt.z >= self.get_z(pt.x, pt.y)
    }
}
