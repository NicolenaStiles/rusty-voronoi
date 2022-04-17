```
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

// graph/the entire plot
struct VoronoiPlot {
    grid_squares: Vec<Vec<VoronoiUnit>>,
}

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
    let mut site_array: Vec<Vec<u32>> =
        generate_rand_pts(num_pts, img_res, img_res, img_pad, img_pad);

    // spinning up empty voronoi graph
    let mut grid_init = Vec::new();
    for x in 0..img_res {
        let mut row_init = Vec::new();
        for y in 0..img_res {
            let mut single_graph_obj = VoronoiUnit {
                is_site: false,
                closest_site: vec![0, 0],
                proximity: 0.0,
            };
            row_init.push(single_graph_obj);
        }
        grid_init.push(row_init);
    }

    // load voronoi graph with sites
    // (flippin' some bools and making nearest site self)
    for site in &site_array {
        let x_site = site[0 as usize];
        let y_site = site[1 as usize];
        grid_init[x_site as usize][y_site as usize].is_site = true;
        grid_init[x_site as usize][y_site as usize].closest_site[0 as usize] = x_site;
        grid_init[x_site as usize][y_site as usize].closest_site[1 as usize] = y_site;
    }

    // actual voronoi pts calculations!
    // haven't tested this at all, but I think it works.
    // TODO: fix the freaking type handling. Sheesh.
    for x in 0..img_res {
        for y in 0..img_res {
            // for each point...
            let mut curr_pt = &grid_init[x as usize][y as usize];
            // if not a site...
            if curr_pt.is_site == false {
                // calculate distance to all sites...
                let mut min_dist: f64 = img_res as f64;
                for site in &site_array {
                    let mut x_diff =
                        f64::powf(((site[0 as usize] as f64) - (x as f64)) as f64, 2.0);
                    let mut y_diff =
                        f64::powf(((site[1 as usize] as f64) - (y as f64)) as f64, 2.0);
                    let mut curr_dist = f64::powf((x_diff + y_diff) as f64, 0.5);
                    if curr_dist < min_dist {
                        min_dist = curr_dist;
                        grid_init[x as usize][y as usize].closest_site[0 as usize] =
                            site[0 as usize];
                        grid_init[x as usize][y as usize].closest_site[1 as usize] =
                            site[1 as usize];
                        grid_init[x as usize][y as usize].proximity = min_dist;
                    }
                }
            }
        }
    }

    println!("{:?}", site_array);

    // resolution of image to create (pixels map directly)

    let mut img = Image::new(img_res, img_res);

    for x in 0..img_res {
        for y in 0..img_res {
            // if it's a site...
            if grid_init[x as usize][y as usize].is_site == true {
                img.set_pixel(x, y, bmp::consts::BLACK);
            } else {
                // otherwise, it's a regular point. those will need to be color-coded... eventually.
                img.set_pixel(x, y, bmp::consts::WHITE);
            }
        }
    }

    // save image
    let _ = img.save("img.bmp");

    /*
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

```
