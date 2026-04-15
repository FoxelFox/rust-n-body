use n_body::Simulation;

fn main() {
    let n = 2048;
    let mut sim = Simulation::new(n);

    println!("Running n-body simulation with {n} bodies...");

    sim.work();
    let ptr = sim.data_ptr();
    // SAFETY: ptr is valid for `n` Vec2 values owned by `sim`
    let positions = unsafe { std::slice::from_raw_parts(ptr, n) };
    let first = &positions[0];
    println!("body[0] = ({:.4}, {:.4})", first.x, first.y);
}
