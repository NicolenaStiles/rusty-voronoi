///////////////////////////////////////////////////////////////////////////////
// VoronoiPoint.rs 
///////////////////////////////////////////////////////////////////////////////
// subunit of the plot. basically a pixel.
///////////////////////////////////////////////////////////////////////////////
// defaults:
//  unit_type = undefined
//  coordinates = (0,0)
//  site_coordinates = (0,0)
//  proximity = 0.0
//  color: white [255,255,255]
///////////////////////////////////////////////////////////////////////////////
// imports for VoronoiPoint
use std::fmt; // for debug

// field UnitType for VoronoiPoint
#[derive(Clone)]
pub enum UnitType {
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
pub struct VoronoiPoint {
    pub unit_type: UnitType,
    pub coordinates: Vec<u32>,
    pub site_coordinates: Vec<u32>,
    pub proximity: f64,
    pub color: Vec<u8>,
}

// implementation for VoronoiPoint
// includes:    
// instantiation
// debug status printing
impl VoronoiPoint {
    pub fn new() -> Self {
        VoronoiPoint {
            unit_type: UnitType::Undefined,
            coordinates: vec![0; 2],
            site_coordinates: vec![0; 2],
            proximity: 0.0,
            color: vec![255, 255, 255], // white
        }
    }

    pub fn print_status(&mut self) {
        println!("type: {:?}", self.unit_type);
        println!("coordinates: {:?}", self.coordinates);
        println!("closest site: {:?}", self.site_coordinates);
        println!("proximity: {:?}", self.proximity);
        println!("rgb: {:?}", self.color);
    }
}