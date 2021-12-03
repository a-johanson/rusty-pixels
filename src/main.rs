use show_image::{Color, WindowOptions, event, create_window};
use image::{ImageFormat, RgbImage};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let image = RgbImage::from_fn(100, 100, |x, y| {
        if (x + y) % 5 == 0 {
            image::Rgb([0u8, 0u8, 0u8])
        } else {
            image::Rgb([255u8, 255u8, 255u8])
        }
    });

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