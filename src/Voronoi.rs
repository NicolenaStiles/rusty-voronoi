///////////////////////////////////////////////////////////////////////////////
// Voronoi Crate
///////////////////////////////////////////////////////////////////////////////
// Contains:
// - VoronoiPoint
// - VoronoiGraph
///////////////////////////////////////////////////////////////////////////////

// General/overall imports:
// imports for VoronoiPoint
use std::fmt; // for debug

// for random seed/site generation
use rand::Rng;

// bitmap processing
use bmp::{Image, Pixel};

///////////////////////////////////////////////////////////////////////////////
// VoronoiPoint
///////////////////////////////////////////////////////////////////////////////
// subunit of the plot. basically a pixel.
///////////////////////////////////////////////////////////////////////////////
// defaults:
// - unit_type = undefined
// - coordinates = (0,0) of u32
// - site_coordinates = (0,0) of u32
// - proximity = 0.0
// - color: white [255,255,255]
///////////////////////////////////////////////////////////////////////////////

// field UnitType for VoronoiPoint
#[derive(Copy,Clone)]
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
    coordinates: Vec<u32>,
    site_coordinates: Vec<u32>,
    proximity: f64,
    color: Vec<u8>,
}

// implementation for VoronoiPoint
// includes:    
// - instantiation
// - debug status printing
impl VoronoiPoint {
    fn new() -> Self {
        VoronoiPoint {
            unit_type: UnitType::Undefined,
            coordinates: vec![0; 2],
            site_coordinates: vec![0; 2],
            proximity: 0.0,
            color: vec![255, 255, 255], // white
        }
    }

    fn print_status(&mut self) {
        println!("type: {:?}", self.unit_type);
        println!("coordinates: {:?}", self.coordinates);
        println!("closest site: {:?}", self.site_coordinates);
        println!("proximity: {:?}", self.proximity);
        println!("rgb: {:?}", self.color);
    }
}

///////////////////////////////////////////////////////////////////////////////
// Helper function: Vornoi Dist
///////////////////////////////////////////////////////////////////////////////
// Returns the distance between two Voronoi points as a float.
///////////////////////////////////////////////////////////////////////////////
// arguments:
// - start: VoronoiPoint
// - end: VoronoiPoint
// returns:
// - dist: f64
///////////////////////////////////////////////////////////////////////////////
fn calc_voronoi_dist(start: VoronoiPoint, end: VoronoiPoint) -> f64 {
    let mut dist: f64 = 0.0;
    // point vals
    let point1_x: f64 = start.coordinates[0] as f64;
    let point1_y: f64 = start.coordinates[1] as f64;
    let point2_x: f64 = end.coordinates[0] as f64;
    let point2_y: f64 = end.coordinates[1] as f64;
    // distance calculation
    let sq_x = f64::powf(point2_x - point1_x, 2.0);
    let sq_y = f64::powf(point2_y - point1_y, 2.0);
    dist = f64::sqrt(sq_x + sq_y); 
    return dist;
}

///////////////////////////////////////////////////////////////////////////////
// VoronoiGraph.rs
///////////////////////////////////////////////////////////////////////////////
// Plot to represent an image of Voronoi Points.
///////////////////////////////////////////////////////////////////////////////
// defaults:
// - unit_type = undefined
// - coordinates = (0,0)
// - site_coordinates = (0,0)
// - proximity = 0.0
// - color: white [255,255,255]
///////////////////////////////////////////////////////////////////////////////
// List of color options:        
/*
    Black	#000000	(0,0,0)
    White	#FFFFFF	(255,255,255)
    Red	#FF0000	(255,0,0)
    Lime	#00FF00	(0,255,0)
    Blue	#0000FF	(0,0,255)
    Yellow	#FFFF00	(255,255,0)
    Cyan / Aqua	#00FFFF	(0,255,255)
    Magenta / Fuchsia	#FF00FF	(255,0,255)
    Silver	#C0C0C0	(192,192,192)
    Gray	#808080	(128,128,128)
    Maroon	#800000	(128,0,0)
    Olive	#808000	(128,128,0)
    Green	#008000	(0,128,0)
    Purple	#800080	(128,0,128)
    Teal	#008080	(0,128,128)
    Navy	#000080	(0,0,128)
*/

// graph/the entire plot
pub struct VoronoiGraph {
    grid_squares: Vec<Vec<VoronoiPoint>>,
    num_sites: u32,
    sites: Vec<VoronoiPoint>,
    palette: Vec<Vec<u8>>,
}

