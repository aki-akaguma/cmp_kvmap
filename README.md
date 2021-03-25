# cmp_kvmap
research: compare the key-value map types

## Benchmark Results

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-std_hashmap-get     |    7.833 us |    9.599 us |
| cmp-indexmap-get        |    8.786 us |    9.823 us |
| cmp-std_btreemap-get    |   22.233 us |   34.712 us |

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-indexmap-create     |   17.054 us |   19.192 us |
| cmp-std_hashmap-create  |   24.327 us |   24.904 us |
| cmp-std_btreemap-create |   35.313 us |   67.690 us |

- `us` is micro seconds, lower is better
- rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz

- [indexmap](https://crates.io/crates/indexmap) - is the iteration order's hash table.

## This benchmark measures kvmap creation and getter.

It is based on `std::collection::HashMap`.

## What do you think? :octocat:
