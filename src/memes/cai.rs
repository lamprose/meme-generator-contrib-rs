use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn cai(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("cai/0.jpg")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let first_image = images[0].circle().resize_exact((500, 500));
        canvas.draw_image(&first_image, (248, 248), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "cai",
    cai,
    min_images = 1,
    max_images = 1,
    keywords = &["菜"],
    date_created = local_date(2023, 3, 16),
    date_modified = local_date(2023, 3, 16),
);
