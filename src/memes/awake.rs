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

const DEFAULT_TEXT: &str = "上仙快醒醒,我们为你准备了大型氪金活动";

fn awake(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let locs = [
        (54, 51, -28.8),
        (46, 61, -39.77),
        (42, 71, -51.54),
        (45, 59, -39.35),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let pic_type = if images.len() == 2 { "" } else { "1" };
        let (x, y, d) = locs[i];
        let logo = images[0].round_corner(5.0).resize_exact((56, 56));
        let head = images[1].circle().resize_exact((51, 51)).rotate(d);
        let frame = load_image(format!("awake/{pic_type}{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if images.len() == 3 {
            let head2 = images[2].circle().resize_exact((83, 83));
            canvas.clear(Color::WHITE);
            canvas.draw_image(&head2, (116, 4), None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&logo, (3, 3), None);
        canvas.draw_image(&head, (x, y), None);

        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 170, 210, 220),
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
            frame_num: 4,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "awake",
    awake,
    min_images = 2,
    max_images = 3,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["醒醒"],
    date_created = local_date(2025, 4, 27),
    date_modified = local_date(2025, 4, 28),
);
