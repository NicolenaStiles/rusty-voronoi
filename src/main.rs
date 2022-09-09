///////////////////////////////////////////////////////////////////////////////
// VoronoiPoint.rs 
///////////////////////////////////////////////////////////////////////////////
// imports for VoronoiPoint
use std::fmt; // for debug

// field UnitType for VoronoiPoint
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

// subunit of the plot. basically a pixel.
// defaults:
// unit_type = undefined
// closest_site = (0,0)
// proximity = 0.0
// color: white [255,255,255]
#[derive(Clone)]
struct VoronoiPoint {
    unit_type: UnitType,
    closest_site: Vec<u32>,
    proximity: f64,
    color: Vec<u8>,
}


// implementation for VoronoiPoint
// includes:
// instantiation
// debug status printing
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

///////////////////////////////////////////////////////////////////////////////
// VoronoiGraph.rs
///////////////////////////////////////////////////////////////////////////////

// for random seed/site generation
use rand::Rng;

// graph/the entire plot
struct VoronoiGraph {
    grid_squares: Vec<Vec<VoronoiPoint>>,
    num_sites: u32,
    sites: Vec<VoronoiPoint>,
    palette: Vec<Vec<u8>>,
}

// where r is resolution and n is number of sites
impl VoronoiGraph {
    fn new(r: u32, n: u32) -> Self {
        // generate empty grid
        let mut grid_init = Vec::new();
        for x in 0..r {
            let mut row_init = Vec::new();
            for y in 0..r {
                let mut single_graph_obj = VoronoiPoint::new();
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
        // Temp forced palette definitions
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
        // 16-bit color. Technically 16 colors, but we ignore white/black for now.
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

    // Warning, converts a u32 into a usize. Not a good way to do it.
    pub fn generate_sites(&mut self, pad: u32) {
        let mut rng = rand::thread_rng();
        let res: u32 = self.grid_squares.len() as u32;
        // generate for each site
        for pts in 0..self.num_sites {
            // update the grid_sqaure unit
            let mut x_val: usize = rng.gen_range(pad..(res - pad)) as usize;
            let mut y_val: usize = rng.gen_range(pad..(res - pad)) as usize;
            self.grid_squares[x_val][y_val].unit_type = UnitType::Site;
            self.grid_squares[x_val][y_val].closest_site[0] = x_val as u32;
            self.grid_squares[x_val][y_val].closest_site[1] = y_val as u32;
            self.grid_squares[x_val][y_val].color = vec![0; 3];
            // update the 'sites' subsection
            self.sites[pts as usize].unit_type = UnitType::Site;
            self.sites[pts as usize].closest_site[0] = x_val as u32;
            self.sites[pts as usize].closest_site[1] = y_val as u32;
            self.sites[pts as usize].color = vec![0; 3];
            // leave proximity as 0.0, maybe change convention later?
            // self.grid_squares[x_val][y_val].proximity = 0.0;
            // self.sites[pts as usize].proximity = 0.0;
        }
    }

    // IN PROGRESS
    // loop over all the pixels and calculate distances
    pub fn solve_sites(&mut self) {
        let x_res: usize = self.grid_squares[0].len();
        let y_res: usize = self.grid_squares.len();
        
        // looping over all the pixels
        for x in 0..x_res {
            for y in 0..y_res {
                println!("{:?}", x);
                println!("{:?}", y);
            }
        }

        println!("x resolution: {:?}", x_res);
        println!("y resolution: {:?}", y_res);
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
            println!("Site #{:?}: {:?}", idx, s.closest_site);
            idx += 1;
        }
        // loop: rgb values of each site
        /* 
        idx = 0;
        for rgb in &self.palette {
            println!("Color #{:?}: {:?}", idx, rgb);
            idx += 1;
        }
        */
        
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////

// bitmap processing
#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

fn main() {
    // constants/initialization stuff
    let num_sites: u32 = 4;
    let img_res: u32 = 256;
    let img_pad: u32 = 16;
    let img_name: String = String::from("img.bmp");

    // generates to be totally empty 
    let mut test_plot = VoronoiGraph::new(img_res, num_sites);
    test_plot.generate_sites(img_pad);
    test_plot.print_status();
    test_plot.solve_sites();
    test_plot.generate_bitmap(img_name);    
    
    //let mut test_pts: Vec<u8> = vec![5, 3];
    //let shift: u8 = 3;
    //let mut tv = TestVect {
    //    vector: test_pts,
    //    is_solved: false,
    //};
    //tv.shift_vector(shift);
    //tv.print_status();
    //let mut test_pts: Vec<u8> = vec![5, 3];
    //let shft: u8 = 3;
    //println!("Before function call: {:?}", test_pts);
    //shift_pts(&mut test_pts, shft);
    //println!("After function call: {:?}", test_pts);
}

/*
// todo: constrain length of vectors?
pub struct TestVect {
    pub vector: Vec<u8>,
    pub is_solved: bool,
}

impl TestVect {
    pub fn print_status(&mut self) {
        println!("Vector contents: {:?}", self.vector);
        println!("Shift status: {:?}", self.is_solved);
    }

    pub fn shift_vector(&mut self, shift: u8) {
        let mut temp_x: u8 = self.vector[0 as usize];
        let mut temp_y: u8 = self.vector[1 as usize];
        temp_x += shift;
        temp_y += shift;
        self.vector[0 as usize] = temp_x;
        self.vector[1 as usize] = temp_y;
        self.is_solved = true;
    }
}

fn shift_pts(pts: &mut Vec<u8>, shift: u8) {
    let vect_len: usize = pts.len();
    for idx in 0..vect_len {
        println!("{:?}", pts[idx]);
    }
    let mut temp_x: u8 = pts[0 as usize];
    let mut temp_y: u8 = pts[1 as usize];
    temp_x += shift;
    temp_y += shift;
    pts[0 as usize] = temp_x;
    pts[1 as usize] = temp_y;
    for idx in 0..vect_len {
        println!("{:?}", idx);
        println!("{:?}", pts[idx]);
    }
}

fn main() {
    let mut test_pts: Vec<u8> = vec![5, 3];
    let shift: u8 = 3;
    let mut tv = TestVect {
       vector: test_pts,
       is_solved: false,
    };
    tv.shift_vector(shift);
    tv.print_status();
    let mut test_pts: Vec<u8> = vec![5, 3];
    let shft: u8 = 3;
    println!("Before function call: {:?}", test_pts);
    shift_pts(&mut test_pts, shft);
    println!("After function call: {:?}", test_pts);
}
*/
