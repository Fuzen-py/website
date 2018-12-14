use num::complex::Complex;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const DEFAULT_IMG_X: usize = 700;
const DEFAULT_IMG_Y: usize = 700;
const MAX_ITERATIONS: u16 = 256;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

struct FractalCore;
impl FractalCore {
    pub fn get_cords(width: u64, position: u64) -> (u64, u64) {
        let p = position + 1;
        (
            (p as f32 / width as f32).floor() as u64,
            (p % width) + width,
        )
    }
    //Rust > 1.31 #[allow(clippy::many_single_char_names)]
    pub fn math_closure(width: u64, height: u64) -> impl Fn(usize) -> u8 {
        let mut rng = rand::thread_rng();
        let zoom = rng.gen_range(1f32, 10f32) / rng.gen_range(20f32, 50f32);

        let scalex = 4f32 / width as f32;
        let scaley = 4f32 / height as f32;

        move |pixel| {
            let cords = Self::get_cords(width, pixel as u64);
            let y: u64 = cords.0;
            let x: u64 = cords.1;
            let cy = (y as f32 * scaley - 2.0) * zoom;
            let cx = (x as f32 * scalex - 2.0) * zoom;
            let mut z = Complex::new(cx, cy);
            let c = Complex::new(-0.4, 0.6);
            let mut i = 0;
            for t in 0..MAX_ITERATIONS {
                if (z.norm() > 2.0) || (i > 800) {
                    break;
                }
                z = z * z + c;
                i = t;
            }
            if i < 800 {
                return i as u8;
            };
            0u8
        }
    }
    #[allow(unused)]
    pub fn sync(width: usize, height: usize) -> FractalSync {
        FractalSync::new(width, height)
    }
    #[allow(unused)]
    pub fn parallel(width: usize, height: usize) -> Vec<u8> {
        (0..width * height)
            .into_par_iter()
            .map(FractalCore::math_closure(width as u64, height as u64))
            .collect()
    }
}

pub struct FractalSync {
    width: usize,
    height: usize,
    pos: usize,
    end: usize,
    closure: Box<Fn(usize) -> u8>,
}

impl FractalSync {
    #[allow(unused)]
    fn new(width: usize, height: usize) -> Self {
        FractalSync {
            width,
            height,
            pos: 0,
            end: (width * height),
            closure: Box::new(FractalCore::math_closure(width as u64, height as u64)),
        }
    }
    #[allow(unused)]
    fn dimentions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl std::io::Read for FractalSync {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut i = 0;
        for b in buf {
            match self.next() {
                Some(byte) => {
                    *b = byte;
                    i += 1;
                }
                None => break,
            }
        }
        Ok(i)
    }
}

impl Iterator for FractalSync {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.end {
            return None;
        }
        self.pos += 1;
        Some((self.closure)(self.pos))
    }
}

pub fn fractal_png(_req: &::actix_web::HttpRequest) -> Result<::actix_web::HttpResponse> {
    let imgx: usize = match std::env::var("FRACTAL_RES_X") {
        Ok(val) => match val.parse() {
            Ok(x) => x,
            Err(_) => DEFAULT_IMG_X,
        },
        Err(_) => DEFAULT_IMG_X,
    };
    let imgy: usize = match std::env::var("FRACTAL_RES_Y") {
        Ok(val) => match val.parse() {
            Ok(x) => x,
            Err(_) => DEFAULT_IMG_Y,
        },
        Err(_) => DEFAULT_IMG_Y,
    };
    let mut imgbuf = ::image::ImageBuffer::new(imgx as u32, imgy as u32);
    let mut pre_process: Vec<u8> = FractalCore::parallel(imgx, imgy);
    imgbuf.swap_with_slice(&mut pre_process);
    // Return the image without saving to disk
    let mut png_buf = Vec::with_capacity((imgx * imgy) as usize);
    image::ImageLuma8(imgbuf).write_to(&mut png_buf, image::PNG)?;
    Ok(::actix_web::HttpResponse::Ok()
        .content_type("image/png")
        .body(png_buf))
}

#[test]
fn sync_iter() {
    let v: Vec<u8> = FractalCore::sync(700, 700).collect();
    assert_eq!(v.len(), 490_000usize);
}
