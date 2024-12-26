use cosmic::widget::image;

#[derive(Clone, Debug)]
pub struct Program {
    pub title: String,
    pub caption: String,
    pub icon: image::Handle,
}

impl Program {
    pub fn new(title: impl Into<String>, caption: impl Into<String>, icon: image::Handle) -> Self {
        Self {
            title: title.into(),
            caption: caption.into(),
            icon,
        }
    }
}
