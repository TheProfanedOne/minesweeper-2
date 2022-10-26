use rand::{thread_rng, Rng};
use crate::minesweeper::Position;

fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

#[export_name = "random_fields"]
pub fn random_fields(width: usize, height: usize, num_fields: usize) -> impl Iterator<Item = Position> {
    let mut positions: Vec<Position> = Vec::new();

    let mut i = 0;
    while i < num_fields {
        let temp: Position = (random_range(0, width), random_range(0, height));
        if !positions.contains(&temp) {
            positions.push(temp);
            i += 1;
        }
    }

    positions.into_iter()
}