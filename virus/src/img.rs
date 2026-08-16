// src/img.rs

use std::time::Duration;
use show_image::{ImageView, create_window}; 
use image::AnimationDecoder;

pub fn gif() -> Result<(), Box<dyn std::error::Error>> {
    
    let gif_bytes = include_bytes!("giphy.gif"); 
    let reader = std::io::Cursor::new(gif_bytes); 
    
    let decoder = image::codecs::gif::GifDecoder::new(reader)?;
    let frames = decoder.into_frames().collect_frames()?;

    
    let window = create_window("GIF Player", Default::default())?;

    println!("Playing GIF. Close the window to exit.");
    
    loop {
        for frame in &frames {

            let buffer = frame.buffer();
            let image_view = ImageView::new(
                show_image::ImageInfo::rgba8(buffer.width(), buffer.height()),
                buffer.as_raw()
            );

            
            window.set_image("gif-frame", image_view)?;

            
            let delay: Duration = frame.delay().into();
            std::thread::sleep(delay);
        }
    }
}