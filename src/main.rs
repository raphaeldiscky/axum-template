use rs_template::math::add;
use rs_template::parking_system::{CarType, ParkingSystem};

fn main() {
    println!("1 + 2 = {}", add(1, 2));

    let mut ps = ParkingSystem::new(1, 1, 0);
    println!("Park big car: {}", ps.add_car(CarType::Big));
    println!("Park small car: {}", ps.add_car(CarType::Small));
}
