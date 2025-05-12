use image::{self, DynamicImage, ImageFormat};
use std::path::PathBuf;
use eframe::egui;

pub struct ImageProcessor {
    image: Option<DynamicImage>,
    image_path: Option<PathBuf>,
    blur_sigma: f32,
    resize_width: u32,
    resize_height: u32,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self {
            image: None,
            image_path: None,
            blur_sigma: 2.0,
            resize_width: 100,
            resize_height: 100,
        }
    }
}

impl eframe::App for ImageProcessor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Image Processor");

            // Upload
            if ui.button("Upload Image").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    match image::open(&path) {
                        Ok(img) => {
                            self.image = Some(img);
                            self.image_path = Some(path);
                        }
                        Err(e) => {
                            eprintln!("Failed to load image: {e}");
                        }
                    }
                }
            }

            // Store the image in a local variable to avoid borrowing issues
            let current_image = self.image.clone();
            
            if let Some(img) = current_image {
                ui.label("Image loaded.");
                
                // Resize controls
                let mut resize_clicked = false;
                let (mut new_width, mut new_height) = (self.resize_width, self.resize_height);
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(egui::DragValue::new(&mut new_width).speed(1));
                    ui.label("Height:");
                    ui.add(egui::DragValue::new(&mut new_height).speed(1));
                    resize_clicked = ui.button("Resize").clicked();
                });
                
                if resize_clicked {
                    self.resize_width = new_width;
                    self.resize_height = new_height;
                    self.image = Some(img.resize_exact(
                        self.resize_width,
                        self.resize_height,
                        image::imageops::FilterType::Lanczos3,
                    ));
                }

                // Blur controls
                let mut blur_clicked = false;
                let mut new_sigma = self.blur_sigma;
                ui.horizontal(|ui| {
                    ui.label("Blur σ:");
                    ui.add(egui::DragValue::new(&mut new_sigma).speed(0.1));
                    blur_clicked = ui.button("Apply Blur").clicked();
                });
                
                if blur_clicked {
                    self.blur_sigma = new_sigma;
                    self.image = Some(img.blur(self.blur_sigma));
                }

                // Grayscale
                if ui.button("Convert to Grayscale").clicked() {
                    self.image = Some(img.grayscale());
                }

                // Save
                if ui.button("Save Output").clicked() {
                    if let Some(path) = &self.image_path {
                        let mut save_path = path.clone();
                        save_path.set_file_name("output_preview.png");
                        if let Some(img_to_save) = &self.image {
                            if let Err(e) = img_to_save.save_with_format(&save_path, ImageFormat::Png) {
                                eprintln!("Failed to save image: {e}");
                            } else {
                                println!("Image saved to {}", save_path.display());
                            }
                        }
                    }
                }
            } else {
                ui.label("No image loaded.");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Image Processor",
        options,
        Box::new(|_cc| Ok(Box::<ImageProcessor>::default())),
    );
}