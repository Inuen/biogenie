use aho_corasick::AhoCorasick;
use bio::pattern_matching::bom::BOM;
use bitap::Pattern;
use itertools::Itertools;
use memchr::memmem;
use nucgen::Sequence;
use std::str::from_utf8;

pub fn generate_data(size: usize) -> Sequence {
    let mut rng = rand::thread_rng();

    let mut genome = Sequence::new();
    genome.fill_buffer(&mut rng, size);
    genome
}

pub fn remove_exact_using_starts_with(
    mut sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let mut trimmed: Vec<u8> = Vec::with_capacity(sequence.len());
    while sequence.starts_with(adapter) {
        sequence = &sequence[adapter.len()..];
        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(sequence);
    trimmed
}

pub fn remove_exact_using_ends_with(
    mut sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let mut trimmed: Vec<u8> = Vec::with_capacity(sequence.len());
    while sequence.ends_with(adapter) {
        sequence = &sequence[..sequence.len() - adapter.len()];
        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(sequence);
    trimmed
}

pub fn remove_exact_using_corasick(
    sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let mut trimmed = Vec::with_capacity(sequence.len());
    let ac = AhoCorasick::new([adapter]).unwrap();
    let mut last_removed_idx: usize = 0;

    for matched in ac.find_iter(sequence) {
        trimmed.extend_from_slice(&sequence[last_removed_idx..matched.start()]);
        last_removed_idx = matched.end();
        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(&sequence[last_removed_idx..]);
    trimmed
}

pub fn remove_exact_using_bom(
    sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let bom = BOM::new(adapter);
    let occ: Vec<usize> = bom.find_all(sequence).collect();
    let mut trimmed = Vec::with_capacity(sequence.len());

    let mut last_idx = 0;
    for match_idx in occ {
        if match_idx < last_idx {
            continue;
        }
        trimmed.extend_from_slice(&sequence[last_idx..match_idx]);
        last_idx = match_idx + adapter.len();
        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(&sequence[last_idx..]);
    trimmed
}

pub fn remove_using_bitap(
    sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let pattern = Pattern::new(from_utf8(adapter).unwrap()).expect("adapter is too long fir bitap");
    let mut trimmed = Vec::with_capacity(sequence.len());
    let allowed_levenshtein_distance: usize = 0; // TODO should be arg

    let matches = pattern.lev(from_utf8(sequence).unwrap(), allowed_levenshtein_distance);
    let mut last_match_end_idx = 0;
    // Bitap doesn't inherently return matches in left to right order
    for adapter_match in matches.sorted_by(|m1, m2| m2.end.cmp(&m1.end)) {
        trimmed.extend_from_slice(&sequence[..adapter_match.end - adapter.len() + 1]);
        last_match_end_idx = adapter_match.end;
        if !allow_multiple_matches {
            break;
        }
    }
    if last_match_end_idx != 0 {
        trimmed.extend_from_slice(&sequence[last_match_end_idx + 1..]);
    } else {
        trimmed.extend_from_slice(sequence);
    }
    trimmed
}

pub fn remove_exact_using_memchcr(
    sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let mut trimmed = Vec::with_capacity(sequence.len());

    let mut read_idx = 0;

    while let Some(idx) = memmem::find(&sequence[read_idx..], adapter) {
        let adapter_idx = idx + read_idx;

        trimmed.extend_from_slice(&sequence[read_idx..adapter_idx]);
        read_idx = adapter_idx + adapter.len();
        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(&sequence[read_idx..]);
    trimmed
}

pub fn remove_using_two_way(
    sequence: &[u8],
    adapter: &[u8],
    allow_multiple_matches: &bool,
) -> Vec<u8> {
    let mut read_idx = 0;
    let mut trimmed = Vec::with_capacity(sequence.len());

    while let Some(idx) = twoway::find_bytes(&sequence[read_idx..], adapter) {
        let adapter_idx = idx + read_idx;
        trimmed.extend_from_slice(&sequence[read_idx..adapter_idx]);

        read_idx = adapter_idx + adapter.len();

        if !allow_multiple_matches {
            break;
        }
    }
    trimmed.extend_from_slice(&sequence[read_idx..]);

    trimmed
}

#[derive(Clone, Debug)]
pub enum TrimmingModes {
    AtStartOnly,
    AtStartWithPrefix,
    AtEndWithPrefix,
    AtEndOnly,
    Anywhere,
}

#[derive(Clone)]
pub enum Tasks<'a> {
    CountKMers(usize),
    TrimAdapt {
        mode: TrimmingModes,
        sequence: &'a [u8],
        adapter: &'a [u8],
        allow_multiple_matches: bool,
        fuzzy_matching: bool,
    },
}
