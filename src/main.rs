use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use clap::{Parser, ValueHint};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author="Church Tao", version
, about="A simple image generator."
, long_about = None)]
struct Args {
    /// Sets the width of the image
    #[clap(short, long, value_name = "WIDTH", default_value_t = 800)]
    width: u32,

    /// Sets the height of the image
    #[clap(short, long, value_name = "HEIGHT", default_value_t = 600)]
    height: u32,

    /// Sets the output format (png, jpg, etc.)
    #[clap(short, long, value_name = "FORMAT", default_value = "png")]
    format: String,

    /// Sets the output filename
    #[clap(short = 'n', long, value_name = "FILENAME", default_value = "output")]
    filename: String,

    /// Sets the output directory
    #[clap(short, long, value_name = "OUTDIR", value_hint = ValueHint::DirPath, default_value = ".")]
    outdir: PathBuf,
}

fn main() {
    let args = Args::parse();

    if (args.width == 0) || (args.height == 0) {
        eprintln!("Width and height must be greater than 0");
        return;
    }

    let mut img =
        ImageBuffer::from_pixel(args.width, args.height, Rgba([255u8, 255u8, 255u8, 255u8]));

    // 使用 fontconfig 查找系统字体
    let font = FontRef::try_from_slice(include_bytes!("./DejaVuSans.ttf")).unwrap();
    // 根据图片大小，设置字体大小，占比50%
    let scale = PxScale {
        x: (args.height as f32) * 0.2,
        y: (args.height as f32) * 0.2,
    };
    let scaled_font = font.as_scaled(scale);
    // 绘制文本
    let text1 = format!("{}x{}", args.width, args.height);

    // 计算文字的宽度和高度
    let glyph_id = scaled_font.glyph_id(text1.chars().next().unwrap());
    let text1_width = scaled_font.h_advance(glyph_id) * text1.len() as f32;
    let text1_height = scaled_font.height();
    // 居中设置文字
    let x = args.width as f32 * 0.5 - text1_width * 0.5;
    let y = args.height as f32 * 0.5 - text1_height * 0.5;
    draw_text_mut(
        &mut img,
        Rgba([0, 0, 0, 255]),
        x as i32,
        y as i32,
        scale,
        &font,
        &text1,
    );

    let mut output_path = args.outdir;
    output_path.push(format!("{}.{}", args.filename, args.format));

    let save_path = output_path.clone(); // 克隆 output_path

    match img.save(&save_path) {
        Ok(_) => println!("Image saved successfully to {}", output_path.display()),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
