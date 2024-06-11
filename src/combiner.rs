use image::{imageops::FilterType::Triangle, DynamicImage, GenericImageView};

#[derive(Debug)]
pub enum ImageDataErrors {
  DifferentImageFormats,
  BufferTooSmall,
}

pub struct FloatingImage {
  pub width: u32,
  pub height: u32,
  pub data: Vec<u8>,
  pub name: String,
}

impl FloatingImage {
  pub fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }

  pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    // If previosuly assigned buffer is too small to hold new data
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

pub fn get_smallest_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
  let pix1 = dim1.0 * dim1.1;
  let pix2 = dim2.0 * dim2.1;
  return if pix1 < pix2 { dim1 } else { dim2 };
}

pub fn standardize_image(
  image1: DynamicImage,
  image2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image1.dimensions(), image2.dimensions());
  println!("width: {} height: {}\n", width, height);
  if image2.dimensions() == (width, height) {
    (image1.resize_exact(width, height, Triangle), image2)
  } else {
    (image1, image2.resize_exact(width, height, Triangle))
  }
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
  // A Vec<u8> is created same as vec1
  let mut combined_data = vec![0u8; vec1.len()];
  let mut i = 0;
  while i < vec1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
    }
    i += 4;
  }
  combined_data
}

pub fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
  let vec1 = image1.to_rgb8().into_vec();
  let vec2 = image2.to_rgb8().into_vec();
  alternate_pixels(vec1, vec2)
}
