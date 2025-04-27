mod board;
mod chess_move;
mod gui;
mod piece;

use gui::ChessUi;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Game",
        options,
        Box::new(|_cc| Box::new(ChessUi::new())),
    )
    .unwrap();
}
