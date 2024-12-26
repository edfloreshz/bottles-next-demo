use cosmic::widget::icon;

#[derive(Clone, Debug)]
pub struct Bottle {
    pub title: String,
    pub caption: String,
    pub icon: icon::Handle,
}

impl Bottle {
    pub fn new(title: impl Into<String>, caption: impl Into<String>, icon: icon::Handle) -> Self {
        Self {
            title: title.into(),
            caption: caption.into(),
            icon,
        }
    }
}
