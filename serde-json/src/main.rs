use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Image {
    path: String,
    width: u32,
    height: u32,
    size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ImageFile {
    original: Image,
    large: Option<Image>,
    small: Option<Image>,
}

fn main() {
    let original_image = Image {
        path: "".to_string(),
        width: 100,
        height: 200,
        size: 300,
    };
    let image_file1 = ImageFile {
        original: original_image,
        large: None,
        small: None,
    };

    let json_str = serde_json::to_string(&image_file1).unwrap();
    std::println!("{}", json_str);
}
