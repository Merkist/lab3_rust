mod task;
mod storage;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let app = ui::App::new();
    let options = eframe::NativeOptions::default();
    eframe::run_native("Додаток для списка справ", options, Box::new(|_cc| Ok(Box::new(app))))?;
    Ok(())
}
