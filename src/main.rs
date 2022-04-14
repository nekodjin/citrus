mod comb;
mod post;

mod field;
mod imgen;

fn main() {
    let frames = imgen::gif_color(
        600,
        600,
        100,
        35.,
        35.,
        &field::Worley,
        &comb::Average,
        &post::Identity,
    );

    image::codecs::gif::GifEncoder::new_with_speed(
        std::fs::File::create("output.gif").unwrap(),
        30,
    )
    .encode_frames(frames.into_iter())
    .unwrap();

    // imgen::png_color(
    //     600,
    //     600,
    //     35.,
    //     &field::Worley,
    //     &comb::Average,
    //     &post::Diverge,
    // )
    // .save("test.png")
    // .unwrap();
}
