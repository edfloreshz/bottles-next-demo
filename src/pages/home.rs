use cosmic::{app::Task, Element};

use crate::app;

pub struct Home {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&self) -> Element<Message> {
        let title = cosmic::widget::text("Bottles Next").size(40).center();

        cosmic::widget::column().spacing(20).push(title).into()
    }

    pub fn classic(&self) -> Element<Message> {
        let title = cosmic::widget::text("Bottles Classic").size(40).center();

        cosmic::widget::column().spacing(20).push(title).into()
    }

    pub fn update(&mut self, _message: Message) -> Task<app::Message> {
        let tasks = vec![];
        // match message {}
        Task::batch(tasks)
    }
}
