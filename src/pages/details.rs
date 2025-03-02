use cosmic::{
    app::Task,
    iced::{Alignment, Length},
    widget, Apply, Element,
};

use crate::{app, components::card::Card};

use super::home::{GridMetrics, Selected};

pub struct Details {
    selected: Option<Selected>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetSelected(Option<Selected>),
}

impl Details {
    pub fn new() -> Self {
        Self { selected: None }
    }

    pub fn view(&self) -> Option<Element<Message>> {
        self.selected.as_ref().map(|_| {
            let spacing = cosmic::theme::active().cosmic().spacing;
            let width = 800.0 as usize;
            let GridMetrics {
                cols,
                item_width,
                column_spacing,
            } = GridMetrics::new(width, 260 + 2 * spacing.space_s as usize, spacing.space_s);

            let mut grid = widget::grid();
            let mut col = 0;
            for _ in 0..10 {
                if col >= cols {
                    grid = grid.insert_row();
                    col = 0;
                }
                grid = grid.push(crate::components::card::card(Card::default(), item_width));
                col += 1;
            }

            widget::container(widget::scrollable(
                grid.column_spacing(column_spacing)
                    .row_spacing(column_spacing),
            ))
            .max_width(1600.)
            .padding(spacing.space_xs)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .apply(widget::container)
            .class(cosmic::style::Container::Card)
            .into()
        })
    }

    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let tasks = vec![];
        match message {
            Message::SetSelected(selected) => {
                self.selected = selected;
            }
        }
        Task::batch(tasks)
    }
}
