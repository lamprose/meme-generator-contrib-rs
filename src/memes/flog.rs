use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn flog(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = if !images[0].name.is_empty() {
        &images[0].name
    } else {
        "黑奴"
    };

    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        &format!("{name},干活!")
    };
    let locs = [
        (91, 311, 157, 157),
        (90, 308, 153, 153),
        (91, 311, 157, 157),
        (90, 308, 153, 153),
        (91, 311, 157, 157),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let (x, y, w, h) = locs[i];
        let frame = load_image(format!("flog/{i}.png"))?;
        let teardrop = load_image(format!("flog/1{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        canvas.draw_image(&frame, (0, 0), None);
        if !images.is_empty() {
            let head = images[0].circle().resize_exact((w, h));
            canvas.draw_image(&head, (x, y), None);
        }
        canvas.draw_image(&teardrop, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 18, 805, 154),
            text,
            10.0,
            50.0,
            None,
        )?;
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "flog",
    flog,
    min_images = 0,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["黑奴,干活!"],
    keywords = &["鞭笞"],
    date_created = local_date(2025, 4, 29),
    date_modified = local_date(2025, 4, 29),
);
