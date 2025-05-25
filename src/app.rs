use std::fs;
use eframe::epaint::FontFamily;
use rfd::FileDialog;

#[derive(Default)]
pub struct MyApp {
    files: Vec<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Hello World!")
                        .heading() // 大きい文字（h1相当）
                        .size(40.0), // 明示的にサイズ指定（ポイント単位）
                );

                if ui.button("フォルダを選択").clicked() {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        let entries = fs::read_dir(&folder)
                            .unwrap()
                            .filter_map(|entry| {
                                entry.ok().and_then(|e| {
                                    e.path().file_name()?.to_str().map(|s| s.to_string())
                                })
                            })
                            .collect::<Vec<String>>();

                        self.files = entries;
                    }
                }

                for file in &self.files {
                    ui.label(file);
                }
            });
        });
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "NotoSansCJKjp-Thin".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansCJKjp-Thin.ttf")).into(),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "NotoSansCJKjp-Thin".to_owned()); // 一番優先度高く追加

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("NotoSansCJKjp-Thin".to_owned());

    ctx.set_fonts(fonts);
}
