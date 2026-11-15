//! Embedded, pre-rendered wooden components. No 3D runtime is required.
use std::result::Result::Ok;
use std::{io::Cursor, ptr::null_mut};
use windows_sys::Win32::Graphics::GdiPlus::*;

pub struct Image {
    pub bitmap: *mut GpBitmap,
    pub width: f32,
    pub height: f32,
    // GDI+ borrows these premultiplied BGRA pixels until the bitmap is disposed.
    _pixels: Vec<u8>,
}
impl Image {
    pub unsafe fn load(bytes: &[u8]) -> Result<Self, String> {
        let mut decoder = png::Decoder::new(Cursor::new(bytes));
        decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
        let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
        let mut pixels = vec![0; reader.output_buffer_size().ok_or("Artwork is too large")?];
        let info = reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
        if info.color_type != png::ColorType::Rgba {
            return Err("Artwork must be RGBA".into());
        }
        pixels.truncate(info.buffer_size());
        for p in pixels.as_chunks_mut::<4>().0 {
            p.swap(0, 2);
            let alpha = p[3] as u16;
            for c in &mut p[..3] {
                *c = ((*c as u16 * alpha + 127) / 255) as u8;
            }
        }
        let mut bitmap = null_mut();
        if GdipCreateBitmapFromScan0(
            info.width as i32,
            info.height as i32,
            info.width as i32 * 4,
            0x000e200b,
            pixels.as_ptr(),
            &mut bitmap,
        ) != 0
        {
            return Err("Windows could not load the wooden artwork".into());
        }
        Ok(Self {
            bitmap,
            width: info.width as f32,
            height: info.height as f32,
            _pixels: pixels,
        })
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            GdipDisposeImage(self.bitmap.cast());
        }
    }
}

pub struct Artwork {
    pub house: Image,
    pub pendulum: Image,
    pub visitors: Vec<Image>,
    pub doors: Vec<Image>,
}
impl Artwork {
    pub unsafe fn load() -> Result<Self, String> {
        let house = Image::load(include_bytes!("../assets/woodland/house.png"))?;
        let pendulum = Image::load(include_bytes!("../assets/woodland/pendulum.png"))?;
        let bytes: [&[u8]; 24] = [
            include_bytes!("../assets/woodland/visitor-atlas-00.png"),
            include_bytes!("../assets/woodland/visitor-atlas-01.png"),
            include_bytes!("../assets/woodland/visitor-atlas-02.png"),
            include_bytes!("../assets/woodland/visitor-atlas-03.png"),
            include_bytes!("../assets/woodland/visitor-atlas-04.png"),
            include_bytes!("../assets/woodland/visitor-atlas-05.png"),
            include_bytes!("../assets/woodland/visitor-atlas-06.png"),
            include_bytes!("../assets/woodland/visitor-atlas-07.png"),
            include_bytes!("../assets/woodland/visitor-atlas-08.png"),
            include_bytes!("../assets/woodland/visitor-atlas-09.png"),
            include_bytes!("../assets/woodland/visitor-atlas-10.png"),
            include_bytes!("../assets/woodland/visitor-atlas-11.png"),
            include_bytes!("../assets/woodland/visitor-atlas-12.png"),
            include_bytes!("../assets/woodland/visitor-atlas-13.png"),
            include_bytes!("../assets/woodland/visitor-atlas-14.png"),
            include_bytes!("../assets/woodland/visitor-atlas-15.png"),
            include_bytes!("../assets/woodland/visitor-atlas-16.png"),
            include_bytes!("../assets/woodland/visitor-atlas-17.png"),
            include_bytes!("../assets/woodland/visitor-atlas-18.png"),
            include_bytes!("../assets/woodland/visitor-atlas-19.png"),
            include_bytes!("../assets/woodland/visitor-atlas-20.png"),
            include_bytes!("../assets/woodland/visitor-atlas-21.png"),
            include_bytes!("../assets/woodland/visitor-atlas-22.png"),
            include_bytes!("../assets/woodland/visitor-atlas-23.png"),
        ];
        let mut visitors = Vec::with_capacity(24);
        for bytes in bytes {
            visitors.push(Image::load(bytes)?);
        }
        let door_bytes: [&[u8]; 17] = [
            include_bytes!("../assets/woodland/doors-00.png"),
            include_bytes!("../assets/woodland/doors-01.png"),
            include_bytes!("../assets/woodland/doors-02.png"),
            include_bytes!("../assets/woodland/doors-03.png"),
            include_bytes!("../assets/woodland/doors-04.png"),
            include_bytes!("../assets/woodland/doors-05.png"),
            include_bytes!("../assets/woodland/doors-06.png"),
            include_bytes!("../assets/woodland/doors-07.png"),
            include_bytes!("../assets/woodland/doors-08.png"),
            include_bytes!("../assets/woodland/doors-09.png"),
            include_bytes!("../assets/woodland/doors-10.png"),
            include_bytes!("../assets/woodland/doors-11.png"),
            include_bytes!("../assets/woodland/doors-12.png"),
            include_bytes!("../assets/woodland/doors-13.png"),
            include_bytes!("../assets/woodland/doors-14.png"),
            include_bytes!("../assets/woodland/doors-15.png"),
            include_bytes!("../assets/woodland/doors-16.png"),
        ];
        let mut doors = Vec::with_capacity(17);
        for bytes in door_bytes {
            doors.push(Image::load(bytes)?);
        }
        Ok(Self {
            house,
            pendulum,
            visitors,
            doors,
        })
    }
}
