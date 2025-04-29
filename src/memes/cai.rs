use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT_1: &str = "菜就哭呀";
const DEFAULT_TEXT_2: &str = "练有几把用";
fn cai(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("cai/0.jpg")?;
    let text1 = if texts.len() > 0 {
        &texts[0]
    } else {
        DEFAULT_TEXT_1
    };
    let text2 = if texts.len() > 1 {
        &texts[1]
    } else {
        DEFAULT_TEXT_2
    };

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let first_image = images[0].resize_exact((50, 50));
        canvas.draw_image(&first_image, (36, 22), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 77, 128, 93),
            text1,
            10.0,
            60.0,
            text_params!(paint = new_paint(Color::from_rgb(0, 0, 0))),
        )?;
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 93, 128, 109),
            text2,
            10.0,
            60.0,
            text_params!(paint = new_paint(Color::from_rgb(0, 0, 0))),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "cai",
    cai,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 2,
    default_texts = &[DEFAULT_TEXT_1, DEFAULT_TEXT_2],
    keywords = &["菜"],
    date_created = local_date(2025, 4, 25),
    date_modified = local_date(2025, 4, 29),
);
