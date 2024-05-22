use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};
use egui::{text::LayoutJob, Color32, Response, TextFormat, Ui, Widget};

/// `AnsiString` renders a label from a string with ANSI Escape Codes.
///
/// Currently only the color yellow is handled. All other sequences are stripped.
pub struct AnsiString<'a> {
    text: &'a str,
}

impl<'a> AnsiString<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Widget for AnsiString<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mode_yellow: heapless::Vec<u8, 5> = heapless::Vec::from_slice(&[33]).unwrap();
        let mut color = ui.style().visuals.text_color();
        let mut job = LayoutJob::default();
        let parsed: Vec<Output> = self.text.ansi_parse().collect();
        for block in parsed.into_iter() {
            match block {
                Output::TextBlock(text) => job.append(
                    text,
                    0.0,
                    TextFormat {
                        color,
                        ..Default::default()
                    },
                ),
                Output::Escape(seq) => {
                    if let AnsiSequence::SetGraphicsMode(mode) = seq {
                        color = if *mode == *mode_yellow {
                            Color32::YELLOW
                        } else {
                            ui.style().visuals.text_color()
                        }
                    }
                }
            }
        }
        ui.label(job)
    }
}
