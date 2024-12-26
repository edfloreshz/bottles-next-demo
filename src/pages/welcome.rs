use cosmic::{
    app::Task,
    iced::{
        alignment::{Horizontal, Vertical},
        Length,
    },
    widget, Apply, Element,
};

use crate::{app, config::AppExperience, icons};

pub struct Welcome {
    experiences: [AppExperience; 2],
    selected: AppExperience,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {
    PickExperience(AppExperience),
    ApplyExperience,
}

impl Welcome {
    pub fn new() -> Self {
        Self {
            experiences: [AppExperience::Next, AppExperience::Classic],
            selected: AppExperience::default(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = widget::text("Welcome")
            .size(40)
            .font(cosmic::font::bold())
            .center();

        let description = widget::text("Choose your experience, you can change this later.");

        let header = widget::column()
            .push(title)
            .push(description)
            .align_x(Horizontal::Center)
            .spacing(10.);

        let experience_buttons = self
            .experiences
            .iter()
            .map(|experience| self.experience_button(experience).into());

        let experience_buttons = widget::column()
            .extend(experience_buttons)
            .spacing(10.)
            .width(Length::FillPortion(1));

        let selected_widget = self.selected_widget();

        let selector = widget::row()
            .push(experience_buttons)
            .push(selected_widget)
            .spacing(15.);

        let apply_button = self.apply_button();

        widget::column()
            .push(header)
            .push(selector)
            .push(apply_button)
            .align_x(Horizontal::Center)
            .spacing(60.)
            .max_width(700.)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let mut tasks = vec![];
        match message {
            Message::ApplyExperience => {
                tasks.push(cosmic::task::message(app::Message::ApplyExperience(
                    self.selected,
                )));
            }
            Message::PickExperience(selected) => {
                self.selected = selected;
            }
        }
        Task::batch(tasks)
    }

    fn experience_button(&self, experience: &AppExperience) -> widget::Button<Message> {
        crate::components::button::button(
            experience.title(),
            experience.caption(),
            None,
            Message::PickExperience(*experience),
            Length::Fill,
        )
    }

    fn selected_widget(&self) -> Element<Message> {
        let selected_title = widget::row()
            .push(widget::icon(icons::get_handle("magic-wand-symbolic", 18)))
            .push(
                widget::text(self.selected.title())
                    .size(18.)
                    .font(cosmic::font::bold()),
            )
            .spacing(10.)
            .align_y(Vertical::Center);

        widget::column()
            .push(selected_title)
            .push(widget::text(self.selected.description().0))
            .push(widget::text(self.selected.description().1))
            .width(Length::Fill)
            .padding(16.)
            .spacing(10.)
            .apply(widget::container)
            .class(cosmic::style::Container::Tooltip)
            .width(Length::FillPortion(1))
            .into()
    }

    fn apply_button(&self) -> Element<Message> {
        widget::button::custom(
            widget::row()
                .push(
                    widget::text("Apply Experience")
                        .size(18.)
                        .font(cosmic::font::bold()),
                )
                .push(widget::icon(icons::get_handle("arrow4-right-symbolic", 18)))
                .spacing(10.)
                .align_y(Vertical::Center),
        )
        .padding([15, 25])
        .on_press(Message::ApplyExperience)
        .class(cosmic::style::Button::HeaderBar)
        .apply(widget::container)
        .class(cosmic::style::Container::Tooltip)
        .into()
    }
}
