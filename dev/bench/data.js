window.BENCHMARK_DATA = {
  "lastUpdate": 1690806929339,
  "repoUrl": "https://github.com/wnfs-wg/deterministic-bloom",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "hello@brooklynzelenka.com",
            "name": "Brooklyn Zelenka",
            "username": "expede"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ae5bda4692d68dab8491893d6eb23a3640712135",
          "message": "Port from `rs-wnfs` (#2)\n\nCo-authored-by: Philipp Krüger <philipp.krueger1@gmail.com>",
          "timestamp": "2023-07-20T20:20:07+02:00",
          "tree_id": "1a18a97802c868fa0022adbcde02fbe9dfa98b12",
          "url": "https://github.com/wnfs-wg/deterministic-bloom/commit/ae5bda4692d68dab8491893d6eb23a3640712135"
        },
        "date": 1689877295424,
        "tool": "cargo",
        "benches": [
          {
            "name": "add",
            "value": 535,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "count_ones",
            "value": 618,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fe077c754cb18067e86371b3e47144c3e7202e0c",
          "message": "Implement a runtime-creation-sized Bloom Filter (#5)\n\n# Description\r\n\r\n- Adds a bloom filter implementation `runtime_sized::BloomFilter`\r\n- Splits out bloom filter implementations across modules\r\n- Implements rejection sampling in `HashIndexIterator` (has no effect if\r\nthe size is a power of two)\r\n\r\n## Link to issue\r\n\r\nFixes #4 \r\n\r\n## Type of change\r\n\r\n- [x] New feature (non-breaking change that adds functionality)\r\n- [x] Refactor (non-breaking change that updates existing functionality)\r\n- [x] Breaking change (fix or feature that would cause existing\r\nfunctionality to not work as expected)\r\n- [x] This change requires a documentation update\r\n- [x] Comments have been added/updated\r\n\r\n## Test plan (required)\r\n\r\n- [x] ~~Needs more tests. Consider this somewhat WIP.~~",
          "timestamp": "2023-07-31T14:34:00+02:00",
          "tree_id": "89fcf62fe97b347106c585a875602bc231d00a0e",
          "url": "https://github.com/wnfs-wg/deterministic-bloom/commit/fe077c754cb18067e86371b3e47144c3e7202e0c"
        },
        "date": 1690806928138,
        "tool": "cargo",
        "benches": [
          {
            "name": "add",
            "value": 1544,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "count_ones",
            "value": 749,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}