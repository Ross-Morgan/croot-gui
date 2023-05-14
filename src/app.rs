use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use eframe::egui;
use egui_extras::RetainedImage;

const ICON: &[u8; 7660] = include_bytes!("../assets/icon.png");

struct ImageApp(RetainedImage);

impl ImageApp {
    fn new(image_path: PathBuf) -> Self {
        let mut file = File::open(image_path).expect("Failed to open file");
        let mut image_bytes = Vec::new();

        file.read_to_end(&mut image_bytes).expect("Failed to read file");

        Self(RetainedImage::from_image_bytes("Graph Image", image_bytes.as_slice()).expect("Invalid image bytes"))
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
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.0 as f32, dimensions.1 as f32)),
        icon_data: Some(eframe::IconData {
            rgba: ICON.to_vec(),
            width: 256,
            height: 256,
        }),
        ..Default::default()
    };

    eframe::run_native(
        name,
        options,
        Box::new(|_cc| Box::new(ImageApp::new(path))),
    )
}