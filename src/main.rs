use image::{self, DynamicImage, ImageFormat};
use std::path::PathBuf;
use eframe::egui;

pub struct ImageProcessor {
    image: Option<DynamicImage>,
    image_path: Option<PathBuf>,
    blur_sigma: f32,
    resize_width: u32,
    resize_height: u32,
    pending_operation: Option<ImageOperation>,
}

#[derive(Debug)]
enum ImageOperation {
    Resize,
    Blur,
    Grayscale,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self {
            image: None,
            image_path: None,
            blur_sigma: 2.0,
            resize_width: 100,
            resize_height: 100,
            pending_operation: None,
        }
    }
}

impl eframe::App for ImageProcessor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending operations first
        if let Some(op) = self.pending_operation.take() {
            if let Some(img) = self.image.take() {
                self.image = Some(match op {
                    ImageOperation::Resize => img.resize_exact(
                        self.resize_width,
                        self.resize_height,
                        image::imageops::FilterType::Lanczos3,
                    ),
                    ImageOperation::Blur => img.blur(self.blur_sigma),
                    ImageOperation::Grayscale => img.grayscale(),
                });
            }
        }

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

            if let Some(_img) = &self.image {
                ui.label("Image loaded.");
                
                // Resize controls
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(egui::DragValue::new(&mut self.resize_width).speed(1));
                    ui.label("Height:");
                    ui.add(egui::DragValue::new(&mut self.resize_height).speed(1));
                    if ui.button("Resize").clicked() {
                        self.pending_operation = Some(ImageOperation::Resize);
                    }
                });

                // Blur controls
                ui.horizontal(|ui| {
                    ui.label("Blur σ:");
                    ui.add(egui::DragValue::new(&mut self.blur_sigma).speed(0.1));
                    if ui.button("Apply Blur").clicked() {
                        self.pending_operation = Some(ImageOperation::Blur);
                    }
                });

                // Grayscale
                if ui.button("Convert to Grayscale").clicked() {
                    self.pending_operation = Some(ImageOperation::Grayscale);
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

    let _ = eframe::run_native(
        "Rust Image Processor",
        options,
        Box::new(|_cc| Ok(Box::<ImageProcessor>::default())),
    );
}