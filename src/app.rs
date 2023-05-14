use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use eframe::egui;
use egui_extras::RetainedImage;

struct ImageApp(RetainedImage);

impl ImageApp {
    fn new(image_path: PathBuf) -> Self {
        let mut file = File::open(image_path).expect("Failed to open file");
        let mut image_bytes = Vec::new();

        file.read_to_end(&mut image_bytes)
            .expect("Failed to read file");

        Self(
            RetainedImage::from_image_bytes("Graph Image", image_bytes.as_slice())
                .expect("Invalid image bytes"),
        )
    }
}

impl eframe::App for ImageApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.0.show(ui);
        });
    }
}

pub fn show_image(name: &str, path: PathBuf, dimensions: (u32, u32)) -> Result<(), eframe::Error> {
    let current_path: PathBuf = file!().into();
    let mut icon_path: PathBuf = current_path
        .parent()
        .expect("Cannot fail")
        .to_path_buf()
        .parent()
        .expect("Cannot fail")
        .to_path_buf();

    icon_path.push("assets");
    icon_path.push("icon.png");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.0 as f32, dimensions.1 as f32)),
        icon_data: Some(load_icon(icon_path)),
        ..Default::default()
    };

    eframe::run_native(name, options, Box::new(|_cc| Box::new(ImageApp::new(path))))
}

fn load_icon(path: PathBuf) -> eframe::IconData {
    println!("");

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
