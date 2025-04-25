use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn died_most_cai(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("died_most_cai/0.jpg")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let first_image = images[0].circle().resize_exact((44, 44)).rotate(-90.0);
        canvas.draw_image(&first_image, (24, 70), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "died_most_cai",
    died_most_cai,
    min_images = 1,
    max_images = 1,
    keywords = &["死了个最菜的"],
    date_created = local_date(2025, 4, 25),
    date_modified = local_date(2025, 4, 25),
);
