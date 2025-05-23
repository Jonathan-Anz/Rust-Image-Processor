use eframe::egui;
use egui::ColorImage;
use image::{self, DynamicImage, ImageFormat};
use std::path::PathBuf;

pub struct ImageProcessor {
    image: Option<DynamicImage>,
    image_path: Option<PathBuf>,
    resize_width: u32,
    resize_height: u32,
    hue_rotation: i32,
    blur_sigma: f32,
    brightness:i32,
    contrast: f32,
    pending_operation: Option<ImageOperation>,
    history: Vec<DynamicImage>,
    redo: Vec<DynamicImage>,
    texture: Option<egui::TextureHandle>,
}

#[derive(Debug, PartialEq)]
enum ImageOperation {
    Resize,
    HueRotate,
    Blur,
    FlipHorizontal,
    FlipVertical,
    Grayscale,
    Brightness,
    Contrast,
    Undo,
    Redo,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self {
            image: None,
            image_path: None,
            resize_width: 100,
            resize_height: 100,
            hue_rotation: 90,
            blur_sigma: 2.0,
            brightness: 0,
            contrast: 1.0,
            pending_operation: None,
            history: vec![],
            redo: vec![],
            texture: None,
        }
    }
}

impl ImageProcessor {
    const MAX_HISTORY: usize = 10;
    fn update_texture(&mut self, ctx: &egui::Context) {
        if let Some(img) = &self.image {
            // Convert the DynamicImage to a ColorImage that egui can use
            let size = [img.width() as usize, img.height() as usize];
            let image_buffer = img.to_rgba8();
            let pixels = image_buffer.as_flat_samples();

            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

            // Create or update the texture
            self.texture = Some(ctx.load_texture("image-preview", color_image, Default::default()));
        } else {
            self.texture = None;
        }
    }
}

impl eframe::App for ImageProcessor {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending operations first
        if let Some(op) = self.pending_operation.take() {
            if let Some(img) = self.image.take() {
                // keep history of NEW operations
                if  op != ImageOperation::Undo {
                    // limit the operations kept
                    if self.history.len() >= Self::MAX_HISTORY{
                        self.history.remove(0);
                    }
                    self.history.push(img.clone());
                }

                self.image = Some(match op {
                    ImageOperation::Resize => img.resize_exact(
                        self.resize_width,
                        self.resize_height,
                        image::imageops::FilterType::Lanczos3,
                    ),
                    ImageOperation::HueRotate => img.huerotate(self.hue_rotation),
                    ImageOperation::Blur => img.blur(self.blur_sigma),
                    ImageOperation::FlipHorizontal => img.fliph(),
                    ImageOperation::FlipVertical => img.flipv(),
                    ImageOperation::Grayscale => img.grayscale(),
                    ImageOperation::Brightness => img.brighten(self.brightness),
                    ImageOperation::Contrast => img.adjust_contrast(self.contrast),
                    ImageOperation::Undo => {
                        // try to pop the last operation
                        if let Some(prev) = self.history.pop() {
                            if self.redo.len() >= Self::MAX_HISTORY{
                                self.redo.remove(0);
                            }
                            self.redo.push(img.clone());
                            prev
                        } else {
                            img.clone()
                        }
                    }
                    ImageOperation::Redo => {
                        // try to pop the last undo operation
                        if let Some(prev) = self.redo.pop() {
                            prev
                        } else {
                            img.clone()
                        }
                    }
                });
                self.update_texture(ctx); // Update texture after modification
            }
        }

        egui::TopBottomPanel::top("top_bar").show(ctx,|ui|{
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                // Undo controls 
                if ui.button("Undo").clicked() {
                    self.pending_operation = Some(ImageOperation::Undo);
                }
                // Redo controls
                if ui.button("Redo").clicked() {
                    self.pending_operation = Some(ImageOperation::Redo);
                }

            });
        }); //end top panel


        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Image Processor");

            // Upload
            if ui.button("Upload Image").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    match image::open(&path) {
                        Ok(img) => {
                            self.image = Some(img.clone());
                            self.image_path = Some(path);
                            self.history.push(img.clone());
                            self.update_texture(ctx); // Update texture after loading
                        }
                        Err(e) => {
                            eprintln!("Failed to load image: {e}");
                        }
                    }
                }
            }

            if let Some(_img) = &self.image {
                ui.label("Image loaded.");

                // Show image preview
                if let Some(texture) = &self.texture {
                    // Show the image with a max size to prevent UI overflow
                    let max_width = ui.available_width() - 20.0;
                    let max_height = 300.0;

                    let texture_size = texture.size();

                    let mut desired_size =
                        egui::vec2(texture_size[0] as f32, texture_size[1] as f32);

                    // Maintain aspect ratio while fitting within bounds
                    let ratio = desired_size.x / desired_size.y;
                    if desired_size.x > max_width {
                        desired_size.x = max_width;
                        desired_size.y = max_width / ratio;
                    }
                    if desired_size.y > max_height {
                        desired_size.y = max_height;
                        desired_size.x = max_height * ratio;
                    }

                    ui.image(texture);
                }

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

                // Hue rotation controls
                ui.horizontal(|ui| {
                    ui.label("Hue Rotation Degrees:");
                    ui.add(egui::DragValue::new(&mut self.hue_rotation).speed(1));
                    if ui.button("Rotate").clicked() {
                        self.pending_operation = Some(ImageOperation::HueRotate);
                    }
                });

                // Blur controls
                ui.horizontal(|ui| {
                    ui.label("Blur Sigma:");
                    ui.add(egui::DragValue::new(&mut self.blur_sigma).speed(0.1));
                    if ui.button("Apply Blur").clicked() {
                        self.pending_operation = Some(ImageOperation::Blur);
                    }
                });

                // Flip controls
                ui.horizontal(|ui| {
                    if ui.button("Flip Horizontal").clicked() {
                        self.pending_operation = Some(ImageOperation::FlipHorizontal);
                    }
                    if ui.button("Flip Vertical").clicked() {
                        self.pending_operation = Some(ImageOperation::FlipVertical);
                    }
                });

                // Grayscale controls
                if ui.button("Convert to Grayscale").clicked() {
                    self.pending_operation = Some(ImageOperation::Grayscale);
                }

                // Exposure controls
                ui.horizontal(|ui|{
                    ui.label("Brightness:");
                    ui.add(egui::DragValue::new(&mut self.brightness).speed(1.0).range(-100..=100));
                    if ui.button("Apply Brightness").clicked() {
                        self.pending_operation = Some(ImageOperation::Brightness);
                    }
                });

                // Contrast controls
                ui.horizontal(|ui| {
                    ui.label("Contrast:");
                    ui.add(egui::DragValue::new(&mut self.contrast).speed(0.1).range(-100.0..=100.0));
                    if ui.button("Apply Contrast").clicked() {
                        self.pending_operation = Some(ImageOperation::Contrast);
                    }
                });

                // Save controls
                if ui.button("Save Output").clicked() {
                    if let Some(path) = &self.image_path {
                        let mut save_path = path.clone();
                        save_path.set_file_name("output_preview.png");
                        if let Some(img_to_save) = &self.image {
                            if let Err(e) =
                                img_to_save.save_with_format(&save_path, ImageFormat::Png)
                            {
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
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 700.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Rust Image Processor",
        options,
        Box::new(|_cc| Ok(Box::<ImageProcessor>::default())),
    );
}
