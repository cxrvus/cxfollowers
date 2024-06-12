use std::path::PathBuf;
use eframe::{egui::{self, Context, Layout}, App, NativeOptions, Result};
use egui_file_dialog::FileDialog;

fn main() -> Result<()> {
	let options = NativeOptions::default();

	eframe::run_native(
		"CX Followers",
		options,
		Box::new(|ctx| Box::new(MyApp::new(ctx))),
	)
}

struct MyApp {
	file_dialog: FileDialog,
	selected_file: Option<PathBuf>,
}

impl MyApp {
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		Self {
			file_dialog: FileDialog::new(),
			selected_file: None,
		}
	}
}

impl App for MyApp {
	fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
		ctx.set_pixels_per_point(1.5);
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
				if ui.button("import ZIP file").clicked() {
					self.file_dialog.select_file();
				}

				ui.label(format!("file: {:?}", self.selected_file));

				if let Some(path) = self.file_dialog.update(ctx).selected() {
					self.selected_file = Some(path.to_path_buf());
				}
			});
		});
	}
}
