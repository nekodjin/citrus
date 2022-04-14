pub trait Post: seal::Seal {
    fn post(&self, x: f64) -> f64;
}

macro_rules! post {
    ($name:ident $impl:tt) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name;
        impl seal::Seal for $name {}

        impl Post for $name {
            fn post(&self, x: f64) -> f64 {
                ($impl)(x)
            }
        }
    };
}

post! { Identity { |x: f64|
    x
}}

post! { Raise {
    f64::sqrt
}}

post! { Lower { |x: f64|
    x * x
}}

post! { Converge { |x: f64|
    (4. * x).sqrt() / 2.
}}

post! { Diverge { |x: f64|
    (2. * x).powi(2) / 4.
}}

mod seal {
    pub trait Seal {}
}
