use cosmic::{
    app::Task,
    iced::{
        alignment::{Horizontal, Vertical},
        Length,
    },
    Apply, Element,
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
        let title = cosmic::widget::text("Welcome")
            .size(40)
            .font(cosmic::font::bold())
            .center();

        let description =
            cosmic::widget::text("Choose your experience, you can change this later.");

        let header = cosmic::widget::column()
            .push(title)
            .push(description)
            .align_x(Horizontal::Center)
            .spacing(10.);

        let experience_buttons = self.experiences.iter().map(|experience| {
            let info = cosmic::widget::column()
                .push(
                    cosmic::widget::text(experience.title())
                        .size(18.)
                        .font(cosmic::font::bold()),
                )
                .push(cosmic::widget::text::caption(experience.caption()))
                .spacing(5);
            let icon = cosmic::widget::icon(icons::get_handle("arrow4-right-symbolic", 18));

            cosmic::widget::button::custom(
                cosmic::widget::row()
                    .push(info)
                    .push(cosmic::widget::horizontal_space())
                    .push(icon)
                    .align_y(Vertical::Center),
            )
            .on_press(Message::PickExperience(*experience))
            .width(Length::Fill)
            .padding(15.)
            .into()
        });

        let experience_buttons = cosmic::widget::column()
            .extend(experience_buttons)
            .spacing(10.)
            .width(Length::FillPortion(1));

        let selected_title = cosmic::widget::row()
            .push(cosmic::widget::icon(icons::get_handle(
                "magic-wand-symbolic",
                18,
            )))
            .push(
                cosmic::widget::text(self.selected.title())
                    .size(18.)
                    .font(cosmic::font::bold()),
            )
            .spacing(10.)
            .align_y(Vertical::Center);

        let selected = cosmic::widget::column()
            .push(selected_title)
            .push(cosmic::widget::text(self.selected.description().0))
            .push(cosmic::widget::text(self.selected.description().1))
            .width(Length::Fill)
            .padding(16.)
            .spacing(10.)
            .apply(cosmic::widget::container)
            .class(cosmic::style::Container::Tooltip)
            .width(Length::FillPortion(1));

        let selector = cosmic::widget::row()
            .push(experience_buttons)
            .push(selected)
            .spacing(15.);

        let apply_button = cosmic::widget::button::custom(
            cosmic::widget::row()
                .push(
                    cosmic::widget::text("Apply Experience")
                        .size(18.)
                        .font(cosmic::font::bold()),
                )
                .push(cosmic::widget::icon(icons::get_handle(
                    "arrow4-right-symbolic",
                    18,
                )))
                .spacing(10.)
                .align_y(Vertical::Center),
        )
        .padding([15, 25])
        .on_press(Message::ApplyExperience)
        .class(cosmic::style::Button::HeaderBar)
        .apply(cosmic::widget::container)
        .class(cosmic::style::Container::Tooltip);

        cosmic::widget::column()
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
}
