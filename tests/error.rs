#[test]
fn should_display_root_error() {
    #[derive(Debug)]
    struct MyError;

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("unable to travel backwards in time")
        }
    }

    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    let error: little_anyhow::Error = MyError.into();

    assert_eq!(format!("{:?}", error), "unable to travel backwards in time");
}

#[test]
fn should_display_error_backtrace() {
    #[derive(Debug)]
    struct MyError(std::io::Error);

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("unable to charge flux capacitor")
        }
    }

    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.0)
        }
    }

    let error: little_anyhow::Error = MyError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "not enough gigawatts",
    ))
    .into();

    assert_eq!(
        format!("{:?}", error),
        r#"
unable to charge flux capacitor

Caused by:
    0: not enough gigawatts
"#
        .trim()
    );
}

#[test]
fn should_increment_backtrace_counter() {
    #[derive(Debug)]
    struct MyInnerError(std::io::Error);

    impl std::fmt::Display for MyInnerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("unable to charge flux capacitor")
        }
    }

    impl std::error::Error for MyInnerError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.0)
        }
    }

    #[derive(Debug)]
    struct MyOuterError(MyInnerError);

    impl std::fmt::Display for MyOuterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("unable to travel backwards in time")
        }
    }

    impl std::error::Error for MyOuterError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.0)
        }
    }

    let error: little_anyhow::Error = MyOuterError(MyInnerError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "not enough gigawatts",
    )))
    .into();

    assert_eq!(
        format!("{:?}", error),
        r#"
unable to travel backwards in time

Caused by:
    0: unable to charge flux capacitor
    1: not enough gigawatts
"#
        .trim()
    );
}
