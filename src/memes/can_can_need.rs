use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn can_can_need(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("can_can_need/0.jpg")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let first_image = images[0].circle().resize_exact((300, 300));
        let second_image = images[1].circle().resize_exact((340, 340));
        canvas.draw_image(&second_image, (120, 21), None);
        canvas.draw_image(&first_image, (611, 718), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "can_can_need",
    can_can_need,
    min_images = 2,
    max_images = 2,
    keywords = &["看看你的"],
    date_created = local_date(2023, 3, 16),
    date_modified = local_date(2023, 3, 16),
);
