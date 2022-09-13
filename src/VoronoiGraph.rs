///////////////////////////////////////////////////////////////////////////////
// VoronoiGraph.rs
///////////////////////////////////////////////////////////////////////////////
// importing other files
use crate::VoronoiPoint::*;

// for random seed/site generation
use rand::Rng;

// bitmap processing
use bmp::{Image, Pixel};

// graph/the entire plot
pub struct VoronoiGraph {
    pub grid_squares: Vec<Vec<VoronoiPoint>>,
    pub num_sites: u32,
    pub sites: Vec<VoronoiPoint>,
    pub palette: Vec<Vec<u8>>,
}

// where r is resolution and n is number of sites
impl VoronoiGraph {
    pub fn new(r: u32, n: u32) -> Self {
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
            self.grid_squares[x_val][y_val].site_coordinates[0] = x_val as u32;
            self.grid_squares[x_val][y_val].site_coordinates[1] = y_val as u32;
            self.grid_squares[x_val][y_val].color = vec![0; 3];
            // update the 'sites' subsection
            self.sites[pts as usize].unit_type = UnitType::Site;
            self.sites[pts as usize].site_coordinates[0] = x_val as u32;
            self.sites[pts as usize].site_coordinates[1] = y_val as u32;
            self.sites[pts as usize].color = vec![0; 3];
            // leave proximity as 0.0, maybe change convention later?
            // self.grid_squares[x_val][y_val].proximity = 0.0;
            // self.sites[pts as usize].proximity = 0.0;
        }
    }

    // IN PROGRESS
    // loop over all the pixels and calculate distances
    // this is done so poorly it's honestly kinda incredible
    pub fn solve_sites(&mut self) {
        let x_res: usize = self.grid_squares[0].len();
        let y_res: usize = self.grid_squares.len();
        
        println!("x,y resolution: {:?},{:?}", x_res, y_res);

        // looping over all the pixels
        for x in 0..x_res {
            for y in 0..y_res {
                // loop over all the sites, too
                // which one is the closest?
                println!("Current pixel: {:?}, {:?}", x, y);
                // fix this later!!! current way to find pixels that aren't sites
                // (won't work once boundaries are implemented, etc)
                if matches!(self.grid_squares[x][y].unit_type, UnitType::Undefined) {
                    let mut distance: Vec<f64> = Vec::new();
                    for st in &self.sites {
                        let mut site_x: usize = (st.site_coordinates[0] as usize);
                        let mut site_y: usize = (st.site_coordinates[1] as usize);

                        let mut curr_dist: f64 = 0.0;
                        let mut sq_x = usize::pow((site_x - x), 2);
                        let mut sq_y = usize::pow((site_y - y), 2);
                        curr_dist = f64::sqrt(sq_y as f64 / sq_y as f64); 
                        println!("{:?}", curr_dist);
                        //f64::sqrt()
                        //let a = 2; // Can also explicitly define type i.e. i32
                        //let a = i32::pow(a, 10);

                    }
                }
            }
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