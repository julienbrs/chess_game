use chess::engine::{
    board::{ BoardFactory, BoardGame, BoardPosition, make_move},
    chess_move::{ChessMove, Square},
    piece::{PieceColor},
};
use egui::{Color32, Rect, Vec2};

pub struct ChessUi {
    board: BoardGame,
    selected_position: Option<Square>,
    dragging_piece: Option<(Square, egui::Pos2)>,
    current_player: PieceColor,
}

impl ChessUi {
    pub fn new() -> Self {
        Self {
            board: BoardFactory::create(BoardPosition::Standard),
            selected_position: None,
            dragging_piece: None,
            current_player: PieceColor::White,
        }
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let board_size = ui.available_width().min(ui.available_height());
        let square_size = board_size / 8.0;
        let board_rect = Rect::from_min_size(ui.cursor().min, Vec2::new(board_size, board_size));

        ui.allocate_rect(board_rect, egui::Sense::click_and_drag());

        let painter = ui.painter();

        for row in 0..8 {
            for col in 0..8 {
                let square_min = egui::pos2(
                    board_rect.min.x + col as f32 * square_size,
                    board_rect.min.y + row as f32 * square_size,
                );
                let square_rect =
                    Rect::from_min_size(square_min, Vec2::new(square_size, square_size));
                let is_white = (row + col) % 2 == 0;
                let square_color = if is_white {
                    Color32::from_rgb(238, 238, 210)
                } else {
                    Color32::from_rgb(118, 150, 86)
                };

                painter.rect_filled(square_rect, 0.0, square_color);

                if let Some(selected) = self.selected_position {
                    if selected.row() == row && selected.col() == col {
                        painter.rect_stroke(
                            square_rect,
                            0.0,
                            egui::Stroke::new(1.0, Color32::YELLOW),
                        );
                    }
                }

                if let Some(piece) = self.board[row][col] {
                    let is_dragging = self.dragging_piece.map_or(false, |(drag_sq, _)| {
                        drag_sq.row() == row && drag_sq.col() == col
                    });

                    if !is_dragging {
                        painter.text(
                            square_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", piece).trim(),
                            egui::FontId::proportional(square_size * 0.7),
                            if piece.color == PieceColor::White {
                                Color32::WHITE
                            } else {
                                Color32::BLACK
                            },
                        );
                    }
                }
            }
        }

        if let Some(mouse_pos) = ui.ctx().pointer_hover_pos() {
            if board_rect.contains(mouse_pos) {
                let col = ((mouse_pos.x - board_rect.min.x) / square_size) as u8;
                let row = ((mouse_pos.y - board_rect.min.y) / square_size) as u8;

                if let Ok(pos) = Square::try_from((row, col)) {
                    if ui.ctx().input(|i| i.pointer.primary_down()) && self.dragging_piece.is_none()
                    {
                        if let Some(piece) = self.board[pos.row()][pos.col()] {
                            if piece.color == self.current_player {
                                self.dragging_piece = Some((pos, mouse_pos));
                                self.selected_position = Some(pos);
                            }
                        }
                    }

                    if ui.ctx().input(|i| i.pointer.primary_released()) {
                        if let Some((from, _)) = self.dragging_piece {
                            if from != pos && self.handle_move(from, pos) {
                                self.switch_player();
                            }
                            self.dragging_piece = None;
                            self.selected_position = None;
                        }
                    }
                }
            }
        }

        if let Some((from, _)) = self.dragging_piece {
            if let Some(mouse_pos) = ui.ctx().pointer_hover_pos() {
                self.dragging_piece = Some((from, mouse_pos));
                if let Some(piece) = self.board[from.row()][from.col()] {
                    painter.text(
                        mouse_pos,
                        egui::Align2::CENTER_CENTER,
                        format!("{}", piece).trim(),
                        egui::FontId::proportional(square_size * 0.8),
                        if piece.color == PieceColor::White {
                            Color32::WHITE
                        } else {
                            Color32::BLACK
                        },
                    );
                }
            }
        }

        if self.dragging_piece.is_some() || self.selected_position.is_some() {
            ui.ctx().request_repaint();
        }
    }

    fn handle_move(&mut self, from: Square, to: Square) -> bool {
        let mv = ChessMove { from, to };
        make_move(&mut self.board, &mv)
            .map(|_| true)
            .unwrap_or_else(|e| {
                println!("Move error: {}", e);
                false
            })
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }
}

impl eframe::App for ChessUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ui);
        });
    }
}

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
