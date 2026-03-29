// ============ ENUMS ============

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, ToSchema)]
pub enum UserSegmentType {
    #[serde(rename = "binge_watcher")]
    BingeWatcher,
    #[serde(rename = "casual_viewer")]
    CasualViewer,
    #[serde(rename = "explorer")]
    Explorer,
    #[serde(rename = "weekend_warrior")]
    WeekendWarrior,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "general")]
    General,
}

impl UserSegmentType {
    pub fn as_str(&self) -> &str {
        match self {
            UserSegmentType::BingeWatcher => "BINGE_WATCHER",
            UserSegmentType::CasualViewer => "CASUAL_VIEWER",
            UserSegmentType::Explorer => "EXPLORER",
            UserSegmentType::WeekendWarrior => "WEEKEND_WARRIOR",
            UserSegmentType::Inactive => "INACTIVE",
            UserSegmentType::General => "GENERAL",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "BINGE_WATCHER" => UserSegmentType::BingeWatcher,
            "CASUAL_VIEWER" => UserSegmentType::CasualViewer,
            "EXPLORER" => UserSegmentType::Explorer,
            "WEEKEND_WARRIOR" => UserSegmentType::WeekendWarrior,
            "INACTIVE" => UserSegmentType::Inactive,
            _ => UserSegmentType::General,
        }
    }
}
