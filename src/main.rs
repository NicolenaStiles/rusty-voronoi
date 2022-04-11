// https://github.com/image-rs/image/tree/master/examples

//! An example of opening an image.
/*
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageFormat};
*/

// bitmap processing
#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

// random seed/site generation
use rand::Rng;

struct VoronoiPlot {
    grid_squares: Vec<Vec<VoronoiUnit>>,
}

#[derive(Clone)]
struct VoronoiUnit {
    is_site: bool,
    proximity: float,
}

// generate some random points for a "good" Voronoi diagram
// (not too close to the boundaries, etc etc)
// INPUT    || n : u8, number of points to generate
//          || x : u32, resolution of image for processing
//          || y : u32, resolution of image for processing
// OUTPUT   || sites : Vec<Vec<u32>>, set of points as [x,y] to mark as sites
// ---------------------------------------------------------------------------
// TODO: Feel like I'm using vectors wrong? figure out best practices for indexing and
//       when to specify u8/u32 vs. usize
fn generate_rand_pts(n: u8, x: u32, y: u32, xpad: u32, ypad: u32) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut sites: Vec<Vec<u32>> = vec![vec![0; 2]; n.into()];
    for pts in 0..n {
        // generate x
        sites[pts as usize][0 as usize] = rng.gen_range(xpad..(x - xpad));
        // generate y
        sites[pts as usize][1 as usize] = rng.gen_range(ypad..(y - ypad));
    }
    return sites;
}

fn main() {
    // constants/initialization stuff
    let num_pts: u8 = 4;
    let img_res: u32 = 256;
    let img_pad: u32 = 16;

    // quick test for site generation
    let quicktest: Vec<Vec<u32>> = generate_rand_pts(num_pts, img_res, img_res, img_pad, img_pad);

    println!("{:?}", quicktest);

    // resolution of image to create (pixels map directly)
    /*
    let mut img = Image::new(512, 512);

    for (x, y) in img.coordinates() {
        if x > 255 && y > 255 {
            img.set_pixel(x, y, bmp::consts::AQUA)
        } else {
            img.set_pixel(x, y, bmp::consts::ORANGE)
        }

        //img.set_pixel(x, y, px!(x, 0, 0));
    }
    let _ = img.save("img.bmp");
    */
}
