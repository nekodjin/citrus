pub trait Comb: seal::Seal {
    fn comb(&self, x: f64, y: f64) -> f64;
}

macro_rules! comb {
    ($name:ident $impl:tt) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name;
        impl seal::Seal for $name {}

        impl Comb for $name {
            fn comb(&self, x: f64, y: f64) -> f64 {
                ($impl)(x, y)
            }
        }
    };
}

comb! { Normal { |x: f64, _: f64|
    x
}}

comb! { Inverse { |_: f64, y: f64|
    y
}}

comb! { Average { |x: f64, y: f64|
    (x + y) / 2.
}}

comb! { Product { |x: f64, y: f64|
    (x * y).sqrt()
}}

mod seal {
    pub trait Seal {}
}
