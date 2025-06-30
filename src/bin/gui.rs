use chess::engine::{
    board::{self, BoardFactory, BoardGame, BoardPosition, make_move, print_board},
    chess_move::{ChessMove, Position, parse_move},
    piece::{Piece, PieceColor, PieceType},
};
use egui::{Color32, Rect, Vec2, pos2};

pub struct ChessUi {
    board: BoardGame,
    selected_position: Option<Position>,
    dragging_piece: Option<(Position, egui::Pos2)>,
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
                    if selected.row == row && selected.column == col {
                        painter.rect_stroke(
                            square_rect,
                            0.0,
                            egui::Stroke::new(1.0, Color32::YELLOW),
                        );
                    }
                }

                if let Some(piece) = self.board[row][col] {
                    let is_dragging = if let Some((drag_pos, _)) = self.dragging_piece {
                        drag_pos.row == row && drag_pos.column == col
                    } else {
                        false
                    };

                    if !is_dragging {
                        let symbol = format!("{}", piece);
                        painter.text(
                            square_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            symbol.trim(),
                            egui::FontId::proportional(square_size * 0.7),
                            if piece.color == PieceColor::White {
                                Color32::WHITE
                            } else {
                                Color32::BLACK
                            },
                        );
                    }
                };
            }
        }

        if let Some(mouse_pos) = ui.ctx().pointer_hover_pos() {
            if mouse_pos.x >= board_rect.min.x
                && mouse_pos.y >= board_rect.min.y
                && mouse_pos.x <= board_rect.max.x
                && mouse_pos.y <= board_rect.max.y
            {
                let board_x = ((mouse_pos.x - board_rect.min.x) / square_size) as usize;
                let board_y = ((mouse_pos.y - board_rect.min.y) / square_size) as usize;

                if board_x < 8 && board_y < 8 {
                    if ui.ctx().input(|i| i.pointer.primary_down()) && self.dragging_piece.is_none()
                    {
                        if let Some(piece) = self.board[board_y][board_x] {
                            if piece.color == self.current_player {
                                let position = Position {
                                    row: board_y,
                                    column: board_x,
                                };
                                self.dragging_piece = Some((position, mouse_pos));
                                self.selected_position = Some(position);
                            }
                        }
                    }

                    if ui.ctx().input(|i| i.pointer.primary_released()) {
                        if let Some((from_pos, _)) = self.dragging_piece {
                            let to_pos = Position {
                                row: board_y,
                                column: board_x,
                            };

                            if from_pos != to_pos {
                                if self.handle_move(from_pos, to_pos) {
                                    self.switch_player();
                                }
                            }

                            self.dragging_piece = None;
                            self.selected_position = None;
                        }
                    }
                }
            }
        }

        if let Some((from_pos, _)) = self.dragging_piece {
            if let Some(mouse_pos) = ui.ctx().pointer_hover_pos() {
                self.dragging_piece = Some((from_pos, mouse_pos));

                if let Some(piece) = self.board[from_pos.row][from_pos.column] {
                    let symbol = format!("{}", piece);
                    painter.text(
                        mouse_pos,
                        egui::Align2::CENTER_CENTER,
                        symbol.trim(),
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

    fn handle_move(&mut self, from: Position, to: Position) -> bool {
        let chess_move = ChessMove { from, to };
        if let Err(e) = make_move(&mut self.board, &chess_move) {
            println!("Move error: {:?}", e);
            return false;
        }
        return true;
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
