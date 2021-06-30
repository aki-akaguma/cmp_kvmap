# cmp_kvmap
research: compare the key-value map types

## Benchmark Results

- rustc 1.53.0 (53cb7b09b 2021-06-17)

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-std_hashmap-get     |   17.606 us |   10.260 us |
| cmp-indexmap-get        |   18.355 us |   10.464 us |
| cmp-std_btreemap-get    |   31.660 us |   24.596 us |

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-indexmap-create     |   16.313 us |   18.155 us |
| cmp-std_hashmap-create  |   23.605 us |   24.365 us |
| cmp-std_btreemap-create |   38.076 us |   55.816 us |

- rustc 1.52.0 (88f19c6da 2021-05-03)

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
- bench on intel Q6600 @ 2.40GHz

- [indexmap](https://crates.io/crates/indexmap) - is the iteration order's hash table.

## This benchmark measures kvmap creation and getter.

It is based on `std::collection::HashMap`.

## What do you think? :octocat:
