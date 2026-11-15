//! Native GDI+ composition of rendered wooden components and live clock hands.
#![allow(clippy::too_many_arguments)] // Geometry helpers follow the GDI+ coordinate API.
use crate::{
    artwork::{Artwork, Image},
    model::{visit_pose, Settings},
};
use chrono::{DateTime, Local, Timelike};
use std::result::Result::Ok;
use std::{
    f32::consts::PI,
    ptr::{null, null_mut},
};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::{Gdi::*, GdiPlus::*},
    UI::WindowsAndMessaging::*,
};
pub const WIDTH: f32 = 400.;
pub const HEIGHT: f32 = 575.;
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub struct Surface {
    pub dc: HDC,
    bitmap: HBITMAP,
    old: HGDIOBJ,
    image: *mut GpBitmap,
    pub g: *mut GpGraphics,
    pub width: i32,
    pub height: i32,
    family: *mut GpFontFamily,
}
impl Surface {
    pub unsafe fn new(width: i32, height: i32) -> Result<Self, String> {
        let dc = CreateCompatibleDC(null_mut());
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = width;
        info.bmiHeader.biHeight = -height;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let mut bits = null_mut();
        let bitmap = CreateDIBSection(dc, &info, DIB_RGB_COLORS, &mut bits, null_mut(), 0);
        if bitmap.is_null() {
            DeleteDC(dc);
            return Err("Windows could not allocate the clock canvas".into());
        }
        let old = SelectObject(dc, bitmap);
        let mut image = null_mut();
        let mut g = null_mut();
        let mut family = null_mut();
        let result = GdipCreateBitmapFromScan0(
            width,
            height,
            width * 4,
            0x000e200b,
            bits as *const u8,
            &mut image,
        );
        if result != 0 || GdipGetImageGraphicsContext(image.cast(), &mut g) != 0 {
            if !image.is_null() {
                GdipDisposeImage(image.cast());
            }
            SelectObject(dc, old);
            DeleteObject(bitmap);
            DeleteDC(dc);
            return Err("GDI+ could not create the clock canvas".into());
        }
        GdipSetSmoothingMode(g, SmoothingModeAntiAlias8x8);
        GdipSetTextRenderingHint(g, TextRenderingHintAntiAlias);
        GdipSetInterpolationMode(g, InterpolationModeHighQualityBicubic);
        GdipCreateFontFamilyFromName(wide("Bahnschrift").as_ptr(), null_mut(), &mut family);
        Ok(Self {
            dc,
            bitmap,
            old,
            image,
            g,
            width,
            height,
            family,
        })
    }
    pub unsafe fn clear(&self, color: u32) {
        GdipResetWorldTransform(self.g);
        GdipGraphicsClear(self.g, color);
    }
    pub unsafe fn transform(&self, scale: f32, x: f32, y: f32, angle: f32) {
        let a = angle * PI / 180.;
        let mut m = null_mut();
        GdipCreateMatrix2(
            scale * a.cos(),
            scale * a.sin(),
            -scale * a.sin(),
            scale * a.cos(),
            x,
            y,
            &mut m,
        );
        GdipSetWorldTransform(self.g, m);
        GdipDeleteMatrix(m);
    }
    pub unsafe fn ellipse(&self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        let mut b = null_mut();
        GdipCreateSolidFill(0xff000000 | color, &mut b);
        GdipFillEllipse(self.g, b.cast(), x, y, w, h);
        GdipDeleteBrush(b.cast());
    }
    pub unsafe fn alpha_ellipse(&self, x: f32, y: f32, w: f32, h: f32, argb: u32) {
        let mut b = null_mut();
        GdipCreateSolidFill(argb, &mut b);
        GdipFillEllipse(self.g, b.cast(), x, y, w, h);
        GdipDeleteBrush(b.cast());
    }
    pub unsafe fn poly(&self, pts: &[(f32, f32)], c: u32) {
        let points: Vec<PointF> = pts.iter().map(|&(x, y)| PointF { X: x, Y: y }).collect();
        let mut b = null_mut();
        GdipCreateSolidFill(0xff000000 | c, &mut b);
        GdipFillPolygon(
            self.g,
            b.cast(),
            points.as_ptr(),
            points.len() as i32,
            FillModeWinding,
        );
        GdipDeleteBrush(b.cast());
    }
    pub unsafe fn line(&self, x: f32, y: f32, x2: f32, y2: f32, width: f32, color: u32) {
        let mut p = null_mut();
        GdipCreatePen1(0xff000000 | color, width, UnitPixel, &mut p);
        GdipSetPenStartCap(p, LineCapRound);
        GdipSetPenEndCap(p, LineCapRound);
        GdipDrawLine(self.g, p, x, y, x2, y2);
        GdipDeletePen(p);
    }
    pub unsafe fn text(&self, s: &str, x: f32, y: f32, w: f32, size: f32, c: u32, bold: bool) {
        let mut font = null_mut();
        let mut b = null_mut();
        let mut fmt = null_mut();
        GdipCreateFont(
            self.family,
            size,
            if bold { 1 } else { 0 },
            UnitPixel,
            &mut font,
        );
        GdipCreateSolidFill(0xff000000 | c, &mut b);
        GdipCreateStringFormat(0, 0, &mut fmt);
        GdipSetStringFormatAlign(fmt, StringAlignmentCenter);
        let r = RectF {
            X: x,
            Y: y,
            Width: w,
            Height: size * 1.8,
        };
        let t = wide(s);
        GdipDrawString(
            self.g,
            t.as_ptr(),
            (t.len() - 1) as i32,
            font,
            &r,
            fmt,
            b.cast(),
        );
        GdipDeleteStringFormat(fmt);
        GdipDeleteBrush(b.cast());
        GdipDeleteFont(font);
    }
    pub unsafe fn image(&self, image: &Image, x: f32, y: f32, w: f32, h: f32) {
        GdipDrawImageRectRect(
            self.g,
            image.bitmap.cast(),
            x,
            y,
            w,
            h,
            0.,
            0.,
            image.width,
            image.height,
            UnitPixel,
            null(),
            0,
            null_mut(),
        );
    }
    pub unsafe fn image_region(
        &self,
        image: &Image,
        destination: (f32, f32, f32, f32),
        source: (f32, f32, f32, f32),
    ) {
        let (x, y, w, h) = destination;
        let (sx, sy, sw, sh) = source;
        GdipDrawImageRectRect(
            self.g,
            image.bitmap.cast(),
            x,
            y,
            w,
            h,
            sx,
            sy,
            sw,
            sh,
            UnitPixel,
            null(),
            0,
            null_mut(),
        );
    }
    pub unsafe fn clip_doorway(&self) {
        let mut path = null_mut();
        GdipCreatePath(FillModeWinding, &mut path);
        GdipAddPathArc(path, 143., 104., 114., 114., 180., 180.);
        GdipAddPathLine(path, 257., 161., 257., 225.);
        GdipAddPathLine(path, 257., 225., 143., 225.);
        GdipAddPathLine(path, 143., 225., 143., 161.);
        GdipClosePathFigure(path);
        GdipSetClipPath(self.g, path, CombineModeIntersect);
        GdipDeletePath(path);
    }
    pub unsafe fn shadow_poly(&self, points: &[(f32, f32)]) {
        let points: Vec<PointF> = points.iter().map(|&(x, y)| PointF { X: x, Y: y }).collect();
        let mut b = null_mut();
        GdipCreateSolidFill(0x350d1819, &mut b);
        GdipFillPolygon(
            self.g,
            b.cast(),
            points.as_ptr(),
            points.len() as i32,
            FillModeWinding,
        );
        GdipDeleteBrush(b.cast());
    }
    pub unsafe fn present(&self, hwnd: HWND) -> bool {
        GdipFlush(self.g, FlushIntentionSync);
        let size = SIZE {
            cx: self.width,
            cy: self.height,
        };
        let src = POINT { x: 0, y: 0 };
        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };
        UpdateLayeredWindow(
            hwnd,
            null_mut(),
            null(),
            &size,
            self.dc,
            &src,
            0,
            &blend,
            ULW_ALPHA,
        ) != 0
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            GdipDeleteFontFamily(self.family);
            GdipDeleteGraphics(self.g);
            GdipDisposeImage(self.image.cast());
            SelectObject(self.dc, self.old);
            DeleteObject(self.bitmap);
            DeleteDC(self.dc);
        }
    }
}

pub unsafe fn character(d: &Surface, id: usize, frame: usize, art: &Artwork) {
    let tile = 192.;
    d.image_region(
        &art.visitors[id % 24],
        (-80., -137.143, 160., 160.),
        (
            (frame % 5) as f32 * tile,
            (frame / 5) as f32 * tile,
            tile,
            tile,
        ),
    );
}

pub struct ClockFrame<'a> {
    pub settings: &'a Settings,
    pub now: DateTime<Local>,
    pub time: f32,
    pub visitor: usize,
    pub visit: Option<(f32, f32)>,
}

unsafe fn hand(d: &Surface, angle: f32, length: f32, width: f32, color: u32) {
    let points = [
        (-width * 0.45, 10.),
        (-width, 0.),
        (-width * 0.60, -length * 0.76),
        (0., -length),
        (width * 0.60, -length * 0.76),
        (width, 0.),
        (width * 0.45, 10.),
    ];
    let rotate = |x: f32, y: f32| {
        (
            200. + x * angle.cos() - y * angle.sin(),
            327. + x * angle.sin() + y * angle.cos(),
        )
    };
    let shape: Vec<_> = points.iter().map(|&(x, y)| rotate(x, y)).collect();
    let shadow: Vec<_> = shape.iter().map(|&(x, y)| (x + 1.5, y + 2.)).collect();
    d.shadow_poly(&shadow);
    d.poly(&shape, color);
    let (x, y) = rotate(-width * 0.35, 0.);
    let (xx, yy) = rotate(-width * 0.24, -length * 0.71);
    d.line(x, y, xx, yy, 0.65, 0x718783);
}

