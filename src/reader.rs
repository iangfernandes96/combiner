use image::{io::Reader, DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;

pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image: DynamicImage = image_reader.decode().unwrap();
  (image, image_format)
}
