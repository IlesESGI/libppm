#![feature(test)]
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use test::Bencher;

extern crate test;

// Structure of Pixel representing a pixel with 3 fields red, green and blue
// coded on 8 bits
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    // Constructor of Pixel
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        return Pixel { r: r, g: g, b: b };
    }

    // Return value of pixel's red channel
    pub fn get_r(&self) -> u8 {
        self.r
    }

    // Return value of pixel's green channel
    pub fn get_g(&self) -> u8 {
        self.g
    }

    // Return value of pixel's blue channel
    pub fn get_b(&self) -> u8 {
        self.b
    }

    // Formatter for Pixel structure
    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }

    // Invert pixel colors
    pub fn invert(&mut self) {
        self.r = self.r ^ 255;
        self.g = self.g ^ 255;
        self.b = self.b ^ 255;
    }

    // Convert pixel to shade of gray
    pub fn to_grayscale(&mut self) {
        let average = self.r / 3 + self.g / 3 + self.b / 3;
        self.r = average;
        self.g = average;
        self.b = average;
    }

    // Return pixel values to String
    pub fn to_string(&self) -> String {
        return format!("{} {} {} ", self.r, self.g, self.b);
    }
}

// Formatter to print Pixel structure
impl fmt::Display for Pixel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Pixel (r: {}, g: {},  b: {})",
            self.r, self.g, self.b
        )
    }
}

// Trait for equality comparisons which are partial equivalence relations
impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

// Image structure composed of a Pixel's vector, the width and the length of the image
pub struct Image {
    pixels: Vec<Pixel>,
    width: u32,
    length: u32,
}

impl Image {
    // Constructor of Image structure
    pub fn new(vector: Vec<Pixel>, width: u32, length: u32) -> Image {
        return Image {
            pixels: vector,
            width: width,
            length: length,
        };
    }

    // Save Image structure in a file as a String
    pub fn save(&self, filename: &Path) {
        let header = format!("P3\n{} {}\n255\n", self.width, self.length);
        let mut ofile = File::create(filename).expect("Unable to create file");
        ofile
            .write_all(header.as_bytes())
            .expect("Unable to write header");

        for pixel in &self.pixels {
            ofile
                .write_all(pixel.to_string().as_bytes())
                .expect("Unable to write pixels");
        }
    }

    // Save Image structure in a file as a String
    pub fn save_binary(&self, filename: &Path) {
        let header = format!("P3\n{} {}\n255\n", self.width, self.length);
        let mut ofile = File::create(filename).expect("Unable to create file");
        ofile
            .write_all(header.as_bytes())
            .expect("Unable to write header");
        let encoded: Vec<u8> = bincode::serialize(&self.pixels).unwrap();
        ofile.write_all(&encoded).expect("Unable to write pixels");
    }

    // Returns the Image's vector of Pixel
    pub fn get_pixels(&self) -> &Vec<Pixel> {
        return &self.pixels;
    }

    // Return an Image structure from a file in plain text (not binary)
    pub fn new_with_file_plain(filename: &Path) -> Result<Image, std::io::Error> {
        return Image::extract_image(filename);
        // /home/Iles/image.ppm
    }

    // Return an Image structure from a binary file (plain text)
    pub fn new_with_file_binary(filename: &Path) -> Result<Image, std::io::Error> {
        return Image::extract_image_binary(filename);
    }

