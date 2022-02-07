use nu_test_support::fs::Stub::FileWithContentToBeTrimmed;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, pipeline};

#[test]
fn condition_is_met() {
    Playground::setup("keep_while_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "caballeros.txt",
            r#"
                CHICKEN SUMMARY                        report date: April 29th, 2020
                --------------------------------------------------------------------
<<<<<<< HEAD
                Chicken Collection,29/04/2020,30/04/2020,31/04/2020,
=======
                Chicken Collection,29/04/2020,30/04/2020,31/04/2020
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
                Yellow Chickens,,,
                Andrés,1,1,1
                Jonathan,1,1,1
                Jason,1,1,1
                Yehuda,1,1,1
                Blue Chickens,,,
                Andrés,1,1,2
                Jonathan,1,1,2
                Jason,1,1,2
                Yehuda,1,1,2
                Red Chickens,,,
                Andrés,1,1,3
                Jonathan,1,1,3
                Jason,1,1,3
                Yehuda,1,1,3
            "#,
        )]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                open --raw caballeros.txt
<<<<<<< HEAD
                | lines
                | skip 2
                | split column ','
                | headers
                | skip 1
                | keep while "Chicken Collection" != "Blue Chickens"
                | str to-int "31/04/2020"
=======
                | lines 
                | skip 2 
                | str trim
                | str collect (char nl) 
                | from csv
                | skip 1
                | keep while "Chicken Collection" != "Blue Chickens"
                | into int "31/04/2020"
>>>>>>> 9259a56a28f1dd3a4b720ad815aa19c6eaf6adce
                | get "31/04/2020"
                | math sum
                "#
        ));

        assert_eq!(actual.out, "4");
    })
}
