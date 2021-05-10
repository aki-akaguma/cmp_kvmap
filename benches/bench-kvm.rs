use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indexmap::map::IndexMap;
use std::collections::BTreeMap;
use std::collections::HashMap;

fn process_std_coll_hashmap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<HashMap<&'a str, &'a str>> {
    cmp_kvmap::do_std_coll_hashmap_create(keyvals)
}

fn process_std_coll_hashmap_get(
    map: &HashMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    cmp_kvmap::do_std_coll_hashmap_get(map, keyvals, pattern)
}

fn process_std_coll_btreemap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<BTreeMap<&'a str, &'a str>> {
    cmp_kvmap::do_std_coll_btreemap_create(keyvals)
}

fn process_std_coll_btreemap_get(
    map: &BTreeMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    cmp_kvmap::do_std_coll_btreemap_get(map, keyvals, pattern)
}

fn process_indexmap_indexmap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<IndexMap<&'a str, &'a str>> {
    cmp_kvmap::do_indexmap_indexmap_create(keyvals)
}

fn process_indexmap_indexmap_get(
    map: &IndexMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    cmp_kvmap::do_indexmap_indexmap_get(map, keyvals, pattern)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s) = create_data::create_data();
    let vv: Vec<(&str, &str)> = v
        .iter()
        .map(|(key, val)| (key.as_str(), val.as_str()))
        .collect();
    //
    let hash_map = process_std_coll_hashmap_create(black_box(&vv)).unwrap();
    let btree_map = process_std_coll_btreemap_create(black_box(&vv)).unwrap();
    let index_map = process_indexmap_indexmap_create(black_box(&vv)).unwrap();
    //
    match process_std_coll_hashmap_get(&hash_map, black_box(&vv), black_box(pat_string_s)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_std_coll_btreemap_get(&btree_map, black_box(&vv), black_box(pat_string_s)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_indexmap_indexmap_get(&index_map, black_box(&vv), black_box(pat_string_s)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("cmp-std_hashmap-create", |b| {
        b.iter(|| {
            let _r =
                process_std_coll_hashmap_create(black_box(&vv)).unwrap();
        })
    });
    c.bench_function("cmp-std_btreemap-create", |b| {
        b.iter(|| {
            let _r =
                process_std_coll_btreemap_create(black_box(&vv)).unwrap();
        })
    });
    c.bench_function("cmp-indexmap-create", |b| {
        b.iter(|| {
            let _r =
                process_indexmap_indexmap_create(black_box(&vv)).unwrap();
        })
    });
    c.bench_function("cmp-std_hashmap-get", |b| {
        b.iter(|| {
            let _r =
                process_std_coll_hashmap_get(&hash_map, black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("cmp-std_btreemap-get", |b| {
        b.iter(|| {
            let _r =
                process_std_coll_btreemap_get(&btree_map, black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("cmp-indexmap-get", |b| {
        b.iter(|| {
            let _r =
                process_indexmap_indexmap_get(&index_map, black_box(&vv), black_box(pat_string_s));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
