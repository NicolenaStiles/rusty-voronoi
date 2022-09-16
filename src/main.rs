///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////
mod Voronoi;

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
    let mut test_plot = Voronoi::VoronoiGraph::new(img_res, num_sites, img_pad);
    test_plot.print_status();
    test_plot.generate_bitmap(img_name);    
    
}
