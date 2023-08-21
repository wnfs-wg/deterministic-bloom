window.BENCHMARK_DATA = {
  "lastUpdate": 1692641583464,
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
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a8cd85b1d71da9f79f5058c0a20e53a83a283230",
          "message": "chore(ci)(deps): bump rustsec/audit-check from 0.1.0 to 1.4.1 (#6)\n\nBumps [rustsec/audit-check](https://github.com/rustsec/audit-check) from\r\n0.1.0 to 1.4.1.\r\n<details>\r\n<summary>Release notes</summary>\r\n<p><em>Sourced from <a\r\nhref=\"https://github.com/rustsec/audit-check/releases\">rustsec/audit-check's\r\nreleases</a>.</em></p>\r\n<blockquote>\r\n<h2>v1.4.1</h2>\r\n<ul>\r\n<li>Further corrected reporting on <code>unsound</code> and\r\n<code>notice</code> informationals</li>\r\n</ul>\r\n<h2>v1.4.0</h2>\r\n<ul>\r\n<li>Informational <code>unsound</code> and <code>notice</code>\r\nadvisories are now relayed correctly <a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/9\">#9</a></li>\r\n</ul>\r\n<p>Thanks <a\r\nhref=\"https://github.com/FabianLars\"><code>@​FabianLars</code></a> for\r\nreporting :partying_face:</p>\r\n<h2>v1.3.2</h2>\r\n<ul>\r\n<li>Bumped to node16 and fixed outdated dependencies</li>\r\n</ul>\r\n<p>In case someone reads CHANGELOG this is a v1 release mirror.</p>\r\n<h2>v1</h2>\r\n<p>This is directly-compatible fork-version from\r\nactions-rs/audit-check.</p>\r\n<p>This will not be updated beyond to reflect v1.3.2 so please use the\r\npoint versions on and after v1.4.0</p>\r\n</blockquote>\r\n</details>\r\n<details>\r\n<summary>Changelog</summary>\r\n<p><em>Sourced from <a\r\nhref=\"https://github.com/rustsec/audit-check/blob/main/CHANGELOG.md\">rustsec/audit-check's\r\nchangelog</a>.</em></p>\r\n<blockquote>\r\n<h2>[1.4.1] - 2023-04-04</h2>\r\n<h3>Fixed</h3>\r\n<ul>\r\n<li>Corrected reporting on <code>unsound</code> and <code>notice</code>\r\ninformationals</li>\r\n</ul>\r\n<h2>[1.4.0] - 2023-04-04</h2>\r\n<h3>Fixed</h3>\r\n<ul>\r\n<li>Reflect change to enable warning on <code>unsound</code> and\r\n<code>notice</code> informationals</li>\r\n</ul>\r\n<h2>[1.3.2] - 2023-03-13</h2>\r\n<h3>Changed</h3>\r\n<ul>\r\n<li>Update various dependencies to fix some known vulnerabilities.</li>\r\n</ul>\r\n<h2>[1.3.1] - 2020-05-10</h2>\r\n<h3>Fixed</h3>\r\n<ul>\r\n<li>GitHub Actions does not support sequences as input</li>\r\n</ul>\r\n<h2>[1.3.0] - 2022-05-09</h2>\r\n<h3>Added</h3>\r\n<ul>\r\n<li>Add support for ignores (<a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/1\">#1</a>)</li>\r\n</ul>\r\n<h2>[1.2.0] - 2020-05-07</h2>\r\n<h3>Fixed</h3>\r\n<ul>\r\n<li>Compatibility with latest <code>cargo-audit == 0.12</code> JSON\r\noutput (<a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/115\">#115</a>)</li>\r\n<li>Do not fail check if no critical vulnerabilities were found when\r\nexecuted for a fork repository (closes <a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/104\">#104</a>)</li>\r\n</ul>\r\n<h2>[1.1.0]</h2>\r\n<h3>Fixed</h3>\r\n<ul>\r\n<li>Invalid input properly terminates Action execution (<a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/1\">#1</a>)</li>\r\n<li>Compatibility with new <code>cargo-audit</code> JSON output (<a\r\nhref=\"https://redirect.github.com/rustsec/audit-check/issues/70\">#70</a>)</li>\r\n</ul>\r\n<h2>[1.0.0] - 2019-10-09</h2>\r\n<h3>Added</h3>\r\n<ul>\r\n<li>First public version</li>\r\n</ul>\r\n</blockquote>\r\n</details>\r\n<details>\r\n<summary>Commits</summary>\r\n<ul>\r\n<li>See full diff in <a\r\nhref=\"https://github.com/rustsec/audit-check/compare/0.1.0...v1.4.1\">compare\r\nview</a></li>\r\n</ul>\r\n</details>\r\n<br />\r\n\r\n\r\n[![Dependabot compatibility\r\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=rustsec/audit-check&package-manager=github_actions&previous-version=0.1.0&new-version=1.4.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\r\n\r\nDependabot will resolve any conflicts with this PR as long as you don't\r\nalter it yourself. You can also trigger a rebase manually by commenting\r\n`@dependabot rebase`.\r\n\r\n[//]: # (dependabot-automerge-start)\r\n[//]: # (dependabot-automerge-end)\r\n\r\n---\r\n\r\n<details>\r\n<summary>Dependabot commands and options</summary>\r\n<br />\r\n\r\nYou can trigger Dependabot actions by commenting on this PR:\r\n- `@dependabot rebase` will rebase this PR\r\n- `@dependabot recreate` will recreate this PR, overwriting any edits\r\nthat have been made to it\r\n- `@dependabot merge` will merge this PR after your CI passes on it\r\n- `@dependabot squash and merge` will squash and merge this PR after\r\nyour CI passes on it\r\n- `@dependabot cancel merge` will cancel a previously requested merge\r\nand block automerging\r\n- `@dependabot reopen` will reopen this PR if it is closed\r\n- `@dependabot close` will close this PR and stop Dependabot recreating\r\nit. You can achieve the same result by closing it manually\r\n- `@dependabot ignore this major version` will close this PR and stop\r\nDependabot creating any more for this major version (unless you reopen\r\nthe PR or upgrade to it yourself)\r\n- `@dependabot ignore this minor version` will close this PR and stop\r\nDependabot creating any more for this minor version (unless you reopen\r\nthe PR or upgrade to it yourself)\r\n- `@dependabot ignore this dependency` will close this PR and stop\r\nDependabot creating any more for this dependency (unless you reopen the\r\nPR or upgrade to it yourself)\r\n\r\n\r\n</details>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-31T17:58:27+02:00",
          "tree_id": "a1f027671cfdd758a991c07596f3a526b19d72d3",
          "url": "https://github.com/wnfs-wg/deterministic-bloom/commit/a8cd85b1d71da9f79f5058c0a20e53a83a283230"
        },
        "date": 1690819185869,
        "tool": "cargo",
        "benches": [
          {
            "name": "add",
            "value": 1370,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "count_ones",
            "value": 662,
            "range": "± 37",
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
          "id": "e668ca4a538402d6bc55c1bab780c3476d7672ba",
          "message": "Correctly handle empty bloom filters, avoid infinite loop (#9)\n\n`HashIndexIterator` used to loop infinitely when `bit_size` was 0,\r\nbecause it wouldn't be able to generate a random index that's `< 0`\r\n(fair enough).\r\n\r\nNow it simply exists instantly if `bit_size == 0`.\r\n\r\nAlso added a test case for empty blooms. They \"technically\" contain\r\neverything as a false positive. I opted for that, rather than \"an empty\r\nbloom contains nothing\", since that keeps the invariant that if you\r\n`.insert` something into a bloom filter, it will *always* be\r\n`contain`ed.",
          "timestamp": "2023-08-21T20:08:25+02:00",
          "tree_id": "7bcec74d1143f1aa24a7f6ba840655fcca758e1f",
          "url": "https://github.com/wnfs-wg/deterministic-bloom/commit/e668ca4a538402d6bc55c1bab780c3476d7672ba"
        },
        "date": 1692641408721,
        "tool": "cargo",
        "benches": [
          {
            "name": "add",
            "value": 1470,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "count_ones",
            "value": 740,
            "range": "± 32",
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
            "email": "philipp.krueger1@gmail.com",
            "name": "Philipp Krüger",
            "username": "matheus23"
          },
          "distinct": true,
          "id": "62849fa6b3d1731ea4f4cb18b8d81e845404dc10",
          "message": "Add description",
          "timestamp": "2023-08-21T20:12:09+02:00",
          "tree_id": "04d19e0e42f6aa8654e7641e0821b852292e0ad1",
          "url": "https://github.com/wnfs-wg/deterministic-bloom/commit/62849fa6b3d1731ea4f4cb18b8d81e845404dc10"
        },
        "date": 1692641582968,
        "tool": "cargo",
        "benches": [
          {
            "name": "add",
            "value": 924,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "count_ones",
            "value": 642,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}