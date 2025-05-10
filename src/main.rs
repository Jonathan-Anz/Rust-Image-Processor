use image::{self, DynamicImage, ImageFormat};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {

    let mut current_image: Option<DynamicImage> = None;
    let mut image_path = PathBuf::from("None");

    // Main loop
    loop {

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
            // Save the image after making a change
            "1" => upload_image(&mut current_image, &mut image_path),
            "2" => {
                rescale_image(&mut current_image);
                save_image(&mut current_image, &mut image_path);
            },
            "3" => {
                blur_image(&mut current_image);
                save_image(&mut current_image, &mut image_path);
            },
            "4" => {
                convert_grayscale(&mut current_image);
                save_image(&mut current_image, &mut image_path);
            },
            "5" => {
                println!("\nExiting...\n");
                break;
            }
            _ => println!("\nInvalid option. Please try again."),
        }

    }

}

fn upload_image(image: &mut Option<DynamicImage>, path: &mut PathBuf) {

    print!("\nEnter an image file path: ");

    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).unwrap();
    let input_path = input_path.trim();
    let new_path = PathBuf::from(input_path);

    match image::open(&new_path) {
        Ok(img) => {
            println!("\nImage loaded successfully.");
            *image = Some(img);
            *path = new_path;
        }
        Err(e) => println!("\nFailed to open image: {}", e),
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

fn rescale_image(image: &mut Option<DynamicImage>) {
    
    if let Some(img) = image {
        
        print!("\nEnter new width: ");
        io::stdout().flush().unwrap();
        let mut width = String::new();
        io::stdin().read_line(&mut width).unwrap();
        let width: u32 = width.trim().parse().unwrap_or(img.width());

        print!("Enter new height: ");
        io::stdout().flush().unwrap();
        let mut height = String::new();
        io::stdin().read_line(&mut height).unwrap();
        let height: u32 = height.trim().parse().unwrap_or(img.height());

        *image = Some(img.resize_exact(width, height, image::imageops::FilterType::Lanczos3));
        println!("\nImage resized.");
    } else {
        println!("\nNo image loaded.");
    }
}

fn blur_image(image: &mut Option<DynamicImage>) {

    if let Some(img) = image {
        
        print!("\nEnter blur sigma (Ex: 2.0): ");
        io::stdout().flush().unwrap();
        let mut sigma = String::new();
        io::stdin().read_line(&mut sigma).unwrap();
        let sigma: f32 = sigma.trim().parse().unwrap_or(2.0);
        
        *image = Some(img.blur(sigma));
        println!("\nImage blurred.");
    } else {
        println!("\nNo image loaded.");
    }
}

fn convert_grayscale(image: &mut Option<DynamicImage>) {

    if let Some(img) = image {
        
        *image = Some(img.grayscale());
        println!("\nImage converted to grayscale.");
    } else {
        println!("\nNo image loaded.");
    }
}
