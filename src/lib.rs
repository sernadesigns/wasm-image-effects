use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
  log(&"Grayscale called".into());

  // Decode the encoded file
  let base64_to_vector = decode(encoded_file).unwrap();
  log(&"Image decoded".into());

  // Load decoded image and store it for manipulation
  let mut img = load_from_memory(&base64_to_vector).unwrap();
  log(&"Image loaded".into());

  // Convert the image to grayscale
  img = img.grayscale();
  log(&"Grayscale effect applied".into());

  // Write new image to PNG format
  let mut buffer = vec![];
  img.write_to(&mut buffer, Png).unwrap();
  log(&"New image written".into());

  let encoded_img = encode(&buffer);
  let data_url = format!("data:image/png;base64,{}", encoded_img);

  data_url
}
