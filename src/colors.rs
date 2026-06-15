use crate::{Result, TemplateContext};

use material_colors::{
    blend::harmonize,
    color::Argb,
    dynamic_color::Variant,
    hct::Cam16,
    image::{AsPixels, FilterType, ImageReader},
    quantize::{Quantizer, QuantizerCelebi},
    score::Score,
    theme::ThemeBuilder,
};
use std::{collections::HashMap, path::Path};

pub fn generate_material_colors(
    wallpaper_path: &Path,
    theme: &str,
    variant: &str,
    context: &mut TemplateContext,
) -> Result<()> {
    let mut img = ImageReader::open(wallpaper_path).map_err(|err| {
        format!(
            "Could not load wallpaper {}: {err}",
            wallpaper_path.display()
        )
    })?;
    img.resize(112, 112, FilterType::Triangle);
    let pixels = img
        .as_pixels()
        .iter()
        .copied()
        .filter(|argb| argb.alpha == 255)
        .collect::<Vec<_>>();
    let mut quantized = QuantizerCelebi::quantize(&pixels, 128);
    quantized
        .color_to_count
        .retain(|&argb, _| Cam16::from(argb).chroma >= 5.0);
    let ranked = Score::score(&quantized.color_to_count, None, None, None);
    let source_color = ranked.first().copied().unwrap();

    let variant = match variant {
        "monochrome" => Variant::Monochrome,
        "neutral" => Variant::Neutral,
        "tonal_spot" => Variant::TonalSpot,
        "vibrant" => Variant::Vibrant,
        "expressive" => Variant::Expressive,
        "fidelity" => Variant::Fidelity,
        "content" => Variant::Content,
        "rainbow" => Variant::Rainbow,
        "fruit_salad" => Variant::FruitSalad,
        _ => return Err(format!("invalid variant {variant}\nPossible values: \"monochrome\", \"neutral\", \"tonal_spot\", \"vibrant\", \"expressive\", \"fidelity\", \"content\", \"rainbow\", \"fruit_salad\"").into()),
    };

    let color_palette = ThemeBuilder::with_source(source_color)
        .variant(variant)
        .build();

    context.insert("source_color".to_string(), color_palette.source.to_hex());

    match theme {
        "dark" => {
            for (k, v) in color_palette.schemes.dark.into_iter() {
                context.insert(k, v.to_hex());
            }
        }
        "light" => {
            for (k, v) in color_palette.schemes.light.into_iter() {
                context.insert(k, v.to_hex());
            }
        }
        _ => {
            return Err(
                format!("invalid theme {theme}\nPossible values: \"dark\", \"light\"").into(),
            );
        }
    }

    generate_terminal_ansi_colors(context, source_color);
    context.insert("theme".to_string(), theme.to_string());
    Ok(())
}

fn generate_terminal_ansi_colors(config: &mut HashMap<String, String>, source_color: Argb) {
    // default 4-bit ansi colors used by xterm
    let ansi16: [(&str, Argb); 16] = [
        ("black", Argb::new(255, 0, 0, 0)),
        ("red", Argb::new(255, 205, 0, 0)),
        ("green", Argb::new(255, 0, 205, 0)),
        ("yellow", Argb::new(255, 205, 205, 0)),
        ("blue", Argb::new(255, 0, 0, 238)),
        ("magenta", Argb::new(255, 205, 0, 205)),
        ("cyan", Argb::new(255, 0, 205, 205)),
        ("white", Argb::new(255, 229, 229, 229)),
        ("bright_black", Argb::new(255, 127, 127, 127)),
        ("bright_red", Argb::new(255, 255, 0, 0)),
        ("bright_green", Argb::new(255, 0, 255, 0)),
        ("bright_yellow", Argb::new(255, 255, 255, 0)),
        ("bright_blue", Argb::new(255, 92, 92, 255)),
        ("bright_magenta", Argb::new(255, 255, 0, 255)),
        ("bright_cyan", Argb::new(255, 0, 255, 255)),
        ("bright_white", Argb::new(255, 255, 255, 255)),
    ];
    for (name, value) in ansi16.into_iter() {
        let blended_color = harmonize(value, source_color);
        config.insert(name.to_string(), blended_color.to_hex());
    }
}
