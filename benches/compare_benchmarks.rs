use biogenie::{
    generate_data, remove_exact_using_bom, remove_exact_using_corasick, remove_exact_using_memchcr,
    remove_using_bitap, remove_using_two_way,
};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::fmt::Debug;
use std::fs;
use std::time::Duration;

fn compare_trimming(c: &mut Criterion) {
    let func_names = [
        "remove_exact_using_memchcr",
        "remove_exact_using_corasick",
        "remove_exact_using_bom",
        // "remove_using_bitap",
        "remove_using_two_way",
    ];

    for (idx, trimming_method) in [
        remove_exact_using_memchcr,
        remove_exact_using_corasick,
        remove_exact_using_bom,
        // remove_using_bitap,
        remove_using_two_way,
    ]
        .iter()
        .enumerate() {
        let mut group = c.benchmark_group(func_names[idx]);

        for seq_len in (10_000_000..30_000_000).step_by(10_000_000) {
            let genome = generate_data(seq_len);
            let path = format!("data/inputs/sequence_bench_{}.txt", seq_len);
            fs::write(path, genome.bytes()).unwrap();
            for adapter_len in (4..34).step_by(10) {
                let adapter = generate_data(adapter_len);
                let id: String = format!(
                    "{}_seq_{}_adapter_{}",
                    func_names[idx], seq_len, adapter_len
                );
                println!("{}", id);
                group.bench_with_input(
                    BenchmarkId::new(
                        func_names[idx],
                        format!("seq_{}_adapter_{}", seq_len, adapter_len),
                    ),
                    &(genome.bytes(), adapter.bytes(), &true),
                    |b, i| b.iter(|| black_box(trimming_method(i.0, i.1, i.2))),
                );
            }
        }
    }
}

fn custom() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = custom();
    targets = compare_trimming
}

criterion_main!(benches);
