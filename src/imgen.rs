use crate::comb::Comb;
use crate::field::{Field, FieldImplementation};
use crate::post::Post;
use image::{Frame, Rgba, RgbaImage};

pub fn png_mono(
    w: u32,
    h: u32,
    scale: f64,
    color: Rgba<u8>,
    field: &dyn Field,
    comb: &dyn Comb,
    post: &dyn Post,
) -> RgbaImage {
    let mut buffer = RgbaImage::new(w, h);
    let a = field.new();
    let b = field.new();

    for w in 0..w {
        for h in 0..h {
            let value = post.post(comb.comb(
                a.normal(w as f64 / scale, h as f64 / scale, 0.),
                b.inverse(w as f64 / scale, h as f64 / scale, 0.),
            ));

            let pixel = Rgba::from([
                (value * color.0[0] as f64) as u8,
                (value * color.0[1] as f64) as u8,
                (value * color.0[2] as f64) as u8,
                0xff,
            ]);

            buffer[(w, h)] = pixel;
        }
    }

    buffer
}

pub fn png_color(
    w: u32,
    h: u32,
    scale: f64,
    field: &dyn Field,
    comb: &dyn Comb,
    post: &dyn Post,
) -> RgbaImage {
    let mut buffer = RgbaImage::new(w, h);

    let r_a = field.new();
    let g_a = field.new();
    let b_a = field.new();
    let r_b = field.new();
    let g_b = field.new();
    let b_b = field.new();

    for w in 0..w {
        for h in 0..h {
            let value = |x: &Box<dyn FieldImplementation>, y: &Box<dyn FieldImplementation>| {
                256. * post.post(comb.comb(
                    x.normal(w as f64 / scale, h as f64 / scale, 0.),
                    y.inverse(w as f64 / scale, h as f64 / scale, 0.),
                ))
            };

            let pixel = Rgba::from([
                value(&r_a, &r_b) as u8,
                value(&g_a, &g_b) as u8,
                value(&b_a, &b_b) as u8,
                0xff,
            ]);

            buffer[(w, h)] = pixel;
        }
    }

    buffer
}

pub fn gif_mono(
    w: u32,
    h: u32,
    frames: u16,
    scale: f64,
    z_scale: f64,
    color: Rgba<u8>,
    field: &dyn Field,
    comb: &dyn Comb,
    post: &dyn Post,
) -> Vec<Frame> {
    let mut frame_vec = Vec::new();
    let a = field.new();
    let b = field.new();

    for z in 0..frames {
        println!("Frame {z} of {frames}");

        let mut buffer = RgbaImage::new(w, h);

        for w in 0..w {
            for h in 0..h {
                let value = post.post(comb.comb(
                    a.normal(w as f64 / scale, h as f64 / scale, z as f64 / z_scale),
                    b.inverse(w as f64 / scale, h as f64 / scale, z as f64 / z_scale),
                ));

                let pixel = Rgba::from([
                    (value * color.0[0] as f64) as u8,
                    (value * color.0[1] as f64) as u8,
                    (value * color.0[2] as f64) as u8,
                    0xff,
                ]);

                buffer[(w, h)] = pixel;
            }
        }

        frame_vec.push(Frame::new(buffer));
    }

    frame_vec
}

pub fn gif_color(
    w: u32,
    h: u32,
    frames: u16,
    scale: f64,
    z_scale: f64,
    field: &dyn Field,
    comb: &dyn Comb,
    post: &dyn Post,
) -> Vec<Frame> {
    let mut frame_vec = Vec::new();

    let r_a = field.new();
    let g_a = field.new();
    let b_a = field.new();
    let r_b = field.new();
    let g_b = field.new();
    let b_b = field.new();

    for z in 0..frames {
        println!("Frame {z} of {frames}");

        let mut buffer = RgbaImage::new(w, h);

        for w in 0..w {
            for h in 0..h {
                let value = |x: &Box<dyn FieldImplementation>, y: &Box<dyn FieldImplementation>| {
                    256. * post.post(comb.comb(
                        x.normal(w as f64 / scale, h as f64 / scale, z as f64 / z_scale),
                        y.inverse(w as f64 / scale, h as f64 / scale, z as f64 / z_scale),
                    ))
                };

                let pixel = Rgba::from([
                    value(&r_a, &r_b) as u8,
                    value(&g_a, &g_b) as u8,
                    value(&b_a, &b_b) as u8,
                    0xff,
                ]);

                buffer[(w, h)] = pixel;
            }
        }

        frame_vec.push(Frame::new(buffer));
    }

    frame_vec
}
