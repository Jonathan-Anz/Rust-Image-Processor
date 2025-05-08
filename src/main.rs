use image::{self, DynamicImage, GenericImageView, ImageFormat};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {

    let mut current_image: Option<DynamicImage> = None;
    //let mut image_path: Option<String> = None;
    let mut image_path = PathBuf::from("None");

    // Main loop
    loop {

        // Get the path as a string
        // let path_string = match &image_path {
        //     Some(path) => path.to_string_lossy().into_owned(),
        //     None => String::from("None"),
        // };

        //let path = Some()

        println!("\nRust Image Processor [Select an option]");
        println!("Current Image: {}", image_path.display());
        println!("[1] Upload an image");
        println!("[2] Resize image");
        println!("[3] Blur image");
        println!("[4] Convert to grayscale");
        println!("[5] Exit");
        println!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {

            // Call functions for each action
            "1" => upload_image(&mut current_image, &mut image_path),
            "2" => break,
            "3" => break,
            "4" => break,
            "5" => {
                println!("\nExiting...\n");
                break;
            }
            _ => println!("\nInvalid option. Please try again."),
        }

        save_image(&mut current_image, &mut image_path);

    }

}

// Uploads an image
fn upload_image(image: &mut Option<DynamicImage>, path: &mut PathBuf) {

    print!("\nEnter an image file path: ");

    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).unwrap();
    let input_path = input_path.trim();
    let new_path = PathBuf::from(input_path);

    match image::open(&new_path) {
        Ok(img) => {
            println!("Image loaded successfully.");
            *image = Some(img);
            *path = new_path;
        }
        Err(e) => println!("Failed to open image: {}", e),
    }
}

// Saves the modified image to the same directory as the uploaded image
fn save_image(image: &mut Option<DynamicImage>, path: &mut PathBuf) {

    if let Some(img) = &image {

        let mut parent_path_string = match path.parent() {
            Some(parent) => parent.to_string_lossy().into_owned(),
            None => {
                println!("Could not find parent path for {}", path.display());
                return;
            },
        };
        //println!("Parent path: {}", parent_path_string);

        parent_path_string.push_str("\\output_preview.png");
        //println!("Output path: {}", parent_path_string);

        let output_path = PathBuf::from(parent_path_string);

        if let Err(e) = img.save_with_format(&output_path, ImageFormat::Png) {
            println!("Failed to save output image: {}", e);
        }
        else {
            println!("Output image saved to {}.", output_path.as_path().display());
        }
    }
}