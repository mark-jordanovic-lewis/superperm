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

use std::env;

fn main() {
    if let Some(mut genome) = env::args().nth(1) {
        let mut labels: Vec<char> = genome.chars().collect();
        let genome_length: usize = labels.len();
        // println!("labels: {:?}", labels);
        let mut genome_mask: Vec<Vec<Option<char>>> = Vec::with_capacity(genome_length);
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
        // println!("genome_mask: {:?}", genome_mask);

        // draw genome_mask to screen
        // display needs to be from top left to bottom right.
        for mask in genome_mask.iter().rev() {
            println!("mask: {:?}", mask);
        }
        ()
    } else {
        println!("Need to include a string arg to this program.");
    }
}
