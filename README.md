[![crates.io](https://img.shields.io/crates/v/datafile-test.svg)](https://crates.io/crates/datafile-test)
[![docs.rs](https://docs.rs/datafile-test/badge.svg)](https://docs.rs/datafile-test)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/build.yml/badge.svg)](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/build.yml)
[![Test](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/test.yml/badge.svg)](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/test.yml)
[![Audit](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/audit.yml/badge.svg)](https://github.com/lambdalisue/rs-datafile-test/actions/workflows/audit.yml)

# 📃 datafile-test

This crate provides a macro for data file driven test.
The macro generates a test function for each entry in the data file and runs the test function for each entry.

## Usage

Use `datafile_test` attribute macro to define a test function.
Note that this macro requires the [`serde`] and [`serde_json`] crate to be available.

[`serde`]: https://crates.io/crates/serde
[`serde_json`]: https://crates.io/crates/serde_json

```rust
use datafile_test::datafile_test;

#[derive(Debug, serde::Deserialize)]
struct TestCaseInput {
    a: i32,
    b: i32,
}

#[derive(Debug, serde::Deserialize)]
struct TestCase {
    input: TestCaseInput,
    output: i32,
}

#[datafile_test("tests/datafile_test.json")]
fn test_addition(test_case: TestCase) {
    assert_eq!(test_case.input.a + test_case.input.b, test_case.output);
}
```

This code will generate a test function for each entry in the data file `tests/datafile_test.json`.
The data file should be a JSON or YAML file containing an array of objects like this:

```json
[
    {
        "input": {
            "a": 1,
            "b": 2
        },
        "output": 3
    },
    {
        "input": {
            "a": 2,
            "b": 3
        },
        "output": 5
    }
]
```

# License

The code follows the MIT license written in [LICENSE](./LICENSE). Contributors
need to agree that any modifications sent to this repository follow the license.
