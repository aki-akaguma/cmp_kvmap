# cmp_kvmap
research: compare the key-value map types

## Benchmark Results

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-std_hashmap-get     |    9.167 us |   13.599 us |
| cmp-indexmap-get        |    9.782 us |   13.833 us |
| cmp-std_btreemap-get    |   23.092 us |   31.758 us |

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-indexmap-create     |   17.313 us |   19.059 us |
| cmp-std_hashmap-create  |   24.775 us |   24.661 us |
| cmp-std_btreemap-create |   35.523 us |   61.599 us |

- `us` is micro seconds, lower is better
- rustc 1.51.0 (2fd73fabe 2021-03-23)
- bench on intel Q6600 @ 2.40GHz

- [indexmap](https://crates.io/crates/indexmap) - is the iteration order's hash table.

## This benchmark measures kvmap creation and getter.

It is based on `std::collection::HashMap`.

## What do you think? :octocat:
