// subunit of the plot. basically a pixel.
// defaults:
// unit_type = undefined
// closest_site = (0,0)
// proximity = 0.0
// color: white [255,255,255]
use std::fmt; // for debug

#[derive(Clone)]
enum UnitType {
    Undefined,
    Site,
    Boundary,
    Regular,
}

// debug implementation for UnitType enum
impl fmt::Debug for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnitType::Undefined => write!(f, "Undefined"),
            UnitType::Site => write!(f, "Site"),
            UnitType::Boundary => write!(f, "Boundary"),
            UnitType::Regular => write!(f, "Regular"),        
        }
    }
}

#[derive(Clone)]
struct VoronoiPoint {
    unit_type: UnitType,
    closest_site: Vec<u32>,
    proximity: f64,
    color: Vec<u8>,
}

impl VoronoiPoint {
    fn new() -> Self {
        VoronoiPoint {
            unit_type: UnitType::Undefined,
            closest_site: vec![0; 2],
            proximity: 0.0,
            color: vec![255, 255, 255], // white
        }
    }

    pub fn print_status(&mut self) {
        println!("type: {:?}", self.unit_type);
        println!("closest site: {:?}", self.closest_site);
        println!("site dist: {:?}", self.proximity);
        println!("rgb: {:?}", self.color);
    }
}