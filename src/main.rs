pub use satellite::godmod::planet::Planet;
pub use satellite::godmod::universe::Universe;

use std::{thread, time};

fn main() {
    let terre = Planet::new("Terre".to_string(), 0.0, 0.0, 0.0, 0.0, 5.97e24, 6.371e6).unwrap();
    let lune1 = Planet::new(
        "Lune 1".to_string(),
        384000000.,
        0.0,
        0.0,
        1000.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune2 = Planet::new(
        "Lune 2".to_string(),
        -384000000.,
        0.0,
        0.0,
        600.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune3 = Planet::new(
        "Lune 3".to_string(),
        -184000000.,
        0.0,
        0.0,
        -600.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune4 = Planet::new(
        "Lune 4".to_string(),
        184000000.,
        0.0,
        0.0,
        -800.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune5 = Planet::new(
        "Lune 5".to_string(),
        584000000.,
        0.0,
        0.0,
        -400.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune6 = Planet::new(
        "Lune 6".to_string(),
        584000000.,
        584000000.0,
        -2000.0,
        -2000.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let lune7 = Planet::new(
        "Lune 7".to_string(),
        584000000.,
        584000000.0,
        -2000.0,
        -2000.0,
        7.36e22,
        1.737e6,
    )
    .unwrap();
    let mut planets = Vec::new();
    planets.push(terre);
    planets.push(lune1);
    planets.push(lune2);
    planets.push(lune3);
    planets.push(lune4);
    planets.push(lune5);
    planets.push(lune6);
    planets.push(lune7);

    let mut universe = Universe { planets: planets };

    // universe.remove("Terre".to_string());

    let dt = 10.; // [s]
    let total_simulation_time = 3600. * 24. * 20.; // [s]
    let n_steps = ((total_simulation_time / dt) + 1.0) as i32;
    for step in 0..n_steps {
        universe.do_time_step(dt).expect("Something went wrong");
        if step % 600 == 0 {
            universe.draw();
            let sleep_millis = time::Duration::from_millis(50);
            thread::sleep(sleep_millis);
        };
    }
    println!("Universe is now : {}", &universe);
}
