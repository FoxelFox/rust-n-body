use getrandom::getrandom;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn distance_to(&self, other: &Vec2) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }
}

fn get_random_f64_raw() -> f64 {
    let mut buf = [0u8; 8];
    getrandom(&mut buf).unwrap();
    f64::from_ne_bytes(buf)
}

#[wasm_bindgen]
pub fn work(array_size: usize) {


    let mut buffer_a: Vec<Vec2> = (0..array_size)
        .map(|_| Vec2 {
            x: get_random_f64_raw() - 0.5,
            y: get_random_f64_raw() - 0.5,
        })
        .collect();

    let mut buffer_b: Vec<Vec2> = vec![Vec2 { x: 0.0, y: 0.0 }; array_size];

    for _ in 0..128 {
        for x in 0..1024 {
            for y in 0..1024 {
                buffer_b[x].x = 0.0;
                buffer_b[x].y = 0.0;

                if x == y {
                    continue;
                }
                // distance
                let f = 1.0 / buffer_a[x].distance_to(&buffer_a[y]).powi(2);
                buffer_b[x].x += (buffer_a[x].x - buffer_a[y].x) * f;
                buffer_b[x].y += (buffer_a[x].y - buffer_a[y].y) * f;
            }
            buffer_b[x].x = buffer_a[x].x + buffer_b[x].x;
            buffer_b[x].y = buffer_a[x].y + buffer_b[x].y;
        }

        // swap
        let tmp = buffer_a;
        buffer_a = buffer_b;
        buffer_b = tmp;
    }

    println!("{:?}", buffer_a[0]);
}
