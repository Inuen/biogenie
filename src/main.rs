#[macro_use]
extern crate timeit;
use nucgen::Sequence;
use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::io::Read;
use std::str::from_utf8;
use std::thread::{Scope, scope};
use std::{env, fs};
use timeit::timeit;

pub fn run_parallel_on_thread_pool<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
{
    todo!();
}


fn generate_data(size: usize) -> Sequence {
    let mut rng = rand::thread_rng();

    let mut genome = Sequence::new();
    genome.fill_buffer(&mut rng, size);
    genome
}

#[derive(Clone, Debug)]
enum TrimModes {
    RemoveFromStart,
    RemoveFromEnd,
    RemoveFromAnywhere,
}

#[derive(Clone)]
enum Tasks {
    CountKMers(usize),
    FastAdapt {
        mode: TrimModes,
        sequence: &'static str,
    },
}

#[derive(Clone)]
struct ParallelTask {
    input_file_path: &'static str,
    output_file_path: &'static str,
    task: Tasks,
}

impl ParallelTask {
    fn new(input_file_path: &'static str, output_file_path: &'static str, task: Tasks) -> Self {
        Self {
            input_file_path,
            output_file_path,
            task,
        }
    }

    fn run(&self) {
        match &self.task {
            Tasks::CountKMers(i) => {
                let local_contents = fs::read_to_string(self.input_file_path)
                    .expect("Should have been able to read the file");
                let estimated_count: usize = local_contents.len() - (i + 1);
                let mut ot = FxHashMap::with_capacity_and_hasher(estimated_count, FxBuildHasher);
                local_contents.as_bytes().windows(*i).for_each(|window| {
                    ot.entry(window).and_modify(|cnt| *cnt += 1).or_insert(1);
                });
            }
            Tasks::FastAdapt { mode, sequence } => (),
        };
    }
}

fn main() {
    let kmer_size: usize = 3;
    let pt = ParallelTask::new(
        "data/inputs/sequence1_10_000_000.txt",
        "data/outputs/output1.txt",
        Tasks::CountKMers(kmer_size),
    );
    let tasks = [pt.clone(), pt.clone(), pt.clone(), pt.clone()];
    let contents = fs::read_to_string(
        "data/inputs/sequence1_10_000_000.txt",
    )
    .expect("Should have been able to read the file");

    timeit!({
        let pool = rayon::ThreadPoolBuilder::new()
            // for .num_threads() we use default i.e. by number of available CPUs
            .build()
            .unwrap();
        pool.install(|| {
            scope(|s| {
                let local_tasks = tasks.clone();
                for task in local_tasks {
                    s.spawn(move || task.run());
                }
            })
        })
    });
    println!("/////////////");

    timeit!({
        let estimated_count: usize = contents.len() - kmer_size + 1;
        for _ in 0..4 {
            let mut ot = FxHashMap::with_capacity_and_hasher(estimated_count, FxBuildHasher);
            contents.as_bytes().windows(3).for_each(|window| {
                ot.entry(window).and_modify(|cnt| *cnt += 1).or_insert(1);
            });
        }
    });
}
