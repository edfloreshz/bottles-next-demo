use cosmic::{
    iced::{alignment::Vertical, Length},
    widget::icon,
};

use crate::icons;

pub fn button<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    caption: impl Into<String>,
    icon: Option<icon::Handle>,
    message: Message,
    width: impl Into<Length>,
) -> cosmic::widget::Button<'a, Message> {
    let info = cosmic::widget::column()
        .push(
            cosmic::widget::text(title.into())
                .size(18.)
                .font(cosmic::font::bold()),
        )
        .push(if let Some(icon) = icon {
            cosmic::widget::row()
                .spacing(10.)
                .push(cosmic::widget::icon(icon))
                .push(cosmic::widget::text::caption(caption.into()))
        } else {
            cosmic::widget::row().push(cosmic::widget::text::caption(caption.into()))
        })
        .spacing(5);

    let icon = cosmic::widget::icon(icons::get_handle("arrow4-right-symbolic", 18));

    cosmic::widget::button::custom(
        cosmic::widget::row()
            .push(info)
            .push(cosmic::widget::horizontal_space())
            .push(icon)
            .align_y(Vertical::Center),
    )
    .on_press(message)
    .width(width)
    .padding(15.)
}
