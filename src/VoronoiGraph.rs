// imports
// base unit object
pub mod VoronoiPoint;

// for random seed/site generation
use rand::Rng;

// graph/the entire plot
struct VoronoiGraph {
    grid_squares: Vec<Vec<VoronoiPoint::VoronoiPoint>>,
    num_sites: u32,
    sites: Vec<VoronoiPoint::VoronoiPoint>,
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
                let mut single_graph_obj = VoronoiPoint::VoronoiPoint::new();
                row_init.push(single_graph_obj);
            }
            grid_init.push(row_init);
        }
        // generate empty sites
        let mut sites_init = Vec::new();
        for x in 0..n {
            let mut single_sites_obj = VoronoiPoint::VoronoiPoint::new();
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
        Voronoi {
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
            self.grid_squares[x_val][y_val].is_site = true;
            self.grid_squares[x_val][y_val].closest_site[0] = x_val as u32;
            self.grid_squares[x_val][y_val].closest_site[1] = y_val as u32;
            self.grid_squares[x_val][y_val].color = vec![0; 3];
            // update the 'sites' subsection
            self.sites[pts as usize].is_site = true;
            self.sites[pts as usize].closest_site[0] = x_val as u32;
            self.sites[pts as usize].closest_site[1] = y_val as u32;
            self.sites[pts as usize].color = vec![0; 3];
            // leave proximity as 0.0, maybe change convention later?
            // self.grid_squares[x_val][y_val].proximity = 0.0;
            // self.sites[pts as usize].proximity = 0.0;
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
            println!("Site #{:?}: {:?}", idx, s.closest_site);
            idx += 1;
        }
        // loop: rgb values of each site
        idx = 0;
        for rgb in &self.palette {
            println!("Color #{:?}: {:?}", idx, rgb);
            idx += 1;
        }
    }
}