pub unsafe fn clock(d: &Surface, frame: ClockFrame<'_>, art: &Artwork) {
    let ClockFrame {
        settings: s,
        now,
        time,
        visitor,
        visit,
    } = frame;
    d.clear(0);
    let scale = d.width as f32 / WIDTH;
    // The anchor is hidden inside the case; the bob swings naturally below it.
    let swing = if s.motion {
        (time * 2.8).sin() * 3.5
    } else {
        0.
    };
    d.transform(scale, 200. * scale, 365. * scale, swing);
    d.image(&art.pendulum, -64., -16., 128., 256.);
    d.transform(scale, 0., 0., 0.);
    d.image(&art.house, 0., 0., WIDTH, HEIGHT);
    let (progress, duration) = visit.unwrap_or((0., 0.));
    let pose = visit_pose(progress, duration, s.motion);
    let door_open = if visit.is_some() { pose.doors } else { 0. };
    let id = if visit.is_some() { visitor } else { 0 };
    let proportion = match id {
        13 => 0.86,
        4 | 9 | 19 => 0.91,
        15 => 0.96,
        _ => 1.05,
    };
    let size = proportion * (0.82 + 0.18 * pose.depth);
    let feet = 204. + 18. * pose.depth;
    // Arrive through the left jamb, then return through the right jamb.
    // The case hides the solid toy at either end; its opacity never changes.
    let x = 200. + 130. * pose.lateral;
    let stride = (pose.depth * PI).sin();
    let step = (progress * crate::model::STEP_RADIANS_PER_SECOND).sin();
    if visit.is_some() && pose.visible {
        d.clip_doorway();
        d.alpha_ellipse(x - 25. * size, feet - 3., 50. * size, 6., 0x28081112);
        d.transform(
            scale * size,
            x * scale,
            (feet - step.abs() * stride * 2.) * scale,
            step * stride * 2.,
        );
        character(d, id, pose.frame, art);
        GdipResetClip(d.g);
    }
    d.transform(scale, 0., 0., 0.);
    let frame = (door_open * (art.doors.len() - 1) as f32).round() as usize;
    d.image(&art.doors[frame], 90., 90., 220., 145.);
    let minutes = now.minute() as f32 + now.second() as f32 / 60.;
    let hour = ((now.hour() % 12) as f32 + minutes / 60.) * PI / 6.;
    hand(d, hour, 47., 3.9, 0x344b49);
    hand(d, minutes * PI / 30., 69., 2.5, 0x344b49);
    if s.seconds {
        let a = now.second() as f32 * PI / 30.;
        d.line(
            200. - a.sin() * 12.,
            327. + a.cos() * 12.,
            200. + a.sin() * 78.,
            327. - a.cos() * 78.,
            1.,
            0xae694a,
        );
    }
    d.alpha_ellipse(195., 323., 13., 13., 0x330d1819);
    d.ellipse(194.5, 321.5, 11., 11., 0xb18b53);
    d.ellipse(195.5, 322., 8.5, 8.5, 0xdcc291);
    d.ellipse(198., 324., 3.5, 3.5, 0xf0dfb8);
    if s.digital {
        let t = now
            .format(if s.clock_24h { "%H:%M" } else { "%-I:%M" })
            .to_string();
        d.text(&t, 162., 365., 76., 11., 0x58645c, false);
    }
}
