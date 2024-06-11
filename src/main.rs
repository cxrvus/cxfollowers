mod app;
use eframe::{egui::{self, Context}, Frame, NativeOptions};
// use std::{fs, path::PathBuf};

fn main() {
	let options = NativeOptions::default();
	eframe::run_simple_native("CX Followers", options, update).unwrap();
}

fn update(ctx: &Context, _frame: &mut Frame) {
	egui::CentralPanel::default().show(ctx, |ui| { app::main(ui)} );
}