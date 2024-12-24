use cosmic::{
    app::Task,
    iced::{Alignment, Length},
    widget::segmented_button::{Entity, Model, SingleSelect},
    Apply, Element,
};

use crate::{app, components::card::Card, icons};

pub struct Home {
    tabs: Model<SingleSelect>,
    query: String,
    library: Vec<Card>,
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
            library: vec![
                Card::new(
                    "Assassin's Creed Valhalla",
                    "Running...",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/assassins-creed-valhalla.jpg")
                            .to_vec(),
                    ),
                ),
                Card::new(
                    "Battle.net",
                    "1h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/blizzard.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Cyberpunk 2077",
                    "4h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/cyberpunk.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Steam",
                    "6h ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/steam.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Need for Speed Unbound",
                    "12 days ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/nfs-unbound.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Overwatch 2",
                    "Last week",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/overwatch-2.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Need for Speed Heat",
                    "Last week",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/nfs-heat.jpg").to_vec(),
                    ),
                ),
                Card::new(
                    "Apex Legends",
                    "2 weeks ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/apex-legends.webp").to_vec(),
                    ),
                ),
                Card::new(
                    "Warframe",
                    "2 weeks ago",
                    cosmic::widget::image::Handle::from_bytes(
                        include_bytes!("../../resources/images/warframe.jpg").to_vec(),
                    ),
                ),
            ],
        }
    }

    pub fn next(&self) -> Element<Message> {
        self.grid()
    }

    pub fn classic(&self) -> Element<Message> {
        let active = self.tabs.active_data::<Tab>();
        if let Some(Tab::Bottles) = active {
            let title = cosmic::widget::text("Bottles").size(40).center();
            cosmic::widget::column().spacing(20).push(title).into()
        } else {
            self.grid()
        }
    }

    fn grid(&self) -> Element<Message> {
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
            for card in self.library.iter() {
                if col >= cols {
                    grid = grid.insert_row();
                    col = 0;
                }
                grid = grid.push(crate::components::card::card(card, item_width));
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
