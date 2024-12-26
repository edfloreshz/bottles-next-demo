use bottle::Bottle;
use cosmic::{
    app::Task,
    iced::{alignment::Vertical, Alignment, Length},
    widget::segmented_button::{Entity, Model, SingleSelect},
    Apply, Element,
};
use program::Program;

use crate::{app, icons};

pub mod bottle;
pub mod program;

pub struct Home {
    classic_tabs_model: Model<SingleSelect>,
    program_tabs_model: Option<Model<SingleSelect>>,
    bottle_tabs_model: Option<Model<SingleSelect>>,
    query: String,
    pub selected: Option<Selected>,
    library: Vec<Program>,
    bottles: Vec<Bottle>,
}

#[derive(Clone, Debug)]
pub enum Message {
    QueryInput(String),
    ProgramTabActivated(Entity),
    BottleTabActivated(Entity),
    ClassicTabActivated(Entity),
    Select(Option<Selected>),
}

#[derive(Clone, Debug)]
pub enum Selected {
    Bottle(Bottle),
    Program(Program),
}

pub enum ClassicTab {
    Bottles,
    Library,
}

pub enum BottleTab {
    Programs,
    Settings,
    Snapshots,
}

pub enum ProgramTab {
    Program,
    Advanced,
}

impl Home {
    pub fn new() -> Self {
        Self {
            classic_tabs_model: Model::builder()
                .insert(move |b| b.text("Bottles").data(ClassicTab::Bottles).activate())
                .insert(move |b| b.text("Library").data(ClassicTab::Library))
                .build(),
            program_tabs_model: None,
            bottle_tabs_model: None,
            query: String::new(),
            selected: None,
            library: vec![
                Program::new(
                    "Assassin's Creed Valhalla",
                    "Running...",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/assassins-creed-valhalla.jpg")
                            .to_vec(),
                    ),
                ),
                Program::new(
                    "Battle.net",
                    "1h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/blizzard.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Cyberpunk 2077",
                    "4h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/cyberpunk.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Steam",
                    "6h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/steam.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Need for Speed Unbound",
                    "12 days ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/nfs-unbound.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Overwatch 2",
                    "Last week",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/overwatch-2.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Need for Speed Heat",
                    "Last week",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/nfs-heat.jpg").to_vec(),
                    ),
                ),
                Program::new(
                    "Apex Legends",
                    "2 weeks ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/apex-legends.webp").to_vec(),
                    ),
                ),
                Program::new(
                    "Warframe",
                    "2 weeks ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/warframe.jpg").to_vec(),
                    ),
                ),
            ],
            bottles: vec![
                Bottle::new(
                    "Gaming paradise",
                    "Gaming",
                    icons::get_handle("xbox-controller-symbolic", 18),
                ),
                Bottle::new(
                    "Windows development",
                    "Software",
                    icons::get_handle("build-alt-symbolic", 18),
                ),
                Bottle::new(
                    "Game engines",
                    "Gaming",
                    icons::get_handle("xbox-controller-symbolic", 18),
                ),
                Bottle::new(
                    "Weird experiments",
                    "Custom",
                    icons::get_handle("issue-symbolic", 18),
                ),
            ],
        }
    }

    pub fn update(&mut self, message: Message) -> Task<app::Message> {
        let mut tasks = vec![];
        match message {
            Message::QueryInput(query) => {
                self.query = query;
            }
            Message::ClassicTabActivated(entity) => {
                self.bottle_tabs_model = None;
                self.program_tabs_model = None;
                self.selected = None;
                tasks.push(cosmic::task::message(app::Message::Details(
                    crate::pages::details::Message::SetSelected(None),
                )));
                self.classic_tabs_model.activate(entity)
            }
            Message::ProgramTabActivated(entity) => {
                if let Some(ref mut program_tabs_model) = self.program_tabs_model {
                    program_tabs_model.activate(entity)
                }
            }
            Message::BottleTabActivated(entity) => {
                if let Some(ref mut bottle_tabs_model) = self.bottle_tabs_model {
                    bottle_tabs_model.activate(entity)
                }
            }
            Message::Select(selected) => {
                self.selected = selected.clone();
                tasks.push(cosmic::task::message(app::Message::Details(
                    crate::pages::details::Message::SetSelected(selected),
                )));
                match &self.selected {
                    Some(Selected::Bottle(_bottle)) => {
                        self.bottle_tabs_model = Some(
                            Model::builder()
                                .insert(move |b| {
                                    b.text("Programs").data(BottleTab::Programs).activate()
                                })
                                .insert(move |b| b.text("Settings").data(BottleTab::Settings))
                                .insert(move |b| b.text("Snapshots").data(BottleTab::Snapshots))
                                .build(),
                        )
                    }
                    Some(Selected::Program(program)) => {
                        self.program_tabs_model = Some(
                            Model::builder()
                                .insert(move |b| {
                                    b.text(program.title.clone())
                                        .data(ProgramTab::Program)
                                        .activate()
                                })
                                .insert(move |b| b.text("Advanced").data(ProgramTab::Advanced))
                                .build(),
                        )
                    }
                    None => {
                        self.bottle_tabs_model = None;
                        self.program_tabs_model = None;
                    }
                }
            }
        }
        Task::batch(tasks)
    }

    pub fn next(&self) -> Element<Message> {
        self.library_grid()
    }

    pub fn classic(&self) -> Element<Message> {
        let active = self.classic_tabs_model.active_data::<ClassicTab>();
        if let Some(ClassicTab::Bottles) = active {
            self.bottles_grid()
        } else {
            self.library_grid()
        }
    }

    fn bottles_grid(&self) -> Element<Message> {
        cosmic::widget::responsive(move |size| {
            let spacing = cosmic::theme::active().cosmic().spacing;
            let width = (size.width - 2.0 * spacing.space_s as f32).floor().max(0.0) as usize;
            let GridMetrics {
                cols,
                item_width,
                column_spacing,
            } = GridMetrics::new(width, 260 + 2 * spacing.space_s as usize, spacing.space_s);

            let mut grid = cosmic::widget::grid();
            let mut col = 0;
            for bottle in self.bottles.iter() {
                if col >= cols {
                    grid = grid.insert_row();
                    col = 0;
                }
                grid = grid.push(crate::components::button::button(
                    &bottle.title,
                    &bottle.caption,
                    Some(bottle.icon.clone()),
                    Message::Select(Some(Selected::Bottle(bottle.clone()))),
                    item_width as f32,
                ));
                col += 1;
            }

            cosmic::widget::container(cosmic::widget::scrollable(
                grid.column_spacing(column_spacing)
                    .row_spacing(column_spacing),
            ))
            .max_width(1600.)
            .padding(spacing.space_xs)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        })
        .into()
    }

    fn library_grid(&self) -> Element<Message> {
        cosmic::widget::responsive(move |size| {
            let spacing = cosmic::theme::active().cosmic().spacing;
            let width = (size.width - 2.0 * spacing.space_s as f32).floor().max(0.0) as usize;
            let GridMetrics {
                cols,
                item_width,
                column_spacing,
            } = GridMetrics::new(width, 260 + 2 * spacing.space_s as usize, spacing.space_s);

            let mut grid = cosmic::widget::grid();
            let mut col = 0;
            for program in self.library.iter() {
                if col >= cols {
                    grid = grid.insert_row();
                    col = 0;
                }
                grid = grid.push(
                    crate::components::card::card(program.into(), item_width)
                        .on_press(Message::Select(Some(Selected::Program(program.clone())))),
                );
                col += 1;
            }

            cosmic::widget::container(cosmic::widget::scrollable(
                grid.column_spacing(column_spacing)
                    .row_spacing(column_spacing),
            ))
            .max_width(1600.)
            .padding(spacing.space_xs)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        })
        .into()
    }

    pub fn classic_tabs(&self) -> Element<Message> {
        cosmic::widget::tab_bar::horizontal(&self.classic_tabs_model)
            .width(Length::Shrink)
            .on_activate(Message::ClassicTabActivated)
            .padding(4.)
            .into()
    }

    pub fn search_bar(&self) -> Element<Message> {
        let icon = icons::get_icon("loupe-large-symbolic", 18).into();
        if self.selected.is_none() {
            cosmic::widget::text_input("Search for software and games...", &self.query)
                .width(Length::Shrink)
                .leading_icon(icon)
                .on_input(Message::QueryInput)
                .editable()
                .padding(8.)
                .size(16.)
                .apply(cosmic::widget::container)
                .max_width(300.)
                .into()
        } else {
            cosmic::widget::button::custom(
                cosmic::widget::row()
                    .push(cosmic::widget::horizontal_space())
                    .push(icon)
                    .push(cosmic::widget::horizontal_space())
                    .align_y(Vertical::Center)
                    .padding(5.)
                    .width(100.),
            )
            .into()
        }
    }

    pub fn program_back_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("left-symbolic", 18))
            .on_press(Message::Select(None))
            .into()
    }

    pub fn options_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("view-more-symbolic", 18)).into()
    }

    pub fn new_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("plus-large-symbolic", 18)).into()
    }

    pub fn program_options_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("view-more-symbolic", 18)).into()
    }

    pub fn bottle_options_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("view-more-symbolic", 18)).into()
    }

    pub fn program_power_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("power-symbolic", 18)).into()
    }

    pub fn bottle_power_button(&self) -> Element<Message> {
        cosmic::widget::button::icon(icons::get_handle("power-symbolic", 18)).into()
    }

    pub fn program_tabs(&self) -> Option<Element<Message>> {
        self.program_tabs_model.as_ref().map(|model| {
            cosmic::widget::tab_bar::horizontal(&model)
                .width(Length::Shrink)
                .on_activate(Message::ProgramTabActivated)
                .padding(4.)
                .into()
        })
    }

    pub fn bottle_tabs(&self) -> Option<Element<Message>> {
        self.bottle_tabs_model.as_ref().map(|model| {
            cosmic::widget::tab_bar::horizontal(&model)
                .width(Length::Shrink)
                .on_activate(Message::BottleTabActivated)
                .padding(4.)
                .into()
        })
    }
}

pub struct GridMetrics {
    pub cols: usize,
    pub item_width: usize,
    pub column_spacing: u16,
}

impl GridMetrics {
    pub fn new(width: usize, min_width: usize, column_spacing: u16) -> Self {
        let width_m1 = width.checked_sub(min_width).unwrap_or(0);
        let cols_m1 = width_m1 / (min_width + column_spacing as usize);
        let cols = cols_m1 + 1;
        let item_width = width
            .checked_sub(cols_m1 * column_spacing as usize)
            .unwrap_or(0)
            .checked_div(cols)
            .unwrap_or(0);
        Self {
            cols,
            item_width,
            column_spacing,
        }
    }
}
