//! Pointer device kinds and tablet-tool data carried by pointer input events.
//!
//! These types mirror the pointer and tablet model of the `winit` crate.

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

#[cfg(all(feature = "serialize", feature = "bevy_reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// The kind of device that produced a pointer event.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum PointerKind {
    /// A standard mouse.
    Mouse,
    /// A touch contact, such as a finger on a touchscreen.
    Touch,
    /// A graphics-tablet tool, such as a pen or eraser.
    Tablet {
        /// The kind of tool that produced the event.
        tool: TabletToolKind,
        /// Pressure, tilt, and orientation reported by the tool.
        ///
        /// Every field is [`None`] when no contact data is available, such as
        /// while the tool is hovering or entering/leaving the window.
        data: TabletToolData,
    },
}

/// The kind of tool used with a graphics tablet.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum TabletToolKind {
    /// A pen.
    Pen,
    /// The eraser end of a pen.
    Eraser,
    /// A brush.
    Brush,
    /// A pencil.
    Pencil,
    /// An airbrush.
    Airbrush,
    /// A finger used on a tablet that reports tool data.
    Finger,
    /// A mouse-like puck device used on a tablet.
    Mouse,
    /// A lens cursor (magnifier puck).
    Lens,
}

/// Pressure, tilt, and orientation data reported by a tablet tool.
///
/// Each field is [`None`] when the device or platform does not report it.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub struct TabletToolData {
    /// The force the tool is applying against the surface.
    pub force: Option<Force>,
    /// Normalized tangential ("barrel") pressure, in the range -1.0 to 1.0.
    pub tangential_force: Option<f32>,
    /// Clockwise rotation of the tool about its own axis, in degrees (0 to 359).
    pub twist: Option<u16>,
    /// The tool's tilt relative to the surface.
    pub tilt: Option<TabletToolTilt>,
    /// The tool's angular position relative to the surface.
    pub angle: Option<TabletToolAngle>,
}

/// The tilt of a tablet tool, in degrees, along each surface axis.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub struct TabletToolTilt {
    /// Tilt along the surface X axis, in degrees, in the range -90 to 90.
    pub x: i8,
    /// Tilt along the surface Y axis, in degrees, in the range -90 to 90.
    pub y: i8,
}

/// The angular position of a tablet tool relative to the surface.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub struct TabletToolAngle {
    /// Altitude angle in radians: 0 is parallel to the surface, π/2 is
    /// perpendicular to it.
    pub altitude: f64,
    /// Azimuth angle in radians, measured clockwise around the surface.
    pub azimuth: f64,
}

/// A force/pressure measurement reported by a pointer device.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Clone)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum Force {
    /// A calibrated force, where `1.0` is the force of an average touch
    /// regardless of the device.
    Calibrated {
        /// The force of the contact.
        force: f64,
        /// The maximum possible force, providing a dynamic range for `force`.
        max_possible_force: f64,
    },
    /// A normalized force in the range `0.0` to `1.0`, with no known
    /// calibration to a physical unit.
    Normalized(f64),
}
