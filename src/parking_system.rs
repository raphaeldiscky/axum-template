/// The type of car to park.
pub enum CarType {
    Big = 0,
    Medium = 1,
    Small = 2,
}

/// A parking system with big, medium, and small spots.
pub struct ParkingSystem {
    spots: [u32; 3],
}

impl ParkingSystem {
    pub const fn new(big: u32, medium: u32, small: u32) -> Self {
        Self {
            spots: [big, medium, small],
        }
    }

    pub const fn add_car(&mut self, car_type: CarType) -> bool {
        let idx = car_type as usize;
        if self.spots[idx] > 0 {
            self.spots[idx] -= 1;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_car() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert!(ps.add_car(CarType::Big));
        assert!(ps.add_car(CarType::Medium));
        assert!(!ps.add_car(CarType::Small));
        assert!(!ps.add_car(CarType::Big));
    }
}
