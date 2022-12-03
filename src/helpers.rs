pub mod err {
    use std::fmt;
    #[macro_export] macro_rules! tokenerr {
        ($expected:tt, $reason:tt) => {{
            err::TokenError{
                token: $expected
                reason: $reason
            }
        }};
    }

    pub use tokenerr;
    pub trait Error = fmt::Debug + fmt::Display;

    // Now we will be able to write our own errors, defer to an underlying error
    #[derive(Debug, Clone)]
    pub struct TokenError {
        pub token: String,
        pub reason: String,
    }

    impl fmt::Display for TokenError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(format_args!("Could not parse {} because {}", &self.token, &self.reason))
        }
    }

}

