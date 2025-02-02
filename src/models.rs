use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Authorization {
    pub auth_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelInfo {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    #[serde(rename = "firmwareVersion")]
    pub firmware_version: String,
    #[serde(rename = "serialNo")]
    pub serial_number: String,
    pub state: State,
    pub effects: Effects,
    #[serde(rename = "panelLayout")]
    pub panel_layout: PanelLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    #[serde(rename = "colorMode")]
    pub color_mode: String,
    pub brightness: Range,
    pub ct: Range,
    pub hue: Range,
    pub sat: Range,
    pub on: On,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct On {
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub max: u32,
    pub min: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    #[serde(rename = "effectsList")]
    pub effects_list: Vec<String>,
    pub select: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelLayout {
    #[serde(rename = "globalOrientation")]
    pub global_orientation: GlobalOrientation,
    pub layout: Layout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalOrientation {
    pub max: u32,
    pub min: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    #[serde(rename = "numPanels")]
    pub num_panels: u32,
    #[serde(rename = "sideLength")]
    pub side_length: u32,
    #[serde(rename = "positionData")]
    pub position_data: Vec<Position>,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ShapeType {
    Triangle = 0,
    Rhythm = 1,
    Square = 2,
    ControlSquarePrimary = 3,
    ControlSquarePassive = 4,
    PowerSupply = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "panelId")]
    pub panel_id: u32,
    pub o: i32,
    pub x: i32,
    pub y: i32,
    #[serde(rename = "shapeType")]
    pub shape_type: ShapeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Brightness {
    Increment(i32),
    Set { value: u32 },
    SetWithDuration { value: u32, duration: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetRange {
    Increment(i32),
    Set { value: u32 },
}

/*
TODO translate this json
        "rhythm": Object(
            {
                "auxAvailable": Null,
                "firmwareVersion": Null,
                "hardwareVersion": Null,
                "rhythmActive": Null,
                "rhythmConnected": Bool(
                    false
                ),
                "rhythmId": Null,
                "rhythmMode": Null,
                "rhythmPos": Null
            }
        ),
*/
