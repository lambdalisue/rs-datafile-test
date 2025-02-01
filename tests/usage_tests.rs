#![cfg(test)]

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

#[datafile_test("tests/testcase.json")]
fn test_with_json(testcase: TestCase) {
    assert_eq!(testcase.input.a + testcase.input.b, testcase.output);
}

#[datafile_test("tests/testcase.yml")]
fn test_with_yaml(testcase: TestCase) {
    assert_eq!(testcase.input.a + testcase.input.b, testcase.output);
}
