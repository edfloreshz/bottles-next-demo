use cosmic::{
    iced::{alignment::Vertical, Length},
    widget::{self, icon},
};

use crate::icons;

pub fn button<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    caption: impl Into<String>,
    icon: Option<icon::Handle>,
    message: Message,
    width: impl Into<Length>,
) -> widget::Button<'a, Message> {
    let info = widget::column()
        .push(
            widget::text(title.into())
                .size(18.)
                .font(cosmic::font::bold()),
        )
        .push(if let Some(icon) = icon {
            widget::row()
                .spacing(10.)
                .push(widget::icon(icon))
                .push(widget::text::caption(caption.into()))
        } else {
            widget::row().push(widget::text::caption(caption.into()))
        })
        .spacing(5);

    let icon = widget::icon(icons::get_handle("arrow4-right-symbolic", 18));

    widget::button::custom(
        widget::row()
            .push(info)
            .push(widget::horizontal_space())
            .push(icon)
            .align_y(Vertical::Center),
    )
    .on_press(message)
    .width(width)
    .padding(15.)
}
