// using genome sequence algorithm to check for patterns in superpermutations.
// Hopefully some transform through the ACF or XCF oatterns will point to an
// improved algorithm for calculating them.
// ============================================================================= //

// What is the incidence of non-permutations of n occurring in the string?
//   - any chunk of n chars which do not contain the complete set
//   - can we colour those differently?

// takes arg of a _statement_ and returns that statement and it's result as a string
macro_rules! debug_statement {
  ($x:expr) => {
    println!("{} = {:?}", stringify!($x), $x)
  }
}

// minimal superperm of n = 3 : abcabacba
// minimal superperm of n = 4 : abcdabcadbcabdcabacdbacbdacbadcba
extern crate image;
use std::env;

fn main() {
    if let Some(genome) = env::args().nth(1) {
        let mut labels: Vec<char> = genome.chars().collect();
        let genome_length: usize = labels.len();
        // println!("labels: {:?}", labels);
        let mut genome_mask: Vec<Vec<Option<char>>> = Vec::with_capacity(genome_length);
        // generate LU half of matrix
        for (index, label) in labels.iter().enumerate() {
            genome_mask.push(Vec::with_capacity(index + 1));
            for comparitor in (0..index+1).rev() {
                // println!("label: {}, &labels[{}]: {}", label, comparitor, &labels[comparitor]);
                // debug_statement!(label == &labels[comparitor]);
                genome_mask[index].push(
                    if label == &labels[comparitor] {
                        Some(label.clone())
                    } else {
                        None
                    }
                );
            }
            // println!("genome_mask[{}]: {:?}", index, genome_mask[index]);
        }
        // println!("LU genome_mask: {:?}", genome_mask);
        // make square and flatten
        let half_length: usize = genome_length/2;
        let mut complete_genome: Vec<Option<char>> = Vec::with_capacity(genome_length^2);
        let mut image_vec: Vec<u8> = Vec::with_capacity(genome_length^2); // don't leave this here. Do something better with it.
        let mut genome_matrix: Vec<Vec<Option<char>>> = Vec::with_capacity(genome_length);
        for (index, row_a) in genome_mask.iter().enumerate() {
            let mut row_b: Vec<Option<char>> = genome_mask[(genome_length-1)-index].clone();
            row_b.remove(0);
            let mut row_a = row_a.clone();
            row_b.reverse();
            row_b.append(&mut row_a);
            row_b.reverse();
            // println!("index: {}, row_a: {:?}, row_b: {:?}", index, row_a, row_b);
            let mut output_row: Vec<u8> = Vec::with_capacity(genome_length);
            // not the final form of this image generation...
            for maybe_char in row_b.iter() {
                match maybe_char {
                    &Some(_) => {
                        output_row.push(1 as u8);
                        image_vec.push(1 as u8);
                    },
                    &None => {
                        output_row.push(0 as u8);
                        image_vec.push(0 as u8);
                    }
                }
            }
            println!("{:?}", output_row);
            // keep char identity for colourising
            genome_matrix.push(row_b.clone());
            complete_genome.append(&mut row_b);
        };

        println!("long_genome: {:?}", complete_genome);

        // println!("genome_mask: {:?}", genome_mask);

        // convert genome to [u8] and write to png file
        // let png_genome: &[u8] = genome.map( |row: Vec<Option<char>>| {}).as_ptr();
        let filename: String = match env::args().nth(2) {
            Some(name) => name,
            None => "image.png".to_string()
        };
        println!("filename: {}", filename);
        image::save_buffer(filename, &image_vec, genome_length as u32, genome_length as u32, image::RGBA(8));
        // draw genome_mask to screen
        // display needs to be from top left to bottom right.
        // for mask in genome_mask.iter().rev() {
        //     println!("mask: {:?}", mask);
        // }
        ()
    } else {
        println!("Need to include a string arg to this program.");
    }
}
