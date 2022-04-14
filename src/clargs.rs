use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, disable_help_flag(true))]
pub struct Clargs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Png {
        #[clap(short, long)]
        width: u32,

        #[clap(short, long)]
        height: u32,

        #[clap(short, long, parse(try_from_str = parse_hex))]
        mono_tint: Option<u32>,

        #[clap(arg_enum, short, long)]
        field: Field,

        #[clap(arg_enum, short, long, default_value_t = Comb::Normal)]
        combinator: Comb,

        #[clap(arg_enum, short, long, default_value_t = Post::Identity)]
        post_processor: Post,

        #[clap(short, long, default_value_t = 35.)]
        scale: f64,

        file: String,
    },
    Gif {
        #[clap(short, long)]
        width: u32,

        #[clap(short, long)]
        height: u32,

        #[clap(short, long, default_value_t = 35.)]
        scale: f64,

        #[clap(short, long, default_value_t = 30.)]
        z_scale: f64,

        #[clap(short, long, parse(try_from_str = parse_hex))]
        mono_tint: Option<u32>,

        #[clap(arg_enum, short, long)]
        field: Field,

        #[clap(long)]
        frames: u16,

        #[clap(arg_enum, short, long, default_value_t = Comb::Normal)]
        combinator: Comb,

        #[clap(arg_enum, short, long, default_value_t = Post::Identity)]
        post_processor: Post,

        file: String,
    },
}

#[derive(ArgEnum, Debug, Copy, Clone)]
pub enum Field {
    Fbm,
    OpenSimplex,
    SuperSimplex,
    Perlin,
    Worley,
    Value,
}

impl Field {
    pub fn as_field(&self) -> Box<dyn crate::field::Field> {
        use crate::field::*;

        match self {
            Self::Fbm => Box::new(Fbm),
            Self::OpenSimplex => Box::new(OpenSimplex),
            Self::SuperSimplex => Box::new(SuperSimplex),
            Self::Perlin => Box::new(Perlin),
            Self::Worley => Box::new(Worley),
            Self::Value => Box::new(Value),
        }
    }
}

#[derive(ArgEnum, Debug, Copy, Clone)]
pub enum Comb {
    Normal,
    Inverse,
    Average,
    Product,
}

impl Comb {
    pub fn as_comb(&self) -> Box<dyn crate::comb::Comb> {
        use crate::comb::*;

        match self {
            Self::Normal => Box::new(Normal),
            Self::Inverse => Box::new(Inverse),
            Self::Average => Box::new(Average),
            Self::Product => Box::new(Product),
        }
    }
}

#[derive(ArgEnum, Debug, Copy, Clone)]
pub enum Post {
    Identity,
    Raise,
    Lower,
    Converge,
    Diverge,
}

impl Post {
    pub fn as_post(&self) -> Box<dyn crate::post::Post> {
        use crate::post::*;

        match self {
            Self::Identity => Box::new(Identity),
            Self::Raise => Box::new(Raise),
            Self::Lower => Box::new(Lower),
            Self::Converge => Box::new(Converge),
            Self::Diverge => Box::new(Diverge),
        }
    }
}

fn parse_hex(s: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(s, 16)
}
