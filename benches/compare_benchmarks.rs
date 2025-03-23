use std::fmt::Debug;
use biogenie::{
    generate_data, remove_exact_using_bom, remove_exact_using_corasick, remove_exact_using_memchcr,
    remove_using_bitap, remove_using_two_way,
};
use criterion::{Criterion, criterion_group, criterion_main};
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
    
    for seq_len in (50_000_000..1_000_000_000).step_by(50_000_000) {
        let genome = generate_data(seq_len);
        let path = format!("data/inputs/sequence_bench_{}.txt", seq_len);
        fs::write(path, genome.bytes()).unwrap();
        for adapter_len in (4..134).step_by(10) {
            let adapter = generate_data(adapter_len);
            for (idx, trimming_method) in [
                remove_exact_using_memchcr,
                remove_exact_using_corasick,
                remove_exact_using_bom,
                // remove_using_bitap,
                remove_using_two_way,
            ].iter().enumerate() {
                let id: String = format!("{}_seq_{}_adapter_{}", func_names[idx], seq_len, adapter_len);
                println!("{}", id);

                c.bench_function(id.as_str(), |b| {
                    b.iter(|| trimming_method(genome.bytes(), adapter.bytes(), &true));
                });
            }
        }
    }
}

fn custom() -> Criterion {
    Criterion::default().sample_size(10)
        // .without_plots()
        // .warm_up_time(Duration::from_secs(2))
        // .measurement_time(Duration::from_secs(2))
}



criterion_group!{
    name = benches;
    config = custom();
    targets = compare_trimming
}

criterion_main!(benches);
