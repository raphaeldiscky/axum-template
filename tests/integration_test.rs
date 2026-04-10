use rs_template::math::add;
use rs_template::parking_system::{CarType, ParkingSystem};

#[test]
fn test_parking_with_math() {
    let capacity = add(1, 0) as u32;
    let mut ps = ParkingSystem::new(capacity, capacity, capacity);

    assert!(ps.add_car(CarType::Big));
    assert!(!ps.add_car(CarType::Big));
}

#[test]
fn test_full_parking_flow() {
    let mut ps = ParkingSystem::new(2, 1, 0);

    assert!(ps.add_car(CarType::Big));
    assert!(ps.add_car(CarType::Big));
    assert!(!ps.add_car(CarType::Big));

    assert!(ps.add_car(CarType::Medium));
    assert!(!ps.add_car(CarType::Medium));

    assert!(!ps.add_car(CarType::Small));
}
