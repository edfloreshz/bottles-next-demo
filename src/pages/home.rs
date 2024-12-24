use cosmic::{
    app::Task,
    iced::Length,
    widget::segmented_button::{Entity, Model, SingleSelect},
    Apply, Element,
};

use crate::{app, icons};

pub struct Home {
    tabs: Model<SingleSelect>,
    query: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    QueryInput(String),
    TabActivated(Entity),
}

pub enum Tab {
    Bottles,
    Library,
}

impl Home {
    pub fn new() -> Self {
        Self {
            tabs: Model::builder()
                .insert(move |b| b.text("Bottles").data(Tab::Bottles).activate())
                .insert(move |b| b.text("Library").data(Tab::Library))
                .build(),
            query: String::new(),
        }
    }

    pub fn next(&self) -> Element<Message> {
        let title = cosmic::widget::text("Bottles Next").size(40).center();

        cosmic::widget::column().spacing(20).push(title).into()
    }

    pub fn classic(&self) -> Element<Message> {
        let active = self.tabs.active_data::<Tab>();
        if let Some(Tab::Bottles) = active {
            let title = cosmic::widget::text("Bottles").size(40).center();
            cosmic::widget::column().spacing(20).push(title).into()
        } else {
            let title = cosmic::widget::text("Library").size(40).center();
            cosmic::widget::column().spacing(20).push(title).into()
        }
    }

    pub fn classic_header_bar(&self) -> Vec<Element<Message>> {
        let tabs = cosmic::widget::tab_bar::horizontal(&self.tabs)
            .width(Length::Shrink)
            .on_activate(Message::TabActivated)
            .padding(4.);
        vec![tabs.into()]
    }

    pub fn next_header_bar(&self) -> Vec<Element<Message>> {
        let placeholder = "Search for softwre and games...";
        let icon = icons::get_icon("loupe-large-symbolic", 18).into();
        let input = cosmic::widget::text_input(placeholder, &self.query)
            .width(Length::Shrink)
            .leading_icon(icon)
            .on_input(Message::QueryInput)
            .padding(8.)
            .editable()
            .size(16.)
            .apply(cosmic::widget::container)
            .max_width(300.);
        vec![input.into()]
    }

    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let tasks = vec![];
        match message {
            Message::QueryInput(query) => {
                self.query = query;
            }
            Message::TabActivated(entity) => self.tabs.activate(entity),
        }
        Task::batch(tasks)
    }
}
