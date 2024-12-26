// SPDX-License-Identifier: MPL-2.0

use crate::config::{AppExperience, Config};
use crate::pages::home::Selected;
use crate::{fl, pages};
use cosmic::app::{self, Core, Task};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::cosmic_theme::ThemeBuilder;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Length, Subscription};
use cosmic::widget::{self, nav_bar};
use cosmic::{Application, ApplicationExt, Apply, Element};
use std::sync::Arc;

const SELECTED_WIDTH: f32 = 350.;

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    // Configuration data that persists between application runs.
    config: Config,
    welcome: pages::welcome::Welcome,
    home: pages::home::Home,
    details: pages::details::Details,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    UpdateConfig(Config),
    ApplyExperience(AppExperience),
    Welcome(pages::welcome::Message),
    Home(pages::home::Message),
    Details(pages::details::Message),
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "dev.edfloreshz.Bottles.Next";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        // Create a nav bar with three page items.
        let nav = nav_bar::Model::default();

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            nav,
            // Optional configuration file for an application.
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            welcome: pages::welcome::Welcome::new(),
            home: pages::home::Home::new(),
            details: pages::details::Details::new(),
        };

        app.core.nav_bar_set_toggled(false);

        // Create a startup command that sets the window title and the theme.
        let mut tasks = vec![app.update_title()];

        let theme_str = include_str!("../resources/themes/Bottles.ron");
        if let Ok(builder) = ron::from_str::<ThemeBuilder>(theme_str) {
            let theme = cosmic::theme::Theme::custom(Arc::new(builder.build()));
            tasks.push(app::command::set_theme(theme));
        }

        (app, Task::batch(tasks))
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        match (self.config.app_experience, &self.home.selected) {
            (Some(AppExperience::Next), Some(Selected::Program(_))) => {
                vec![widget::row()
                    .push(self.home.program_back_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(self.home.search_bar().map(Message::Home))
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .width(SELECTED_WIDTH)
                    .into()]
            }
            (Some(AppExperience::Next), None) => {
                vec![self.home.options_button().map(Message::Home)]
            }
            (Some(AppExperience::Classic), Some(Selected::Bottle(_))) => {
                vec![widget::row()
                    .push(self.home.new_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(self.home.classic_tabs().map(Message::Home))
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .width(SELECTED_WIDTH)
                    .into()]
            }
            (Some(AppExperience::Classic), Some(Selected::Program(_))) => {
                vec![widget::row()
                    .push(self.home.new_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(self.home.classic_tabs().map(Message::Home))
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .width(SELECTED_WIDTH)
                    .into()]
            }
            (Some(AppExperience::Classic), None) => {
                vec![self.home.new_button().map(Message::Home)]
            }
            (_, _) => vec![],
        }
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        match (self.config.app_experience, &self.home.selected) {
            (Some(AppExperience::Next), None) => {
                vec![self.home.search_bar().map(Message::Home)]
            }
            (Some(AppExperience::Classic), None) => {
                vec![self.home.classic_tabs().map(Message::Home)]
            }
            (_, _) => vec![],
        }
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        match (self.config.app_experience, &self.home.selected) {
            (Some(AppExperience::Next), Some(Selected::Program(_))) => {
                vec![widget::row()
                    .push(self.home.program_options_button().map(Message::Home))
                    .push(self.home.program_power_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(
                        self.home
                            .program_tabs()
                            .map(|tabs| tabs.map(Message::Home))
                            .unwrap_or(cosmic::widget::text("").into()),
                    )
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .apply(widget::container)
                    .width(Length::Fill)
                    .into()]
            }
            (Some(AppExperience::Classic), Some(Selected::Bottle(_))) => {
                vec![widget::row()
                    .push(self.home.bottle_options_button().map(Message::Home))
                    .push(self.home.bottle_power_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(
                        self.home
                            .bottle_tabs()
                            .map(|tabs| tabs.map(Message::Home))
                            .unwrap_or(cosmic::widget::text("").into()),
                    )
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .apply(widget::container)
                    .width(Length::Fill)
                    .into()]
            }
            (Some(AppExperience::Classic), Some(Selected::Program(_))) => {
                vec![widget::row()
                    .push(self.home.program_options_button().map(Message::Home))
                    .push(self.home.program_power_button().map(Message::Home))
                    .push(widget::horizontal_space())
                    .push(
                        self.home
                            .program_tabs()
                            .map(|tabs| tabs.map(Message::Home))
                            .unwrap_or(cosmic::widget::text("").into()),
                    )
                    .push(widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .spacing(10.)
                    .apply(widget::container)
                    .width(Length::Fill)
                    .into()]
            }
            (_, _) => vec![],
        }
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        None
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        let details = self
            .details
            .view()
            .map(|details| details.map(Message::Details));

        match self.config.app_experience {
            Some(app_experience) => {
                let content = match app_experience {
                    AppExperience::Next => self.home.next().map(Message::Home),
                    AppExperience::Classic => self.home.classic().map(Message::Home),
                };

                widget::row()
                    .push(
                        widget::container(content)
                            .class(cosmic::style::Container::Card)
                            .width(if self.home.selected.is_none() {
                                Length::Fill
                            } else {
                                Length::Fixed(SELECTED_WIDTH)
                            }),
                    )
                    .push_maybe(details)
                    .spacing(10.)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Vertical::Center)
                    .into()
            }
            None => widget::container(self.welcome.view().map(Message::Welcome))
                .class(cosmic::style::Container::Background)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        let mut tasks = vec![];

        match message {
            Message::Details(message) => tasks.push(self.details.update(message)),
            Message::Home(message) => tasks.push(self.home.update(message)),
            Message::Welcome(message) => tasks.push(self.welcome.update(message)),
            Message::ApplyExperience(choice) => {
                self.config.app_experience = Some(choice);
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }
        }
        Task::batch(tasks)
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        self.update_title()
    }
}

impl AppModel {
    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<Message> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}
