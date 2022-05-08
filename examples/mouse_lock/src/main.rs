#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{self, Button, CursorLock, Sense, Widget},
    emath::Pos2,
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    value: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Example of mouse capture PoC");
            ui.separator();

            ui.label(format!("value is: {}", self.value));

            let resp = Button::new("DRAG ME").sense(Sense::drag()).ui(ui);

            if ui.ctx().input().cursor_lock.is_locked().is_some() {
                let delta = ui.input().pointer.delta();
                self.value += delta.x as i32;
            }

            if resp.drag_started() {
                let mut pos = ui.ctx().pointer_latest_pos().unwrap_or_default();

                let pd = ui.ctx().pixels_per_point();

                dbg!(pd);

                pos.x = pos.x * pd;
                pos.y = pos.y * pd;

                ui.ctx().input_mut().lock_cursor(true, pos);
            }

            if resp.drag_released() {
                let pos = {
                    let lock = &ui.ctx().input().cursor_lock;
                    let pos = if let CursorLock::Locked {
                        point_to_return: pos,
                        ..
                    } = lock
                    {
                        pos.clone()
                    } else {
                        Pos2::ZERO
                    };
                    pos
                };

                dbg!(pos);

                ui.ctx().input_mut().lock_cursor(false, pos);
            }

            ui.separator();

            ui.label("cursor can be hidden too, and this looks even more awesome");
        });
    }
}
