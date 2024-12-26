use cosmic::{
    iced::{alignment::Vertical, ContentFit, Length},
    widget::image,
    Apply,
};

use crate::{icons, pages::home::program::Program};

#[derive(Clone, Debug)]
pub struct Card {
    title: String,
    caption: String,
    image: image::Handle,
}

impl From<&Program> for Card {
    fn from(program: &Program) -> Self {
        Self {
            title: program.title.clone(),
            caption: program.caption.clone(),
            image: program.icon.clone(),
        }
    }
}

pub fn card<'a, Message: 'a + Clone + 'static>(
    card: Card,
    width: usize,
) -> cosmic::widget::Button<'a, Message> {
    cosmic::widget::column()
        .push(
            cosmic::widget::image(&card.image)
                .width(Length::Fill)
                .content_fit(ContentFit::Cover)
                .height(150.),
        )
        .push(
            cosmic::widget::row()
                .align_y(Vertical::Center)
                .padding(20.)
                .push(
                    cosmic::widget::column()
                        .push(cosmic::widget::text(card.title).size(18.))
                        .push(cosmic::widget::text::caption(card.caption)),
                )
                .push(cosmic::widget::horizontal_space())
                .push(
                    cosmic::widget::button::icon(icons::get_handle("play-large-symbolic", 18))
                        .class(cosmic::style::Button::Standard),
                ),
        )
        .spacing(5)
        .apply(cosmic::widget::button::custom)
        .padding(0)
        .width(width as f32)
}

impl Default for Card {
    fn default() -> Self {
        Self {
            title: String::new(),
            caption: String::new(),
            image: image::Handle::from_bytes(vec![]),
        }
    }
}
