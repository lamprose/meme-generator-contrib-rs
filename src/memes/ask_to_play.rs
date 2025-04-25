use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn ask_to_play(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("why_at_me/0.png")?;
    let text = texts[0];
    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0]
            .square()
            .resize_exact((265, 265))
            .rotate_crop(-19.0);
        canvas.draw_image(&img, (42, 13), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(80, 410, 182, 513),
            text,
            25.0,
            60.0,
            text_params!(paint = new_paint(Color::from_rgb(111, 95, 95))),
        );
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "ask_to_play",
    ask_to_play,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["说 为什么@我"],
    keywords = &["为什么@我"],
    date_created = local_date(2025, 4, 25),
    date_modified = local_date(2025, 4, 25),
);
