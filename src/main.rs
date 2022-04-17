// imports
// for random seed/site generation
use rand::Rng;

// subunit of the plot. basically a pixel.
// defaults:
// is_site = false
// closest_site = (0,0)
// proximity = 0.0
#[derive(Clone)]
struct VoronoiUnit {
    is_site: bool,
    closest_site: Vec<u32>,
    proximity: f64,
}

impl VoronoiUnit {
    fn new() -> Self {
        VoronoiUnit {
            is_site: false,
            closest_site: vec![0; 2],
            proximity: 0.0,
        }
    }

    pub fn print_status(&mut self) {
        println!("is site? {:?}", self.is_site);
        println!("closest site? {:?}", self.closest_site);
        println!("site dist: {:?}", self.proximity);
    }
}

// graph/the entire plot
struct Voronoi {
    grid_squares: Vec<Vec<VoronoiUnit>>,
    num_sites: u32,
    sites: Vec<VoronoiUnit>,
}

// where r is resolution and n is number of sites
impl Voronoi {
    fn new(r: u32, n: u32) -> Self {
        // generate empty grid
        let mut grid_init = Vec::new();
        for x in 0..r {
            let mut row_init = Vec::new();
            for y in 0..r {
                let mut single_graph_obj = VoronoiUnit::new();
                row_init.push(single_graph_obj);
            }
            grid_init.push(row_init);
        }
        // generate empty sites
        let mut sites_init = Vec::new();
        for x in 0..n {
            let mut single_sites_obj = VoronoiUnit::new();
            sites_init.push(single_sites_obj);
        }
        // load into object
        Voronoi {
            grid_squares: grid_init,
            sites: sites_init,
            num_sites: n,
        }
    }

    // Warning, converts a u32 into a usize. Not a good way to do it.
    pub fn generate_sites(&mut self, pad: u32) {
        let mut rng = rand::thread_rng();
        let res: u32 = self.grid_squares.len() as u32;
        for pts in 0..self.num_sites {
            // update the grid_sqaure unit
            let mut x_val: usize = rng.gen_range(pad..(res - pad)) as usize;
            let mut y_val: usize = rng.gen_range(pad..(res - pad)) as usize;
            self.grid_squares[x_val][y_val].is_site = true;
            self.grid_squares[x_val][y_val].closest_site[0] = x_val as u32;
            self.grid_squares[x_val][y_val].closest_site[1] = y_val as u32;
            // update the 'sites' subsection
            self.sites[pts as usize].is_site = true;
            self.sites[pts as usize].closest_site[0] = x_val as u32;
            self.sites[pts as usize].closest_site[1] = y_val as u32;
            // leave proximity as 0.0, maybe change convention later?
            // self.grid_squares[x_val][y_val].proximity = 0.0;
            // self.sites[pts as usize].proximity = 0.0;
        }
    }

    pub fn print_status(&mut self) {
        println!("Number of sites: {:?}", self.num_sites);
        let mut idx: usize = 0;
        for s in &self.sites {
            println!("Site #{:?}: {:?}", idx, s.closest_site);
            idx += 1;
        }
    }
}

fn main() {
    // constants/initialization stuff
    let num_sites: u32 = 4;
    let img_res: u32 = 256;
    let img_pad: u32 = 16;

    // generates to be totally empty
    let mut test_plot = Voronoi::new(img_res, num_sites);
    test_plot.generate_sites(img_pad);
    test_plot.print_status();
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
