
pub mod err {
    use std::fmt;
    use std::error::Error;



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

    impl Error for TokenError {

    }

}

