// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::icon;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(crate) static ICON_CACHE: OnceLock<Mutex<IconCache>> = OnceLock::new();

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IconCacheKey {
    name: &'static str,
    size: u16,
}

pub struct IconCache {
    cache: HashMap<IconCacheKey, icon::Handle>,
}

pub fn init() {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
}

impl IconCache {
    pub fn new() -> Self {
        let mut cache = HashMap::new();

        macro_rules! bundle {
            ($name:expr, $size:expr) => {
                let data: &'static [u8] =
                    include_bytes!(concat!("../resources/icons/bundled/", $name, ".svg"));
                cache.insert(
                    IconCacheKey {
                        name: $name,
                        size: $size,
                    },
                    icon::from_svg_bytes(data).symbolic(true),
                );
            };
        }

        bundle!("loupe-large-symbolic", 18);
        bundle!("arrow4-right-symbolic", 18);
        bundle!("magic-wand-symbolic", 18);
        bundle!("plus-large-symbolic", 18);
        bundle!("view-more-symbolic", 18);

        Self { cache }
    }

    fn get_icon(&mut self, name: &'static str, size: u16) -> icon::Icon {
        let handle = self
            .cache
            .entry(IconCacheKey { name, size })
            .or_insert_with(|| icon::from_name(name).size(size).handle())
            .clone();
        icon::icon(handle).size(size)
    }
}

pub fn get_icon(name: &'static str, size: u16) -> icon::Icon {
    let mut icon_cache = ICON_CACHE.get().unwrap().lock().unwrap();
    icon_cache.get_icon(name, size)
}

pub fn get_handle(name: &'static str, size: u16) -> icon::Handle {
    let mut icon_cache = ICON_CACHE.get().unwrap().lock().unwrap();
    icon_cache
        .cache
        .entry(IconCacheKey { name, size })
        .or_insert_with(|| icon::from_name(name).size(size).handle())
        .clone()
}
