macro_rules! errors {
    ($($name:ident),* $(,)?) => {
        paste! {
            #[derive(Debug)]
            pub enum Error {
                $(
                    [<$name Error>](String),
                )*
            }

            impl Display for Error {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            impl Error {
                $(
                    pub fn [<$name:snake>] (msg: impl std::fmt::Display) -> Self {
                        Error::[<$name Error>](msg.to_string())
                    }
                )*
            }
        }
    };
}

pub(crate) use errors;
