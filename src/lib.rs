use rand::Rng;
use std::mem;
use wasm_bindgen::prelude::*;

#[repr(C)] // Ensure a predictable C-like layout in memory (x followed by y)
#[derive(Debug, Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn distance_to(&self, other: &Vec2) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }
}

#[wasm_bindgen]
pub struct Simulation {
    buffer_a: Vec<Vec2>,
    buffer_b: Vec<Vec2>,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let buffer_a: Vec<Vec2> = (0..size)
            .map(|_| Vec2 {
                x: rng.r#gen::<f64>() - 0.5,
                y: rng.r#gen::<f64>() - 0.5,
            })
            .collect();

        let buffer_b: Vec<Vec2> = vec![Vec2 { x: 0.0, y: 0.0 }; size];

        Simulation { buffer_a, buffer_b }
    }

    pub fn len(&self) -> usize {
        self.buffer_a.len()
    }

    pub fn data_ptr(&self) -> *const Vec2 {
        self.buffer_a.as_ptr()
    }

    pub fn size(&self) -> usize {
        size_of::<Vec2>()
    }

    pub fn work(&mut self) {
        let size = self.buffer_a.len();

        for _ in 0..128 {
            for x in 0..size {
                for y in 0..size {
                    self.buffer_b[x].x = 0.0;
                    self.buffer_b[x].y = 0.0;

                    if x == y {
                        continue;
                    }
                    // distance
                    let f = 1.0 / self.buffer_a[x].distance_to(&self.buffer_a[y]).powi(2);
                    self.buffer_b[x].x += (self.buffer_a[x].x - self.buffer_a[y].x) * f;
                    self.buffer_b[x].y += (self.buffer_a[x].y - self.buffer_a[y].y) * f;
                }
                self.buffer_b[x].x = self.buffer_a[x].x + self.buffer_b[x].x;
                self.buffer_b[x].y = self.buffer_a[x].y + self.buffer_b[x].y;
            }

            // swap
            mem::swap(&mut self.buffer_a, &mut self.buffer_b);
        }
    }
}
