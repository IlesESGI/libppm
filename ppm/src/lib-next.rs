use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Write;


pub fn test() {
    println!("Test");
}

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        return Pixel { r: r, g: g, b: b };
    }
/*
    pub fn get_r(&self) -> u8 {
        self.r
    }

    fn get_g(&self) -> u8 {
        self.g
    }

    fn get_b(&self) -> u8 {
        self.b
    }
*/
    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn invert(&mut self) {
        self.r = self.r ^ 255;
        self.g = self.g ^ 255;
        self.b = self.b ^ 255;
    }

    pub fn to_grayscale(&mut self) {
        let average = self.r / 3 + self.g / 3 + self.b / 3;
        self.r = average;
        self.g = average;
        self.b = average;
    }

    pub fn to_string(&self) -> String {

        return format!("{} {} {} ", self.r, self.g, self.b);
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Pixel (r: {}, g: {},  b: {})",
            self.r, self.g, self.b
        )
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

pub struct Image {
    pixels: Vec<Pixel>,
    width: u32,
    length: u32, // TODO: Add name of image
}

impl Image {

    pub fn new(vector: Vec<Pixel>, width: u32, length: u32) -> Image {
        return Image { pixels: vector, width: width, length: length };
    }

    pub fn save(&self, filename: &Path) {
        let header = format!("P3\n{} {}\n255\n", self.width, self.length);
        let mut ofile = File::create(filename)
                             .expect("unable to create file");
        ofile.write_all(header.as_bytes()).expect("unable to write");

        for pixel in &self.pixels {
            ofile.write_all(pixel.to_string().as_bytes()).expect("unable to write");
        }

    }

    pub fn get_pixels(&self) -> &Vec<Pixel> {
        return &self.pixels;
    }

    pub fn new_with_file_plain() -> Result<Image, std::io::Error> {

        return Image::extract_image(Path::new("/home/sha/Downloads/mandelbrot.ppm"));
    }

    /*pub fn new_with_file_binary() -> Result<Image, std::io::Error> {

        return Image::extract_image_binary(Path::new("/home/sha/Desktop/Rust/image.ppm"));
    }*/

    pub fn extract_image(filename: &Path) -> Result<Image, std::io::Error> {
        if let Ok(file) = File::open(filename) {
            let mut buffer = BufReader::new(file);
            let (width, height) = Image::read_image_info(&mut buffer).expect("oh no!");
            println!("{}, {}", width, height);
            
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
                    }
                    else if counter % 3 == 1 {
                        g = word.parse::<u8>().unwrap();
                    }
                    
                    else if counter % 3 == 2 {
                        b = word.parse::<u8>().unwrap();
                        _pixel = Pixel::new(r, g, b);
                        v.push(_pixel);
                    }

                    counter = counter + 1;

                }
            }
            
            let image = Image::new(v, width as u32, height as u32);

            return Ok(image);

        }
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error can't open file ! Check the path"));
        }
    }

    /*pub fn extract_image_binary(filename: &Path) -> Result<Image, std::io::Error> {
        if let Ok(file) = File::open(filename) {
            let mut buffer = BufReader::new(file);
            let (width, height) = Image::read_image_info(&mut buffer).expect("oh no!");
            println!("{}, {}", width, height);
            
            let mut vector = Vec::<u8>::new();
            buffer.read_to_end(&mut vector)?;
            
            let image = Image::new(&vector, width as u32, height as u32);

            return Ok(image);

        }
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error can't open file ! Check the path"));
        }
    }*/

    fn read_image_info(reader: &mut BufReader<File>) -> Result<(usize, usize), Box<dyn Error>> {
        let mut string_buffer = String::new();
        for _i in 0..3 {
            reader.read_line(&mut string_buffer).unwrap();
        }

        let ppm_id = string_buffer.lines().nth(0usize).unwrap();

        let image_size = string_buffer
            .lines()
            .nth(1usize)
            .unwrap()
            .to_string()
            .clone();
        let (width, height) = Image::extract_image_size(image_size);

        let color_depth = string_buffer
            .lines()
            .nth(2usize)
            .unwrap()
            .to_string()
            .clone();

        Ok((width, height))
    }

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
            .expect("image width should be a number");
        let height = image_size
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("image height should be a number");
        (width, height)
    }

    pub fn invert(&mut self) {
        for pixel in &mut self.pixels {
            pixel.invert();
        }
    }

    pub fn to_grayscale(&mut self) {
        for pixel in &mut self.pixels {
            pixel.to_grayscale();
        }
    }
}