// where r is resolution, n is number of sites, pad is distance from boundaries 
impl VoronoiGraph {
    pub fn new(r: u32, n: u32, pad: u32) -> Self {
        
        // generate empty grid
        let mut grid_init = Vec::new();
        for x in 0..r {
            let mut row_init = Vec::new();
            for y in 0..r {
                let mut single_graph_obj = VoronoiPoint::new();
                single_graph_obj.coordinates = vec![x, y];
                single_graph_obj.unit_type = UnitType::Regular;
                row_init.push(single_graph_obj);
            }
            grid_init.push(row_init);
        }

        // generate empty sites
        let mut sites_init = Vec::new();
        for x in 0..n {
            let mut single_sites_obj = VoronoiPoint::new();
            sites_init.push(single_sites_obj);
        }

        // random site selection
        let mut rng = rand::thread_rng();
        for pts in 0..n {
            // update the grid_sqaure unit
            let mut x_val: usize = rng.gen_range(pad..(r - pad)) as usize;
            let mut y_val: usize = rng.gen_range(pad..(r - pad)) as usize;
            grid_init[x_val][y_val].unit_type = UnitType::Site;
            grid_init[x_val][y_val].site_coordinates[0] = x_val as u32;
            grid_init[x_val][y_val].site_coordinates[1] = y_val as u32;
            grid_init[x_val][y_val].color = vec![255; 3];
            // update the 'sites' subsection
            sites_init[pts as usize].unit_type = UnitType::Site;
            sites_init[pts as usize].coordinates[0] = x_val as u32;
            sites_init[pts as usize].coordinates[1] = y_val as u32;
            sites_init[pts as usize].site_coordinates[0] = x_val as u32;
            sites_init[pts as usize].site_coordinates[1] = y_val as u32;
            sites_init[pts as usize].color = vec![255; 3];
            // leave proximity as 0.0, maybe change convention later?
            grid_init[x_val][y_val].proximity = 0.0;
            sites_init[pts as usize].proximity = 0.0;
        }

        // calculate closest proximity
        for x in 0..r {
            for y in 0..r {
                // current point for dist calcs (so long as it's not a site!)
                let current_pt: VoronoiPoint = grid_init[x as usize][y as usize].clone();
                // init min distance to the max possible distance
                let mut min_dist: f64 = (r as f64) * (2 as f64).sqrt();
                let mut min_dist_site : usize = 0;
                // aggregate distances
                let mut dist_list: Vec<f64> = Vec::new();
                // only works for square images!
                // check what kind of point is currently active
                match current_pt.unit_type {
                    UnitType::Boundary => (),
                    UnitType::Undefined => (),
                    UnitType::Site => (),
                    UnitType::Regular => {
                        // loop over sites and get distances
                        for sites in 0..n {
                            // current site for dist vals
                            let current_site:VoronoiPoint = sites_init[sites as usize].clone();
                            // point vals
                            let mut point_x: f64 = current_pt.coordinates[0] as f64;
                            let mut point_y: f64 = current_pt.coordinates[1] as f64;
                            // site vals
                            let mut site_x: f64 = current_site.coordinates[0] as f64;
                            let mut site_y: f64 = current_site.coordinates[1] as f64;
                            // dist val
                            let mut curr_dist: f64 = 0.0;
                            // distance calculation
                            let mut sq_x = f64::powf((site_x - point_x), 2.0);
                            let mut sq_y = f64::powf((site_y - point_y), 2.0);
                            curr_dist = f64::sqrt(sq_x + sq_y);
                            dist_list.push(curr_dist);

                        }

                        // min dist calcs
                        for dist in 0..n {
                            if min_dist > dist_list[dist as usize] {
                                min_dist = dist_list[dist as usize];
                                min_dist_site = dist as usize;
                            }
                        }
                
                        // update fields with new info
                        grid_init[x as usize][y as usize].site_coordinates[0] = sites_init[min_dist_site].coordinates[0];
                        grid_init[x as usize][y as usize].site_coordinates[1] = sites_init[min_dist_site].coordinates[1];
                        grid_init[x as usize][y as usize].proximity = min_dist;
                
                        // hard-coding some colors for testing...
                        /*
                            Red	#FF0000	(255,0,0)
                            Lime	#00FF00	(0,255,0)
                            Blue	#0000FF	(0,0,255)
                            Yellow	#FFFF00	(255,255,0)
                        */
                        match min_dist_site {
                            0 => grid_init[x as usize][y as usize].color = vec![255, 0, 0],
                            1 => grid_init[x as usize][y as usize].color = vec![0, 255, 0],
                            2 => grid_init[x as usize][y as usize].color = vec![0, 0, 255],
                            3 => grid_init[x as usize][y as usize].color = vec![255, 255, 0],
                            _ => (),
                        };
                    },
                    _ => (),                    
                };

                // debug only
                //println!("Distances: {:?}", dist_list);
                //let min_value = dist_list.iter().cloned().fold(0./0., f64::min);
                //println!("Min val: {:?}", min_value);
                //grid_init[x as usize][y as usize].print_status();
            }
        }

        // 16-bit color. Technically 16 colors, but we ignore white/black for now.
        // see comment at op for what is what
        let mut temp_palette = vec![
            vec![255, 0, 0],
            vec![0, 255, 0],
            vec![0, 0, 255],
            vec![255, 255, 0],
            vec![0, 255, 255],
            vec![255, 0, 255],
            vec![192, 192, 192],
            vec![128, 128, 128],
            vec![128, 0, 0],
            vec![128, 128, 0],
            vec![0, 128, 0],
            vec![128, 0, 128],
            vec![0, 128, 128],
            vec![0, 0, 128],
        ];

        // load into object
        VoronoiGraph {
            grid_squares: grid_init,
            sites: sites_init,
            num_sites: n,
            palette: temp_palette,
        }
    }

    pub fn generate_bitmap(&mut self, img_name: String) {
        let img_res: u32 = self.grid_squares.len() as u32;
        // resolution of image to create (pixels map directly)
        let mut img = Image::new(img_res, img_res);

        for x in 0..img_res {
            for y in 0..img_res {
                let rgb = &self.grid_squares[x as usize][y as usize].color;
                img.set_pixel(x, y, px!(rgb[0], rgb[1], rgb[2]));
            }
        }

        // save image
        let _ = img.save(img_name);
    }

    pub fn print_status(&mut self) {
        // number of total sites
        println!("Number of sites: {:?}", self.num_sites);
        // loop: co-ordinates of all the sites
        let mut idx: usize = 0;
        for s in &self.sites {
            println!("Site #{:?}: {:?}", idx, s.site_coordinates);
            idx += 1;
        }        
    }
}