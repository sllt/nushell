use nu_test_support::{nu, pipeline};

#[test]
fn each_works_separately() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
<<<<<<< HEAD
        echo [1 2 3] | each { echo $it 10 | math sum } | to json
=======
        echo [1 2 3] | each { echo $it 10 | math sum } | to json -r
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
        "#
    ));

    assert_eq!(actual.out, "[11,12,13]");
}

#[test]
fn each_group_works() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
<<<<<<< HEAD
        echo [1 2 3 4 5 6] | each group 3 { $it } | to json
=======
        echo [1 2 3 4 5 6] | each group 3 { $it } | to json --raw
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
        "#
    ));

    assert_eq!(actual.out, "[[1,2,3],[4,5,6]]");
}

#[test]
fn each_window() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
<<<<<<< HEAD
        echo [1 2 3 4] | each window 3 { $it } | to json
=======
        echo [1 2 3 4] | each window 3 { $it } | to json --raw
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
        "#
    ));

    assert_eq!(actual.out, "[[1,2,3],[2,3,4]]");
}

#[test]
fn each_window_stride() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
<<<<<<< HEAD
        echo [1 2 3 4 5 6] | each window 3 -s 2 { echo $it } | to json
=======
        echo [1 2 3 4 5 6] | each window 3 -s 2 { echo $it } | to json --raw
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
        "#
    ));

    assert_eq!(actual.out, "[[1,2,3],[3,4,5]]");
}

#[test]
fn each_no_args_in_block() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
<<<<<<< HEAD
        echo [[foo bar]; [a b] [c d] [e f]] | each { to json } | nth 1 | str collect
        "#
    ));

    assert_eq!(actual.out, r#"{"foo":"c","bar":"d"}"#);
=======
        echo [[foo bar]; [a b] [c d] [e f]] | each {|i| $i | to json -r } | nth 1
        "#
    ));

    assert_eq!(actual.out, r#"{"foo": "c","bar": "d"}"#);
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
}

#[test]
fn each_implicit_it_in_block() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        echo [[foo bar]; [a b] [c d] [e f]] | each { nu --testbin cococo $it.foo }
        "#
    ));

    assert_eq!(actual.out, "ace");
}