    // Return an Image structure from a file in plain text (not binary)
    pub fn extract_image(filename: &Path) -> Result<Image, std::io::Error> {
        if let Ok(file) = File::open(filename) {
            let mut buffer = BufReader::new(file);
            let (width, height) = Image::read_image_info(&mut buffer)
                .expect("An error occured while reading image info !");
            let mut v: Vec<Pixel> = Vec::new();

            let mut counter = 0;

            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8;
            let mut _pixel: Pixel;

            for line in buffer.lines() {
                for word in line.unwrap().split_whitespace() {
                    if counter % 3 == 0 {
                        r = word.parse::<u8>().unwrap();
                    } else if counter % 3 == 1 {
                        g = word.parse::<u8>().unwrap();
                    } else if counter % 3 == 2 {
                        b = word.parse::<u8>().unwrap();
                        _pixel = Pixel::new(r, g, b);
                        v.push(_pixel);
                    }

                    counter = counter + 1;
                }
            }
            let image = Image::new(v, width as u32, height as u32);

            return Ok(image);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error can't open file ! Check the path",
            ));
        }
    }

    // Return an Image structure from a binary file (plain text)
    pub fn extract_image_binary(filename: &Path) -> Result<Image, std::io::Error> {
        if let Ok(file) = File::open(filename) {
            let mut buffer = BufReader::new(file);
            let (width, height) = Image::read_image_info(&mut buffer)
                .expect("An error occured while reading image info !");
            let mut vector = Vec::<u8>::new();
            buffer.read_to_end(&mut vector)?;
            let decoded = bincode::deserialize(&vector[..]).unwrap();
            let image = Image::new(decoded, width as u32, height as u32);

            return Ok(image);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error can't open file ! Check the path",
            ));
        }
    }

    // Return infos of a PPM image
    fn read_image_info(reader: &mut BufReader<File>) -> Result<(usize, usize), Box<dyn Error>> {
        let mut string_buffer = String::new();
        for _i in 0..3 {
            reader.read_line(&mut string_buffer).unwrap();
        }

        let _ppm_id = string_buffer.lines().nth(0usize).unwrap();

        let image_size = string_buffer
            .lines()
            .nth(1usize)
            .unwrap()
            .to_string()
            .clone();

        let (width, height) = Image::extract_image_size(image_size);

        Ok((width, height))
    }

    // Return width and length of a PPM image
    fn extract_image_size(size: String) -> (usize, usize) {
        let image_size: Vec<String> = size
            .split_whitespace()
            .into_iter()
            .map(|w| w.to_string())
            .collect();
        let width = image_size
            .first()
            .unwrap()
            .parse::<usize>()
            .expect("Image width should be a number");
        let height = image_size
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Image height should be a number");
        (width, height)
    }

    // Invert every Pixel in the image
    pub fn invert(&mut self) {
        self.pixels.par_iter_mut().for_each(|p| p.invert());
    }

    // Transform every Pixel in grayscale
    pub fn to_grayscale(&mut self) {
        self.pixels.par_iter_mut().for_each(|p| p.to_grayscale());
    }

    // Rotate image 180°
    pub fn rotate(&mut self) {
        self.pixels.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate ppm;

    #[test]
    fn test_pixel_values() {
        let mut _pixel = ppm::Pixel::new(255, 0, 0);
        let mut _pixel2 = ppm::Pixel::new(255, 0, 0);
        assert_eq!(_pixel.get_r(), 255);
        assert_eq!(_pixel.get_g(), 0);
        assert_eq!(_pixel.get_b(), 0);
        assert_eq!(_pixel, _pixel2);
    }

    #[test]
    fn test_pixel_invert() {
        let mut _pixel = ppm::Pixel::new(255, 0, 0);
        let mut _pixel2 = ppm::Pixel::new(0, 255, 255);
        _pixel.invert();
        assert_eq!(_pixel, _pixel2);
    }

    #[test]
    fn test_pixel_grayscale() {
        let mut _pixel = ppm::Pixel::new(255, 0, 0);
        let mut _pixel2 = ppm::Pixel::new(85, 85, 85);
        _pixel.to_grayscale();
        assert_eq!(_pixel, _pixel2);
    }
/*
    #[bench]
    fn bench_extract_image(b: &mut Bencher) {
        b.iter(|| ppm::Image::extract_image(Path::new("/home/sha/Downloads/mandelbrot.ppm")));
    }

    #[bench]
    fn bench_extract_image_binary(b: &mut Bencher) {
        b.iter(|| {
            ppm::Image::extract_image_binary(Path::new("/home/sha/Downloads/mandelbrot_bin.ppm"))
        });
    }*/
}
