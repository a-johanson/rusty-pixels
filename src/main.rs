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

fn draw_point(image: &mut RgbImage, x: f32, y: f32, r: u8, g: u8, b: u8) {
    let f_x = x.fract();
    let f_y = y.fract();
    let t_x = x as i32;
    let t_y = y as i32;

    put_pixel(image, t_x, t_y, r, g, b, (1.0 - f_x) * (1.0 - f_y));
    put_pixel(image, t_x + 1, t_y, r, g, b, f_x * (1.0 - f_y));
    put_pixel(image, t_x, t_y + 1, r, g, b, (1.0 - f_x) * f_y);
    put_pixel(image, t_x + 1, t_y + 1, r, g, b, f_x * f_y);
}

fn draw_circle_msaa(image: &mut RgbImage, c_x: f32, c_y: f32, radius: f32, r: u8, g: u8, b: u8) {
    // This algorithm is not very efficient as it multi-samples also "inner" pixels
    // Probably could be avoided using a scan line approach

    // Determine bounding box
    let x_min = (c_x - radius).max(0.0) as i32;
    let x_max = (c_x + radius).ceil().min(image.dimensions().0 as f32) as i32;
    let y_min = (c_y - radius).max(0.0) as i32;
    let y_max = (c_y + radius).ceil().min(image.dimensions().1 as f32) as i32;

    // Iterate over all pixels
    for x_i in x_min..x_max {
        for y_i in y_min..y_max {
            let x = x_i as f32;
            let y = y_i as f32;
            let mut alpha = 0.0;
            for inc_x in [0.25, 0.75] {
                for inc_y in [0.25, 0.75] {
                    let x = x + inc_x;
                    let y = y + inc_y;
                    if ((c_x - x).powi(2) + (c_y - y).powi(2)).sqrt() <= radius {
                        alpha += 0.25; // = 1.0 / (sample_count_x * sample_count_y)
                    }
                }
            }
            put_pixel(image, x_i, y_i, r, g, b, alpha);
        }
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

    draw_point(&mut image, 0.0, 0.0, 242, 238, 230);
    draw_point(&mut image, 2.0, 1.75, 242, 238, 230);
    draw_point(&mut image, 10.75, 11.0, 242, 238, 230);
    draw_point(&mut image, 3.5, 3.5, 242, 238, 230);

    draw_circle_msaa(&mut image, 75.0, 75.0, 24.5, 242, 238, 230);

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