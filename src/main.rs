use show_image::{Color, WindowOptions, event, create_window};
use image::{ImageFormat, RgbImage};

/// r, g, b in [0, 255]
/// alpha in [0.0, 1.0]
fn put_pixel(image: &mut RgbImage, x: i32, y: i32, r: u8, g: u8, b: u8, alpha: f32) {
    if x >= 0 && y >= 0 && (x as u32) < image.dimensions().0 && (y as u32) < image.dimensions().1 {
        let pixel = image.get_pixel_mut(x as u32, y as u32);
        let data = (*pixel as image::Rgb<u8>).0;

        let bg_a = 1.0 - alpha;
        let bg_r = data[0] as f32 * bg_a;
        let bg_g = data[1] as f32 * bg_a;
        let bg_b = data[2] as f32 * bg_a;

        *pixel = image::Rgb([
            (r as f32 * alpha + bg_r) as u8,
            (g as f32 * alpha + bg_g) as u8,
            (b as f32 * alpha + bg_b) as u8
        ]);
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    const WIDTH: i32  = 100;
    const HEIGHT: i32 = 100;

    let mut image = RgbImage::from_fn(WIDTH as u32, HEIGHT as u32, |_x, _y| {
        image::Rgb([35u8, 34u8, 28u8])
    });

    for x in 0..WIDTH {
        put_pixel(&mut image, x, HEIGHT / 2, 242, 238, 230, 0.25);
        put_pixel(&mut image, x + 1, HEIGHT / 2, 242, 238, 230, 0.25);
        put_pixel(&mut image, x + 2, HEIGHT / 2, 242, 238, 230, 0.25);
        put_pixel(&mut image, x + 3, HEIGHT / 2, 242, 238, 230, 0.25);
        put_pixel(&mut image, x, HEIGHT / 2 + 1, 242, 238, 230, 1.0);
    }

    // image.save_with_format("image.png", ImageFormat::Png)?;

    // Create a window with default options and display the image.
    let window = create_window("rusty-pixels", WindowOptions {
        preserve_aspect_ratio: true,
        background_color: Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 },
        start_hidden: false,
        size: None,
        resizable: true,
        borderless: false,
        overlays_visible: false,
        default_controls: false
    })?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }

    Ok(())
}