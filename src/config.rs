// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    pub app_experience: Option<AppExperience>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppExperience {
    #[default]
    Next,
    Classic,
}

impl AppExperience {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Next => "Next Mode",
            Self::Classic => "Classic Mode",
        }
    }

    pub fn caption(&self) -> &'static str {
        match self {
            Self::Next => "The easiest way to use Bottles.",
            Self::Classic => "The experience for advanced users.",
        }
    }

    pub fn description(&self) -> (&'static str, &'static str) {
        match self {
            Self::Next => (
                "The software and games you install will be managed by Bottles using a simple environment.", "This is the most convenient way if you're a beginner."
            ),
            Self::Classic => ("The software and games you install will be managed by Bottles in multiple environments.", "This gives advanced users the ability to fine-tune their experience."),
        }
    }
}
