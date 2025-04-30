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

fn want_this(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("want_this/0.png")?;
    let name = if !images[0].name.is_empty() {
        &images[0].name
    } else {
        "群主"
    };

    let text = if !texts.is_empty() {
        texts[0]
    } else {
        format!("{name}我要这个")
    };

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let first_image = images[0].resize_exact((81, 81));
        canvas.draw_image(&first_image, (9, 8), None);
        canvas.draw_image(&frame, (0, 0), None);

        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 139, 198, 176),
            &text,
            10.0,
            60.0,
            text_params!(paint = new_paint(Color::from_rgb(0, 0, 0))),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "want_this",
    want_this,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["群主我要这个"],
    keywords = &["要这个"],
    date_created = local_date(2025, 4, 30),
    date_modified = local_date(2025, 4, 30),
);
