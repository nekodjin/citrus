pub trait Comb: seal::Seal {
    fn comb(x: f64, y: f64) -> f64;
}

macro_rules! comb {
    ($name:ident $impl:tt) => {
        pub struct $name;
        impl seal::Seal for $name {}

        impl Comb for $name {
            fn comb(x: f64, y: f64) -> f64 {
                ($impl)(x, y)
            }
        }
    };
}

comb! { Average { |x: f64, y: f64|
    (x + y) / 2.
}}

comb! { Product { |x: f64, y: f64|
    (x * y).sqrt()
}}

mod seal {
    pub trait Seal {}
}
