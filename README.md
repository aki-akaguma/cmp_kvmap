# cmp_kvmap
research: compare the key-value map types

## Benchmark Results

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-std_hashmap-get     |    8.754 us |   13.576 us |
| cmp-indexmap-get        |    9.941 us |   13.732 us |
| cmp-std_btreemap-get    |   22.618 us |   35.632 us |

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-indexmap-create     |   17.172 us |   18.380 us |
| cmp-std_hashmap-create  |   23.766 us |   24.056 us |
| cmp-std_btreemap-create |   35.046 us |   65.769 us |

- `us` is micro seconds, lower is better
- rustc 1.52.0 (88f19c6da 2021-05-03)
- bench on intel Q6600 @ 2.40GHz

- [indexmap](https://crates.io/crates/indexmap) - is the iteration order's hash table.

## This benchmark measures kvmap creation and getter.

It is based on `std::collection::HashMap`.

## What do you think? :octocat:
