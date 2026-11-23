//! Functionality for displaying the NES screen

use std::time::{Duration, Instant};

use eframe::{CreationContext, Frame, egui::{self, CentralPanel, Color32, ColorImage, Context, Image, TextureHandle, TextureOptions, Ui, ViewportBuilder}};

use crate::NES;

const FRAME_RATE: f64 = 60.0;

pub fn run_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([512.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Constant Speed Pixel Buffer",
        options,
        Box::new(|cc| Ok(Box::new(NESApp::new(cc)))),
    )
}

struct NESApp {
    image: ColorImage,
    texture: TextureHandle,
    frame: usize,
    last_tick: Instant,
    tick_interval: Duration
}

impl NESApp {
    pub fn new(cc: &CreationContext) -> Self {
        let image = ColorImage::filled([256,240], Color32::RED);
        let texture = cc.egui_ctx.load_texture(
            "framebuffer",
            image.clone(),
            TextureOptions::NEAREST
        );

        Self {
            image: image,
            texture: texture,
            frame: 0,
            last_tick: Instant::now(),
            tick_interval: Duration::from_secs_f64(1.0 / FRAME_RATE),
        }
    }
}

/// TODO: Change this logic to support actual PPU drawing
impl eframe::App for NESApp {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {
        let now = Instant::now();

        // Only advance the pixel buffer when the exact interval has passed
        if now.duration_since(self.last_tick) >= self.tick_interval {
            self.last_tick = now;
            self.texture.set(self.image.clone(), TextureOptions::NEAREST);
        }
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        let panel_frame = egui::Frame::central_panel(ui.style())
            .inner_margin(0.0)
            .fill(Color32::BLACK);

        CentralPanel::default()
            .frame(panel_frame)
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.add(
                        Image::new(&self.texture)
                            .fit_to_exact_size(ui.available_size()),
                    );
                });
            });

        // Wake up right when the next pixel tick is due
        let remaining = self.tick_interval.saturating_sub(self.last_tick.elapsed());
        ui.request_repaint_after(remaining);
    }
}