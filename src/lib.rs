use indexmap::map::IndexMap;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn do_std_coll_hashmap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<HashMap<&'a str, &'a str>> {
    let mut map = HashMap::new();
    for (key, val) in keyvals {
        let _ = map.insert(*key, *val);
    }
    Ok(map)
}

pub fn do_std_coll_hashmap_get(
    map: &HashMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for (key, _val) in keyvals {
        match map.get(key) {
            Some(val) => {
                if val == &pattern {
                    found += 1;
                }
            }
            None => {
                eprintln!("can not found {}", key);
                std::process::exit(1);
            }
        }
    }
    Ok(found)
}

pub fn do_std_coll_btreemap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<BTreeMap<&'a str, &'a str>> {
    let mut map = BTreeMap::new();
    for (key, val) in keyvals {
        let _ = map.insert(*key, *val);
    }
    Ok(map)
}

pub fn do_std_coll_btreemap_get(
    map: &BTreeMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for (key, _val) in keyvals {
        match map.get(key) {
            Some(val) => {
                if val == &pattern {
                    found += 1;
                }
            }
            None => {
                eprintln!("can not found {}", key);
                std::process::exit(1);
            }
        }
    }
    Ok(found)
}

pub fn do_indexmap_indexmap_create<'a>(
    keyvals: &[(&'a str, &'a str)],
) -> anyhow::Result<IndexMap<&'a str, &'a str>> {
    let mut map = IndexMap::new();
    for (key, val) in keyvals {
        let _ = map.insert(*key, *val);
    }
    Ok(map)
}

pub fn do_indexmap_indexmap_get(
    map: &IndexMap<&str, &str>,
    keyvals: &[(&str, &str)],
    pattern: &str,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for (key, _val) in keyvals {
        match map.get(key) {
            Some(val) => {
                if val == &pattern {
                    found += 1;
                }
            }
            None => {
                eprintln!("can not found {}", key);
                std::process::exit(1);
            }
        }
    }
    Ok(found)
}
