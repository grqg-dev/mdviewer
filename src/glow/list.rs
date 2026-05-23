use eframe::egui::{RichText, Ui};
use egui_commonmark_backend::elements::{newline, number_point};
use egui_commonmark_backend::misc::CommonMarkOptions;

use crate::theme::glow_latte as palette;

struct ListLevel {
    current_number: Option<u64>,
}

#[derive(Default)]
pub struct List {
    items: Vec<ListLevel>,
    has_list_begun: bool,
}

impl List {
    pub fn start_level_with_number(&mut self, start_number: u64) {
        self.items.push(ListLevel {
            current_number: Some(start_number),
        });
    }

    pub fn start_level_without_number(&mut self) {
        self.items.push(ListLevel {
            current_number: None,
        });
    }

    pub fn is_inside_a_list(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn is_last_level(&self) -> bool {
        self.items.len() == 1
    }

    pub fn start_item(&mut self, ui: &mut Ui, options: &CommonMarkOptions) {
        if self.has_list_begun {
            newline(ui);
        } else {
            self.has_list_begun = true;
        }

        let len = self.items.len();
        if let Some(item) = self.items.last_mut() {
            ui.label(" ".repeat((len - 1) * options.indentation_spaces));

            if let Some(number) = &mut item.current_number {
                number_point(ui, &number.to_string());
                *number += 1;
            } else {
                let bullet = if len > 1 { "◦ " } else { "• " };
                ui.label(RichText::new(bullet).color(palette::TEXT));
            }
        } else {
            unreachable!();
        }

        ui.add_space(4.0);
    }

    pub fn end_level(&mut self, ui: &mut Ui, insert_newline: bool) {
        self.items.pop();

        if self.items.is_empty() && insert_newline {
            newline(ui);
        }
    }
}
