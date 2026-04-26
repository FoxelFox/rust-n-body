use rand::Rng;
use rayon::prelude::*;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    fn distance_to(&self, other: &Vec2) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }
}

pub struct Simulation {
    buffer_a: Vec<Vec2>,
    buffer_b: Vec<Vec2>,
}

impl Simulation {
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
        std::mem::size_of::<Vec2>()
    }

    pub fn work(&mut self) {
        for _ in 0..128 {
            let a = &self.buffer_a;
            self.buffer_b
                .par_iter_mut()
                .enumerate()
                .for_each(|(x, out)| {
                    let mut acc = Vec2 { x: 0.0, y: 0.0 };
                    for y in 0..a.len() {
                        if x == y {
                            continue;
                        }
                        let f = 1.0 / a[x].distance_to(&a[y]).powi(2);
                        acc.x += (a[x].x - a[y].x) * f;
                        acc.y += (a[x].y - a[y].y) * f;
                    }
                    out.x = a[x].x + acc.x;
                    out.y = a[x].y + acc.y;
                });

            mem::swap(&mut self.buffer_a, &mut self.buffer_b);
        }
    }
}
