use image::{io::Reader as ImageReader, AnimationDecoder, imageops::FilterType, codecs::gif::GifDecoder, DynamicImage};
use std::{io::BufReader, fs::File, time::Duration};

#[derive(Copy, Clone)]
struct Dimensions {
    height: u32,
    width: u32,
}

// TODO actually poll for this, maybe w/ fbset?
fn get_dimensions(_fb_path: &str) -> Dimensions {
    Dimensions {
        height: 128,
        width: 128,
    }
}

fn draw_img(fb_path: &str, fb_dims: Dimensions, img: DynamicImage) {
    let resized_img = img.resize(fb_dims.width, fb_dims.height, FilterType::CatmullRom);
    let width = fb_dims.width.min(resized_img.width());
    let height = fb_dims.height.min(resized_img.height());
    let img_rgba8 = resized_img.as_rgba8().unwrap();
    let mut buf = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let px = img_rgba8.get_pixel(x, y);
            let mut rgb565: u16 = (px[0] as u16 & 0b11111000) << 8;
            rgb565 |= (px[1] as u16 & 0b11111100) << 3;
            rgb565 |= (px[2] as u16) >> 3;
            buf.extend(rgb565.to_le_bytes());
        }
    }
    std::fs::write(fb_path, &buf).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} /path/to/fb /path/to/image", args[0]);
        std::process::exit(1);
    }
    let fb_path = &args[1];
    let img_path = &args[2];

    let fb_dims = get_dimensions(fb_path);

    if img_path.ends_with(".gif") {
        loop {
            // this is dumb and i'm sure there's a better way to loop this
            let stream = BufReader::new(File::open(&img_path).unwrap());
            let decoder = GifDecoder::new(stream).unwrap();
            for maybe_frame in decoder.into_frames() {
                let frame = maybe_frame.unwrap();
                let (numerator, _) = frame.delay().numer_denom_ms();
                let img = DynamicImage::from(frame.into_buffer());
                draw_img(fb_path, fb_dims, img);
                std::thread::sleep(Duration::from_millis(numerator as u64));
            }
        }
    } else {
        let img_reader = ImageReader::open(img_path).unwrap();
        let img = img_reader.decode().unwrap();
        draw_img(fb_path, fb_dims, img);
    }
}
