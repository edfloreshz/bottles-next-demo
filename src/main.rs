// SPDX-License-Identifier: MPL-2.0

mod app;
mod components;
mod config;
mod i18n;
mod icons;
mod pages;

fn main() -> cosmic::iced::Result {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Initialize the icon cache.
    icons::init();

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default().debug(false).size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::AppModel>(settings, ())
}
