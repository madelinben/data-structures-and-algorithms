use crate::prelude::*;

pub struct FrameRenderer {
    width: u16,
    height: u16,
}

impl FrameRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    
    pub fn render_static_frame(&self, data: &[i32], highlighted: &[usize]) -> Result<Vec<u8>> {
        let mut buffer = vec![255u8; (self.width as usize) * (self.height as usize) * 3];
        
        if data.is_empty() {
            return Ok(buffer);
        }
        
        let max_value = data.iter().max().copied().unwrap_or(100) as f64;
        let bar_width = (self.width as f64 - 20.0) / data.len() as f64;
        let height_scale = (self.height as f64 - 80.0) / max_value;
        
        for (i, &value) in data.iter().enumerate() {
            let bar_height = (value as f64 * height_scale) as usize;
            let x_start = (10.0 + i as f64 * bar_width) as usize;
            let x_end = (10.0 + (i + 1) as f64 * bar_width - 1.0) as usize;
            let y_start = self.height as usize - 40 - bar_height;
            let y_end = self.height as usize - 40;
            
            let (r, g, b) = if highlighted.contains(&i) {
                (255, 100, 100)
            } else {
                (100, 150, 255)
            };
            
            for y in y_start..y_end {
                for x in x_start..=x_end.min(self.width as usize - 1) {
                    if y < self.height as usize && x < self.width as usize {
                        let idx = (y * self.width as usize + x) * 3;
                        if idx + 2 < buffer.len() {
                            buffer[idx] = r;
                            buffer[idx + 1] = g;
                            buffer[idx + 2] = b;
                        }
                    }
                }
            }
        }
        
        Ok(buffer)
    }
}

pub mod gif_renderer {
    use super::*;
    use gif::{Frame, Encoder, Repeat};
    use std::fs::File;

    pub fn save_gif(frames: &[Vec<u8>], width: u16, height: u16, filename: &str) -> Result<()> {
        let file = File::create(filename).map_err(|e| Error::generic(format!("Failed to create file: {}", e)))?;
        let mut encoder = Encoder::new(file, width, height, &[]).map_err(|e| Error::generic(format!("GIF encoder error: {}", e)))?;
        encoder.set_repeat(Repeat::Infinite).map_err(|e| Error::generic(format!("GIF repeat error: {}", e)))?;

        for frame_data in frames {
            let frame = Frame::from_rgb(width, height, frame_data);
            encoder.write_frame(&frame).map_err(|e| Error::generic(format!("Frame write error: {}", e)))?;
        }

        drop(encoder);
        Ok(())
    }
}
