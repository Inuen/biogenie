#[macro_use]
extern crate timeit;
use biogenie::{
    generate_data, remove_exact_using_bom, remove_exact_using_corasick, remove_exact_using_memchcr,
    remove_using_bitap, remove_using_two_way,
};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use timeit::timeit;

fn main() {
    // let contents = fs::read_to_string("data/inputs/sequence_1_000_000.txt")
    //     .expect("Should have been able to read the file");
    // let adapter = "TCGCAAACTGATCATGGTGGCTACATCCCGTCTGGGTACATGCTAACGTATCTTGACACAACG";
    // let mut results: Vec<Vec<u8>> = Vec::new();
    // let mut len: usize = 0;
    // 
    // println!("memmem");
    // timeit!({
    //     let a = remove_exact_using_memchcr(contents.as_bytes(), adapter.as_bytes(), &true);
    //     len = a.len();
    //     results.push(a);
    // });
    // println!("memmem l {}", len);
    // 
    // println!("corasick");
    // timeit!({
    //     let a = remove_exact_using_corasick(contents.as_bytes(), adapter.as_bytes(), &true);
    //     len = a.len();
    //     results.push(a);
    // });
    // println!("corasick l {}", len);
    // 
    // println!("bom");
    // timeit!({
    //     let a = remove_exact_using_bom(contents.as_bytes(), adapter.as_bytes(), &true);
    //     len = a.len();
    // 
    //     results.push(a);
    // });
    // println!("bom l {}", len);
    // 
    // println!("bitap");
    // timeit!({
    //     let a = remove_using_bitap(contents.as_bytes(), adapter.as_bytes(), &true);
    //     len = a.len();
    //     results.push(a);
    // });
    // println!("BITAP l {}", len);
    // 
    // println!("twoway");
    // timeit!({
    //     let a = remove_using_two_way(contents.as_bytes(), adapter.as_bytes(), &true);
    //     len = a.len();
    //     results.push(a);
    // });
    // println!("twoway l {}", len);
    // println!(stringify!(remove_using_two_way));
    // 
    // assert!(results.iter().all_equal_value().is_ok());
}
