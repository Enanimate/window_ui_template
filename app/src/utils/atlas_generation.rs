use std::{fs, io};

use image::{DynamicImage, GenericImage, ImageBuffer};
use rendering::definitions::{UiAtlas, UiAtlasTexture};



pub fn _generate_texture_atlas() -> UiAtlas {
    let mut images: Vec<(DynamicImage, String)> = Vec::new();
    //let a = include_bytes!(".././assets/folder-1484.png");
    let assets_dir = fs::read_dir(r"./app/assets").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    for asset in assets_dir {
        images.push((image::open(asset.as_path()).unwrap(), asset.file_stem().unwrap().to_str().unwrap().to_string()));
    }

    let mut new_width = 0;
    let mut new_height = 0;

    let mut last_image: Option<DynamicImage> = None;
    for image in &images {
        if last_image.is_none() {
            new_height = image.0.height();
        } else {
            new_height = image.0.height().max(last_image.unwrap().height().max(new_height));
        }
        new_width += image.0.width();
        last_image = Some(image.0.clone());
    }

    let mut atlas = ImageBuffer::new(new_width, new_height);
    let mut atlas_data = UiAtlas::new(new_width, new_height);

    let mut last_coordinate = 0;
    for image in images {
        atlas_data.add_entry(UiAtlasTexture::new(image.1, last_coordinate, 0, image.0.width(), image.0.height()));
        atlas.copy_from(&image.0, last_coordinate, 0).unwrap();
        last_coordinate += &image.0.width();
    }

    atlas.save("./app/atlas.png").unwrap();
    atlas_data
}