use crate::{
    color::Color,
    fonts::TextStyle,
    math::{Rect, Vec2},
    mesher::Mesh,
};

// ----------------------------------------------------------------------------

/// What the integration gives to the gui.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct RawInput {
    /// Is the button currently down?
    pub mouse_down: bool,

    /// Current position of the mouse in points.
    pub mouse_pos: Option<Vec2>,

    /// Size of the screen in points.
    pub screen_size: Vec2,

    /// Also known as device pixel ratio, > 1 for HDPI screens.
    pub pixels_per_point: f32,
}

/// What the gui maintains
#[derive(Clone, Copy, Debug, Default)]
pub struct GuiInput {
    /// Is the button currently down?
    pub mouse_down: bool,

    /// The mouse went from !down to down
    pub mouse_clicked: bool,

    /// The mouse went from down to !down
    pub mouse_released: bool,

    /// Current position of the mouse in points.
    pub mouse_pos: Option<Vec2>,

    /// Size of the screen in points.
    pub screen_size: Vec2,

    /// Also known as device pixel ratio, > 1 for HDPI screens.
    pub pixels_per_point: f32,
}

impl GuiInput {
    pub fn from_last_and_new(last: &RawInput, new: &RawInput) -> GuiInput {
        GuiInput {
            mouse_down: new.mouse_down,
            mouse_clicked: !last.mouse_down && new.mouse_down,
            mouse_released: last.mouse_down && !new.mouse_down,
            mouse_pos: new.mouse_pos,
            screen_size: new.screen_size,
            pixels_per_point: new.pixels_per_point,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct InteractInfo {
    /// The mouse is hovering above this
    pub hovered: bool,

    /// The mouse went got pressed on this thing this frame
    pub clicked: bool,

    /// The mouse is interacting with this thing (e.g. dragging it)
    pub active: bool,

    /// The region of the screen we are talking about
    pub rect: Rect,
}

// ----------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct Outline {
    pub width: f32,
    pub color: Color,
}

#[derive(Clone, Debug, Serialize)] // TODO: copy
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum PaintCmd {
    Circle {
        center: Vec2,
        fill_color: Option<Color>,
        outline: Option<Outline>,
        radius: f32,
    },
    Line {
        points: Vec<Vec2>,
        color: Color,
        width: f32,
    },
    Rect {
        corner_radius: f32,
        fill_color: Option<Color>,
        outline: Option<Outline>,
        rect: Rect,
    },
    /// Paint a single line of text
    Text {
        color: Color,
        /// Top left corner of the first character.
        pos: Vec2,
        text: String,
        text_style: TextStyle,
        /// Start each character in the text, as offset from pos.
        x_offsets: Vec<f32>,
        // TODO: font info
    },
    /// Low-level triangle mesh
    Mesh(Mesh),
}
