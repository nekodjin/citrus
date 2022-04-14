use noise::{Negate, NoiseFn, Seedable};
use rand::random;

pub trait Field: seal::Seal {
    fn field(&self) -> &dyn NoiseFn<[f64; 3]>;

    fn normal(&self, x: f64, y: f64, z: f64) -> f64 {
        self.field().get([x, y, z])
    }

    fn inverse(&self, x: f64, y: f64, z: f64) -> f64 {
        Negate::new(self.field()).get([x, y, z])
    }
}

pub trait FieldInit: seal::Seal {
    fn new() -> Box<dyn Field>;
}

macro_rules! simple_field {
    ($name:ident $hide:ident $impl:ty) => {
        pub struct $name;
        impl seal::Seal for $name {}

        struct $hide(Box<dyn NoiseFn<[f64; 3]>>);
        impl seal::Seal for $hide {}

        impl FieldInit for $name {
            fn new() -> Box<dyn Field> {
                let noise = <$impl>::new().set_seed(random());
                let field = $hide(Box::new(noise));
                Box::new(field)
            }
        }

        impl Field for $hide {
            fn field(&self) -> &dyn NoiseFn<[f64; 3]> {
                &*self.0
            }
        }
    };
}

macro_rules! simple_fields {
    ($($name:ident $hide:ident $impl:ty);*$(;)?) => {$(
        simple_field!{$name $hide $impl}
    )*}
}

simple_fields! {
    Fbm            FbmInit            noise::Fbm;
    OpenSimplex    OpenSimplexInit    noise::OpenSimplex;
    SuperSimplex   SuperSimplexInit   noise::SuperSimplex;
    Perlin         PerlinInit         noise::Perlin;
    Value          ValueInit          noise::Value;
    Worley         WorleyInit         noise::Worley;
}

mod seal {
    pub trait Seal {}
}
