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
    let mut test_plot = Voronoi::VoronoiGraph::new(img_res, num_sites);
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
