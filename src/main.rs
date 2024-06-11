mod args;

use args::Args;

mod reader;
use reader::find_image_from_path;

mod combiner;
use combiner::{combine_images, standardize_image, FloatingImage, ImageDataErrors};

use image::{save_buffer_with_format, ColorType::Rgba8};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image1, format1) = find_image_from_path(args.image1);
  let (image2, format2) = find_image_from_path(args.image2);

  if format1 != format2 {
    return Err(ImageDataErrors::DifferentImageFormats);
  }
  let (image1, image2) = standardize_image(image1, image2);
  let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);
  let combined_data = combine_images(image1, image2);
  output.set_data(combined_data)?;
  let expected_size = output.width * output.height * 4;
  if output.data.len() < expected_size as usize {
    let padding = vec![0u8; expected_size as usize - output.data.len()];
    output.data.extend(padding);
  }
  save_buffer_with_format(
    output.name,
    &output.data,
    output.width,
    output.height,
    Rgba8,
    format1,
  )
  .unwrap();
  Ok(())
}
