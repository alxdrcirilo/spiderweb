use rand::Rng;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

const DEPTH: u16 = 16;
const EDGES: u16 = 24;
const SCALING_FACTOR: f64 = 360.0 / EDGES as f64 / 2.0;

fn get_angles(edges: u16) -> Vec<u16> {
    // Divide the circle into a vector of angles
    (0..edges).map(|i: u16| i * 360 / edges).collect()
}

fn get_coordinates(
    angles: Vec<u16>,
    mut rng: rand::prelude::ThreadRng,
) -> BTreeMap<u16, Vec<(f64, f64)>> {
    // Generate coordinates for each edge [key: angle, value: list of coordinates)
    let mut coordinates: BTreeMap<u16, Vec<(f64, f64)>> = BTreeMap::new();

    for &angle in &angles {
        // Add 5% random scaling factor to the (root) angle
        let angle: f64 =
            rng.gen_range(angle as f64 - SCALING_FACTOR..angle as f64 + SCALING_FACTOR);

        // Convert angle to radians
        let radians: f64 = angle as f64 * std::f64::consts::PI / 180.0;
        
        let mut angle_coordinates: Vec<(f64, f64)> = Vec::with_capacity(DEPTH as usize);

        // Generate coordinates for each level
        for level in 1..=DEPTH {
            // Add 5% random scaling factor to the radius
            let radius: f64 = rng.gen_range(0.95..1.05) * (level as f64).powi(3);

            // Add 3% random scaling factor to the (child) node
            angle_coordinates.push((
                radius * radians.cos() * rng.gen_range(0.97..1.03),
                radius * radians.sin() * rng.gen_range(0.97..1.03),
            ));
        }
        coordinates.insert(angle as u16, angle_coordinates);
    }
    coordinates
}

fn dump(coordinates: BTreeMap<u16, Vec<(f64, f64)>>) {
    // Write to file
    let mut file: File = File::create("data/coordinates.txt").expect("Creation failed");

    // Write contents to the file x1,y1,x2,y2 per line
    for (_node, coords) in &coordinates {
        let line = format!("{:?}\n", coords);
        file.write_all(line.as_bytes()).expect("Write failed");
    }
}

fn main() {
    let rng = rand::thread_rng();
    let angles = get_angles(EDGES);
    let coordinates = get_coordinates(angles, rng);
    dump(coordinates);
}
