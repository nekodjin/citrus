mod comb;
mod post;

mod field;
mod imgen;

mod clargs;

use clap::Parser;
use clargs::*;
use image::{codecs::gif::GifEncoder, ImageFormat, Rgba};
use std::fs::File;

fn main() {
    let args = Clargs::parse();

    match &args.command {
        Command::Png {
            width: w,
            height: h,
            field: f,
            combinator: c,
            post_processor: p,
            mono_tint: m,
            file,
            scale: s,
        } => match m {
            Some(color) => {
                imgen::png_mono(
                    *w,
                    *h,
                    *s,
                    Rgba::from(color.to_be_bytes()),
                    &*f.as_field(),
                    &*c.as_comb(),
                    &*p.as_post(),
                )
                .save_with_format(file, ImageFormat::Png)
                .unwrap();
            }
            None => {
                imgen::png_color(*w, *h, *s, &*f.as_field(), &*c.as_comb(), &*p.as_post())
                    .save_with_format(file, ImageFormat::Png)
                    .unwrap();
            }
        },
        Command::Gif {
            width: w,
            height: h,
            scale: s,
            z_scale: z,
            mono_tint: m,
            field: f,
            frames,
            combinator: c,
            post_processor: p,
            file,
        } => match m {
            Some(color) => {
                let frames = imgen::gif_mono(
                    *w,
                    *h,
                    *frames,
                    *s,
                    *z,
                    Rgba::from(color.to_be_bytes()),
                    &*f.as_field(),
                    &*c.as_comb(),
                    &*p.as_post(),
                );

                GifEncoder::new_with_speed(File::create(file).unwrap(), 30)
                    .encode_frames(frames.into_iter())
                    .unwrap();
            }
            None => {
                let frames = imgen::gif_color(
                    *w,
                    *h,
                    *frames,
                    *s,
                    *z,
                    &*f.as_field(),
                    &*c.as_comb(),
                    &*p.as_post(),
                );

                GifEncoder::new_with_speed(File::create(file).unwrap(), 30)
                    .encode_frames(frames.into_iter())
                    .unwrap();
            }
        },
    }
}
