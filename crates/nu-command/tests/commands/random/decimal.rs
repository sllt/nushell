use nu_test_support::{nu, pipeline};

<<<<<<< HEAD
=======
// FIXME: jt: needs more work
#[ignore]
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
#[test]
fn generates_an_decimal() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        random decimal 42..43
        "#
    ));

    assert!(actual.out.contains("42") || actual.out.contains("43"));
}

<<<<<<< HEAD
=======
// FIXME: jt: needs more work
#[ignore]
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
#[test]
fn generates_55() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        random decimal 55..55
        "#
    ));

    assert!(actual.out.contains("55"));
}

<<<<<<< HEAD
=======
// FIXME: jt: needs more work
#[ignore]
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
#[test]
fn generates_0() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        random decimal ..<1
        "#
    ));

    assert!(actual.out.contains('0'));
}
