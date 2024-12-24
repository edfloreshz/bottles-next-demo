use cosmic::{
    iced::{alignment::Vertical, ContentFit, Length},
    widget::image,
    Apply, Element,
};

use crate::icons;

#[derive(Clone, Debug)]
pub struct Card {
    title: String,
    description: String,
    image: image::Handle,
}

pub fn card<'a, Message: 'a + Clone + 'static>(
    card: &'a Card,
    width: usize,
) -> Element<'a, Message> {
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
                        .push(cosmic::widget::text(&card.title).size(18.))
                        .push(cosmic::widget::text::caption(&card.description)),
                )
                .push(cosmic::widget::horizontal_space())
                .push(cosmic::widget::button::icon(icons::get_handle(
                    "play-large-symbolic",
                    18,
                ))),
        )
        .spacing(5)
        .width(width as f32)
        .apply(cosmic::widget::container)
        .class(cosmic::style::Container::ContextDrawer)
        .into()
}

impl Default for Card {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            image: image::Handle::from_bytes(vec![]),
        }
    }
}

impl Card {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        image: image::Handle,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            image,
        }
    }
}
