use std::ops::{BitAnd, BitOr, BitOrAssign};
#[allow(dead_code)]

/// zwp_linux_buffer_params_v1:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZwpLinuxBufferParamsV1Error {
    /// the dmabuf_batch object has already been used to create a wl_buffer
    AlreadyUsed = 0u32,
    /// plane index out of bounds
    PlaneIdx = 1u32,
    /// the plane index was already set
    PlaneSet = 2u32,
    /// missing or too many planes to create a buffer
    Incomplete = 3u32,
    /// format not supported
    InvalidFormat = 4u32,
    /// invalid width or height
    InvalidDimensions = 5u32,
    /// offset + stride * height goes out of dmabuf bounds
    OutOfBounds = 6u32,
    /// invalid wl_buffer resulted from importing dmabufs via                the create_immed request on given buffer_params
    InvalidWlBuffer = 7u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for ZwpLinuxBufferParamsV1Error {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::AlreadyUsed,
            1 => Self::PlaneIdx,
            2 => Self::PlaneSet,
            3 => Self::Incomplete,
            4 => Self::InvalidFormat,
            5 => Self::InvalidDimensions,
            6 => Self::OutOfBounds,
            7 => Self::InvalidWlBuffer,
            _ => Self::Unexpected,
        }
    }
}

/// zwp_linux_buffer_params_v1:flags enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZwpLinuxBufferParamsV1Flags {
    value: u32,
}

impl From<u32> for ZwpLinuxBufferParamsV1Flags {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl ZwpLinuxBufferParamsV1Flags {
    /// contents are y-inverted
    pub const YINVERT:Self = ZwpLinuxBufferParamsV1Flags{ value: 1 };
    /// content is interlaced
    pub const INTERLACED:Self = ZwpLinuxBufferParamsV1Flags{ value: 2 };
    /// bottom field first
    pub const BOTTOMFIRST:Self = ZwpLinuxBufferParamsV1Flags{ value: 4 };

    pub fn new() -> Self {
        ZwpLinuxBufferParamsV1Flags { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for ZwpLinuxBufferParamsV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// zwp_linux_dmabuf_feedback_v1:tranche_flags enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZwpLinuxDmabufFeedbackV1TrancheFlags {
    value: u32,
}

impl From<u32> for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl ZwpLinuxDmabufFeedbackV1TrancheFlags {
    /// direct scan-out tranche
    pub const SCANOUT:Self = ZwpLinuxDmabufFeedbackV1TrancheFlags{ value: 1 };

    pub fn new() -> Self {
        ZwpLinuxDmabufFeedbackV1TrancheFlags { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for ZwpLinuxDmabufFeedbackV1TrancheFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// xdg_wm_base:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgWmBaseError {
    /// given wl_surface has another role
    Role = 0u32,
    /// xdg_wm_base was destroyed before children
    DefunctSurfaces = 1u32,
    /// the client tried to map or destroy a non-topmost popup
    NotTheTopmostPopup = 2u32,
    /// the client specified an invalid popup parent surface
    InvalidPopupParent = 3u32,
    /// the client provided an invalid surface state
    InvalidSurfaceState = 4u32,
    /// the client provided an invalid positioner
    InvalidPositioner = 5u32,
    /// the client didn’t respond to a ping event in time
    Unresponsive = 6u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgWmBaseError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Role,
            1 => Self::DefunctSurfaces,
            2 => Self::NotTheTopmostPopup,
            3 => Self::InvalidPopupParent,
            4 => Self::InvalidSurfaceState,
            5 => Self::InvalidPositioner,
            6 => Self::Unresponsive,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_positioner:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgPositionerError {
    /// invalid input provided
    InvalidInput = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgPositionerError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidInput,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_positioner:anchor enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgPositionerAnchor {
    None = 0u32,
    Top = 1u32,
    Bottom = 2u32,
    Left = 3u32,
    Right = 4u32,
    TopLeft = 5u32,
    BottomLeft = 6u32,
    TopRight = 7u32,
    BottomRight = 8u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgPositionerAnchor {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Top,
            2 => Self::Bottom,
            3 => Self::Left,
            4 => Self::Right,
            5 => Self::TopLeft,
            6 => Self::BottomLeft,
            7 => Self::TopRight,
            8 => Self::BottomRight,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_positioner:gravity enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgPositionerGravity {
    None = 0u32,
    Top = 1u32,
    Bottom = 2u32,
    Left = 3u32,
    Right = 4u32,
    TopLeft = 5u32,
    BottomLeft = 6u32,
    TopRight = 7u32,
    BottomRight = 8u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgPositionerGravity {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Top,
            2 => Self::Bottom,
            3 => Self::Left,
            4 => Self::Right,
            5 => Self::TopLeft,
            6 => Self::BottomLeft,
            7 => Self::TopRight,
            8 => Self::BottomRight,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_positioner:constraint_adjustment enum
/// The constraint adjustment value define ways the compositor will adjust
/// the position of the surface, if the unadjusted position would result
/// in the surface being partly constrained.
///
/// Whether a surface is considered 'constrained' is left to the compositor
/// to determine. For example, the surface may be partly outside the
/// compositor's defined 'work area', thus necessitating the child surface's
/// position be adjusted until it is entirely inside the work area.
///
/// The adjustments can be combined, according to a defined precedence: 1)
/// Flip, 2) Slide, 3) Resize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XdgPositionerConstraintAdjustment {
    value: u32,
}

impl From<u32> for XdgPositionerConstraintAdjustment {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl XdgPositionerConstraintAdjustment {
    pub const NONE:Self = XdgPositionerConstraintAdjustment{ value: 0 };
    pub const SLIDEX:Self = XdgPositionerConstraintAdjustment{ value: 1 };
    pub const SLIDEY:Self = XdgPositionerConstraintAdjustment{ value: 2 };
    pub const FLIPX:Self = XdgPositionerConstraintAdjustment{ value: 4 };
    pub const FLIPY:Self = XdgPositionerConstraintAdjustment{ value: 8 };
    pub const RESIZEX:Self = XdgPositionerConstraintAdjustment{ value: 16 };
    pub const RESIZEY:Self = XdgPositionerConstraintAdjustment{ value: 32 };

    pub fn new() -> Self {
        XdgPositionerConstraintAdjustment { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for XdgPositionerConstraintAdjustment {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// xdg_surface:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgSurfaceError {
    /// Surface was not fully constructed
    NotConstructed = 1u32,
    /// Surface was already constructed
    AlreadyConstructed = 2u32,
    /// Attaching a buffer to an unconfigured surface
    UnconfiguredBuffer = 3u32,
    /// Invalid serial number when acking a configure event
    InvalidSerial = 4u32,
    /// Width or height was zero or negative
    InvalidSize = 5u32,
    /// Surface was destroyed before its role object
    DefunctRoleObject = 6u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgSurfaceError {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::NotConstructed,
            2 => Self::AlreadyConstructed,
            3 => Self::UnconfiguredBuffer,
            4 => Self::InvalidSerial,
            5 => Self::InvalidSize,
            6 => Self::DefunctRoleObject,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_toplevel:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgToplevelError {
    /// provided value is         not a valid variant of the resize_edge enum
    InvalidResizeEdge = 0u32,
    /// invalid parent toplevel
    InvalidParent = 1u32,
    /// client provided an invalid min or max size
    InvalidSize = 2u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgToplevelError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidResizeEdge,
            1 => Self::InvalidParent,
            2 => Self::InvalidSize,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_toplevel:resize_edge enum
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgToplevelResizeEdge {
    None = 0u32,
    Top = 1u32,
    Bottom = 2u32,
    Left = 4u32,
    TopLeft = 5u32,
    BottomLeft = 6u32,
    Right = 8u32,
    TopRight = 9u32,
    BottomRight = 10u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgToplevelResizeEdge {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Top,
            2 => Self::Bottom,
            4 => Self::Left,
            5 => Self::TopLeft,
            6 => Self::BottomLeft,
            8 => Self::Right,
            9 => Self::TopRight,
            10 => Self::BottomRight,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_toplevel:state enum
/// The different state values used on the surface. This is designed for
/// state values like maximized, fullscreen. It is paired with the
/// configure event to ensure that both the client and the compositor
/// setting the state can be synchronized.
///
/// States set in this way are double-buffered. They will get applied on
/// the next commit.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgToplevelState {
    /// the surface is maximized
    Maximized = 1u32,
    /// the surface is fullscreen
    Fullscreen = 2u32,
    /// the surface is being resized
    Resizing = 3u32,
    /// the surface is now activated
    Activated = 4u32,
    TiledLeft = 5u32,
    TiledRight = 6u32,
    TiledTop = 7u32,
    TiledBottom = 8u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgToplevelState {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Maximized,
            2 => Self::Fullscreen,
            3 => Self::Resizing,
            4 => Self::Activated,
            5 => Self::TiledLeft,
            6 => Self::TiledRight,
            7 => Self::TiledTop,
            8 => Self::TiledBottom,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_toplevel:wm_capabilities enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgToplevelWmCapabilities {
    /// show_window_menu is available
    WindowMenu = 1u32,
    /// set_maximized and unset_maximized are available
    Maximize = 2u32,
    /// set_fullscreen and unset_fullscreen are available
    Fullscreen = 3u32,
    /// set_minimized is available
    Minimize = 4u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgToplevelWmCapabilities {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::WindowMenu,
            2 => Self::Maximize,
            3 => Self::Fullscreen,
            4 => Self::Minimize,
            _ => Self::Unexpected,
        }
    }
}

/// xdg_popup:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgPopupError {
    /// tried to grab after being mapped
    InvalidGrab = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for XdgPopupError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidGrab,
            _ => Self::Unexpected,
        }
    }
}

/// wl_display:error enum
/// These errors are global and can be emitted in response to any
/// server request.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlDisplayError {
    /// server couldn't find object
    InvalidObject = 0u32,
    /// method doesn't exist on the specified interface or malformed request
    InvalidMethod = 1u32,
    /// server is out of memory
    NoMemory = 2u32,
    /// implementation error in compositor
    Implementation = 3u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlDisplayError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidObject,
            1 => Self::InvalidMethod,
            2 => Self::NoMemory,
            3 => Self::Implementation,
            _ => Self::Unexpected,
        }
    }
}

/// wl_shm:error enum
/// These errors can be emitted in response to wl_shm requests.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlShmError {
    /// buffer format is not known
    InvalidFormat = 0u32,
    /// invalid size or stride during pool or buffer creation
    InvalidStride = 1u32,
    /// mmapping the file descriptor failed
    InvalidFd = 2u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlShmError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidFormat,
            1 => Self::InvalidStride,
            2 => Self::InvalidFd,
            _ => Self::Unexpected,
        }
    }
}

/// wl_shm:format enum
/// This describes the memory layout of an individual pixel.
///
/// All renderers should support argb8888 and xrgb8888 but any other
/// formats are optional and may not be supported by the particular
/// renderer in use.
///
/// The drm format codes match the macros defined in drm_fourcc.h, except
/// argb8888 and xrgb8888. The formats actually supported by the compositor
/// will be reported by the format event.
///
/// For all wl_shm formats and unless specified in another protocol
/// extension, pre-multiplied alpha is used for pixel values.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlShmFormat {
    /// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
    ARGB8888 = 0u32,
    /// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
    XRGB8888 = 1u32,
    /// 8-bit color index format, [7:0] C
    C8 = 0x20203843u32,
    /// 8-bit RGB format, [7:0] R:G:B 3:3:2
    RGB332 = 0x38424752u32,
    /// 8-bit BGR format, [7:0] B:G:R 2:3:3
    BGR233 = 0x38524742u32,
    /// 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
    XRGB4444 = 0x32315258u32,
    /// 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
    XBGR4444 = 0x32314258u32,
    /// 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
    RGBX4444 = 0x32315852u32,
    /// 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
    BGRX4444 = 0x32315842u32,
    /// 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
    ARGB4444 = 0x32315241u32,
    /// 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
    ABGR4444 = 0x32314241u32,
    /// 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
    RGBA4444 = 0x32314152u32,
    /// 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
    BGRA4444 = 0x32314142u32,
    /// 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
    XRGB1555 = 0x35315258u32,
    /// 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
    XBGR1555 = 0x35314258u32,
    /// 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
    RGBX5551 = 0x35315852u32,
    /// 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
    BGRX5551 = 0x35315842u32,
    /// 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
    ARGB1555 = 0x35315241u32,
    /// 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
    ABGR1555 = 0x35314241u32,
    /// 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
    RGBA5551 = 0x35314152u32,
    /// 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
    BGRA5551 = 0x35314142u32,
    /// 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
    RGB565 = 0x36314752u32,
    /// 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
    BGR565 = 0x36314742u32,
    /// 24-bit RGB format, [23:0] R:G:B little endian
    RGB888 = 0x34324752u32,
    /// 24-bit BGR format, [23:0] B:G:R little endian
    BGR888 = 0x34324742u32,
    /// 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
    XBGR8888 = 0x34324258u32,
    /// 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
    RGBX8888 = 0x34325852u32,
    /// 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
    BGRX8888 = 0x34325842u32,
    /// 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
    ABGR8888 = 0x34324241u32,
    /// 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
    RGBA8888 = 0x34324152u32,
    /// 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
    BGRA8888 = 0x34324142u32,
    /// 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
    XRGB2101010 = 0x30335258u32,
    /// 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
    XBGR2101010 = 0x30334258u32,
    /// 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
    RGBX1010102 = 0x30335852u32,
    /// 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
    BGRX1010102 = 0x30335842u32,
    /// 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
    ARGB2101010 = 0x30335241u32,
    /// 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
    ABGR2101010 = 0x30334241u32,
    /// 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
    RGBA1010102 = 0x30334152u32,
    /// 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
    BGRA1010102 = 0x30334142u32,
    /// packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
    YUYV = 0x56595559u32,
    /// packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
    YVYU = 0x55595659u32,
    /// packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
    UYVY = 0x59565955u32,
    /// packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
    VYUY = 0x59555956u32,
    /// packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
    AYUV = 0x56555941u32,
    /// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
    NV12 = 0x3231564eu32,
    /// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
    NV21 = 0x3132564eu32,
    /// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
    NV16 = 0x3631564eu32,
    /// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
    NV61 = 0x3136564eu32,
    /// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
    YUV410 = 0x39565559u32,
    /// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
    YVU410 = 0x39555659u32,
    /// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
    YUV411 = 0x31315559u32,
    /// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
    YVU411 = 0x31315659u32,
    /// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
    YUV420 = 0x32315559u32,
    /// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
    YVU420 = 0x32315659u32,
    /// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
    YUV422 = 0x36315559u32,
    /// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
    YVU422 = 0x36315659u32,
    /// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
    YUV444 = 0x34325559u32,
    /// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
    YVU444 = 0x34325659u32,
    /// [7:0] R
    R8 = 0x20203852u32,
    /// [15:0] R little endian
    R16 = 0x20363152u32,
    /// [15:0] R:G 8:8 little endian
    RG88 = 0x38384752u32,
    /// [15:0] G:R 8:8 little endian
    GR88 = 0x38385247u32,
    /// [31:0] R:G 16:16 little endian
    RG1616 = 0x32334752u32,
    /// [31:0] G:R 16:16 little endian
    GR1616 = 0x32335247u32,
    /// [63:0] x:R:G:B 16:16:16:16 little endian
    XRGB16161616F = 0x48345258u32,
    /// [63:0] x:B:G:R 16:16:16:16 little endian
    XBGR16161616F = 0x48344258u32,
    /// [63:0] A:R:G:B 16:16:16:16 little endian
    ARGB16161616F = 0x48345241u32,
    /// [63:0] A:B:G:R 16:16:16:16 little endian
    ABGR16161616F = 0x48344241u32,
    /// [31:0] X:Y:Cb:Cr 8:8:8:8 little endian
    XYUV8888 = 0x56555958u32,
    /// [23:0] Cr:Cb:Y 8:8:8 little endian
    VUY888 = 0x34325556u32,
    /// Y followed by U then V, 10:10:10. Non-linear modifier only
    VUY101010 = 0x30335556u32,
    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels
    Y210 = 0x30313259u32,
    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels
    Y212 = 0x32313259u32,
    /// [63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels
    Y216 = 0x36313259u32,
    /// [31:0] A:Cr:Y:Cb 2:10:10:10 little endian
    Y410 = 0x30313459u32,
    /// [63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    Y412 = 0x32313459u32,
    /// [63:0] A:Cr:Y:Cb 16:16:16:16 little endian
    Y416 = 0x36313459u32,
    /// [31:0] X:Cr:Y:Cb 2:10:10:10 little endian
    XVYU2101010 = 0x30335658u32,
    /// [63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    XVYU12_16161616 = 0x36335658u32,
    /// [63:0] X:Cr:Y:Cb 16:16:16:16 little endian
    XVYU16161616 = 0x38345658u32,
    /// [63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    Y0L0 = 0x304c3059u32,
    /// [63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    X0L0 = 0x304c3058u32,
    /// [63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    Y0L2 = 0x324c3059u32,
    /// [63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    X0L2 = 0x324c3058u32,
    YUV420_8BIT = 0x38305559u32,
    YUV420_10BIT = 0x30315559u32,
    XRGB8888_A8 = 0x38415258u32,
    XBGR8888_A8 = 0x38414258u32,
    RGBX8888_A8 = 0x38415852u32,
    BGRX8888_A8 = 0x38415842u32,
    RGB888_A8 = 0x38413852u32,
    BGR888_A8 = 0x38413842u32,
    RGB565_A8 = 0x38413552u32,
    BGR565_A8 = 0x38413542u32,
    /// non-subsampled Cr:Cb plane
    NV24 = 0x3432564eu32,
    /// non-subsampled Cb:Cr plane
    NV42 = 0x3234564eu32,
    /// 2x1 subsampled Cr:Cb plane, 10 bit per channel
    P210 = 0x30313250u32,
    /// 2x2 subsampled Cr:Cb plane 10 bits per channel
    P010 = 0x30313050u32,
    /// 2x2 subsampled Cr:Cb plane 12 bits per channel
    P012 = 0x32313050u32,
    /// 2x2 subsampled Cr:Cb plane 16 bits per channel
    P016 = 0x36313050u32,
    /// [63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian
    AXBXGXRX106106106106 = 0x30314241u32,
    /// 2x2 subsampled Cr:Cb plane
    NV15 = 0x3531564eu32,
    Q410 = 0x30313451u32,
    Q401 = 0x31303451u32,
    /// [63:0] x:R:G:B 16:16:16:16 little endian
    XRGB16161616 = 0x38345258u32,
    /// [63:0] x:B:G:R 16:16:16:16 little endian
    XBGR16161616 = 0x38344258u32,
    /// [63:0] A:R:G:B 16:16:16:16 little endian
    ARGB16161616 = 0x38345241u32,
    /// [63:0] A:B:G:R 16:16:16:16 little endian
    ABGR16161616 = 0x38344241u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlShmFormat {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::ARGB8888,
            1 => Self::XRGB8888,
            0x20203843 => Self::C8,
            0x38424752 => Self::RGB332,
            0x38524742 => Self::BGR233,
            0x32315258 => Self::XRGB4444,
            0x32314258 => Self::XBGR4444,
            0x32315852 => Self::RGBX4444,
            0x32315842 => Self::BGRX4444,
            0x32315241 => Self::ARGB4444,
            0x32314241 => Self::ABGR4444,
            0x32314152 => Self::RGBA4444,
            0x32314142 => Self::BGRA4444,
            0x35315258 => Self::XRGB1555,
            0x35314258 => Self::XBGR1555,
            0x35315852 => Self::RGBX5551,
            0x35315842 => Self::BGRX5551,
            0x35315241 => Self::ARGB1555,
            0x35314241 => Self::ABGR1555,
            0x35314152 => Self::RGBA5551,
            0x35314142 => Self::BGRA5551,
            0x36314752 => Self::RGB565,
            0x36314742 => Self::BGR565,
            0x34324752 => Self::RGB888,
            0x34324742 => Self::BGR888,
            0x34324258 => Self::XBGR8888,
            0x34325852 => Self::RGBX8888,
            0x34325842 => Self::BGRX8888,
            0x34324241 => Self::ABGR8888,
            0x34324152 => Self::RGBA8888,
            0x34324142 => Self::BGRA8888,
            0x30335258 => Self::XRGB2101010,
            0x30334258 => Self::XBGR2101010,
            0x30335852 => Self::RGBX1010102,
            0x30335842 => Self::BGRX1010102,
            0x30335241 => Self::ARGB2101010,
            0x30334241 => Self::ABGR2101010,
            0x30334152 => Self::RGBA1010102,
            0x30334142 => Self::BGRA1010102,
            0x56595559 => Self::YUYV,
            0x55595659 => Self::YVYU,
            0x59565955 => Self::UYVY,
            0x59555956 => Self::VYUY,
            0x56555941 => Self::AYUV,
            0x3231564e => Self::NV12,
            0x3132564e => Self::NV21,
            0x3631564e => Self::NV16,
            0x3136564e => Self::NV61,
            0x39565559 => Self::YUV410,
            0x39555659 => Self::YVU410,
            0x31315559 => Self::YUV411,
            0x31315659 => Self::YVU411,
            0x32315559 => Self::YUV420,
            0x32315659 => Self::YVU420,
            0x36315559 => Self::YUV422,
            0x36315659 => Self::YVU422,
            0x34325559 => Self::YUV444,
            0x34325659 => Self::YVU444,
            0x20203852 => Self::R8,
            0x20363152 => Self::R16,
            0x38384752 => Self::RG88,
            0x38385247 => Self::GR88,
            0x32334752 => Self::RG1616,
            0x32335247 => Self::GR1616,
            0x48345258 => Self::XRGB16161616F,
            0x48344258 => Self::XBGR16161616F,
            0x48345241 => Self::ARGB16161616F,
            0x48344241 => Self::ABGR16161616F,
            0x56555958 => Self::XYUV8888,
            0x34325556 => Self::VUY888,
            0x30335556 => Self::VUY101010,
            0x30313259 => Self::Y210,
            0x32313259 => Self::Y212,
            0x36313259 => Self::Y216,
            0x30313459 => Self::Y410,
            0x32313459 => Self::Y412,
            0x36313459 => Self::Y416,
            0x30335658 => Self::XVYU2101010,
            0x36335658 => Self::XVYU12_16161616,
            0x38345658 => Self::XVYU16161616,
            0x304c3059 => Self::Y0L0,
            0x304c3058 => Self::X0L0,
            0x324c3059 => Self::Y0L2,
            0x324c3058 => Self::X0L2,
            0x38305559 => Self::YUV420_8BIT,
            0x30315559 => Self::YUV420_10BIT,
            0x38415258 => Self::XRGB8888_A8,
            0x38414258 => Self::XBGR8888_A8,
            0x38415852 => Self::RGBX8888_A8,
            0x38415842 => Self::BGRX8888_A8,
            0x38413852 => Self::RGB888_A8,
            0x38413842 => Self::BGR888_A8,
            0x38413552 => Self::RGB565_A8,
            0x38413542 => Self::BGR565_A8,
            0x3432564e => Self::NV24,
            0x3234564e => Self::NV42,
            0x30313250 => Self::P210,
            0x30313050 => Self::P010,
            0x32313050 => Self::P012,
            0x36313050 => Self::P016,
            0x30314241 => Self::AXBXGXRX106106106106,
            0x3531564e => Self::NV15,
            0x30313451 => Self::Q410,
            0x31303451 => Self::Q401,
            0x38345258 => Self::XRGB16161616,
            0x38344258 => Self::XBGR16161616,
            0x38345241 => Self::ARGB16161616,
            0x38344241 => Self::ABGR16161616,
            _ => Self::Unexpected,
        }
    }
}

/// wl_data_offer:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlDataOfferError {
    /// finish request was called untimely
    InvalidFinish = 0u32,
    /// action mask contains invalid values
    InvalidActionMask = 1u32,
    /// action argument has an invalid value
    InvalidAction = 2u32,
    /// offer doesn't accept this request
    InvalidOffer = 3u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlDataOfferError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidFinish,
            1 => Self::InvalidActionMask,
            2 => Self::InvalidAction,
            3 => Self::InvalidOffer,
            _ => Self::Unexpected,
        }
    }
}

/// wl_data_source:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlDataSourceError {
    /// action mask contains invalid values
    InvalidActionMask = 0u32,
    /// source doesn't accept this request
    InvalidSource = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlDataSourceError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidActionMask,
            1 => Self::InvalidSource,
            _ => Self::Unexpected,
        }
    }
}

/// wl_data_device:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlDataDeviceError {
    /// given wl_surface has another role
    Role = 0u32,
    /// source has already been used
    UsedSource = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlDataDeviceError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Role,
            1 => Self::UsedSource,
            _ => Self::Unexpected,
        }
    }
}

/// wl_data_device_manager:dnd_action enum
/// This is a bitmask of the available/preferred actions in a
/// drag-and-drop operation.
///
/// In the compositor, the selected action is a result of matching the
/// actions offered by the source and destination sides.  "action" events
/// with a "none" action will be sent to both source and destination if
/// there is no match. All further checks will effectively happen on
/// (source actions ∩ destination actions).
///
/// In addition, compositors may also pick different actions in
/// reaction to key modifiers being pressed. One common design that
/// is used in major toolkits (and the behavior recommended for
/// compositors) is:
///
/// - If no modifiers are pressed, the first match (in bit order)
/// will be used.
/// - Pressing Shift selects "move", if enabled in the mask.
/// - Pressing Control selects "copy", if enabled in the mask.
///
/// Behavior beyond that is considered implementation-dependent.
/// Compositors may for example bind other modifiers (like Alt/Meta)
/// or drags initiated with other buttons than BTN_LEFT to specific
/// actions (e.g. "ask").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlDataDeviceManagerDndAction {
    value: u32,
}

impl From<u32> for WlDataDeviceManagerDndAction {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl WlDataDeviceManagerDndAction {
    /// no action
    pub const NONE:Self = WlDataDeviceManagerDndAction{ value: 0 };
    /// copy action
    pub const COPY:Self = WlDataDeviceManagerDndAction{ value: 1 };
    /// move action
    pub const MOVE:Self = WlDataDeviceManagerDndAction{ value: 2 };
    /// ask action
    pub const ASK:Self = WlDataDeviceManagerDndAction{ value: 4 };

    pub fn new() -> Self {
        WlDataDeviceManagerDndAction { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for WlDataDeviceManagerDndAction {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// wl_shell:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlShellError {
    /// given wl_surface has another role
    Role = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlShellError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Role,
            _ => Self::Unexpected,
        }
    }
}

/// wl_shell_surface:resize enum
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation. The server may
/// use this information to adapt its behavior, e.g. choose
/// an appropriate cursor image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlShellSurfaceResize {
    value: u32,
}

impl From<u32> for WlShellSurfaceResize {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl WlShellSurfaceResize {
    /// no edge
    pub const NONE:Self = WlShellSurfaceResize{ value: 0 };
    /// top edge
    pub const TOP:Self = WlShellSurfaceResize{ value: 1 };
    /// bottom edge
    pub const BOTTOM:Self = WlShellSurfaceResize{ value: 2 };
    /// left edge
    pub const LEFT:Self = WlShellSurfaceResize{ value: 4 };
    /// top and left edges
    pub const TOPLEFT:Self = WlShellSurfaceResize{ value: 5 };
    /// bottom and left edges
    pub const BOTTOMLEFT:Self = WlShellSurfaceResize{ value: 6 };
    /// right edge
    pub const RIGHT:Self = WlShellSurfaceResize{ value: 8 };
    /// top and right edges
    pub const TOPRIGHT:Self = WlShellSurfaceResize{ value: 9 };
    /// bottom and right edges
    pub const BOTTOMRIGHT:Self = WlShellSurfaceResize{ value: 10 };

    pub fn new() -> Self {
        WlShellSurfaceResize { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for WlShellSurfaceResize {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for WlShellSurfaceResize {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for WlShellSurfaceResize {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// wl_shell_surface:transient enum
/// These flags specify details of the expected behaviour
/// of transient surfaces. Used in the set_transient request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlShellSurfaceTransient {
    value: u32,
}

impl From<u32> for WlShellSurfaceTransient {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl WlShellSurfaceTransient {
    /// do not set keyboard focus
    pub const INACTIVE:Self = WlShellSurfaceTransient{ value: 0x1 };

    pub fn new() -> Self {
        WlShellSurfaceTransient { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for WlShellSurfaceTransient {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for WlShellSurfaceTransient {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for WlShellSurfaceTransient {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// wl_shell_surface:fullscreen_method enum
/// Hints to indicate to the compositor how to deal with a conflict
/// between the dimensions of the surface and the dimensions of the
/// output. The compositor is free to ignore this parameter.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlShellSurfaceFullscreenMethod {
    /// no preference, apply default policy
    Default = 0u32,
    /// scale, preserve the surface's aspect ratio and center on output
    Scale = 1u32,
    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    Driver = 2u32,
    /// no upscaling, center on output and add black borders to compensate size mismatch
    Fill = 3u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlShellSurfaceFullscreenMethod {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Scale,
            2 => Self::Driver,
            3 => Self::Fill,
            _ => Self::Unexpected,
        }
    }
}

/// wl_surface:error enum
/// These errors can be emitted in response to wl_surface requests.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlSurfaceError {
    /// buffer scale value is invalid
    InvalidScale = 0u32,
    /// buffer transform value is invalid
    InvalidTransform = 1u32,
    /// buffer size is invalid
    InvalidSize = 2u32,
    /// buffer offset is invalid
    InvalidOffset = 3u32,
    /// surface was destroyed before its role object
    DefunctRoleObject = 4u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlSurfaceError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::InvalidScale,
            1 => Self::InvalidTransform,
            2 => Self::InvalidSize,
            3 => Self::InvalidOffset,
            4 => Self::DefunctRoleObject,
            _ => Self::Unexpected,
        }
    }
}

/// wl_seat:capability enum
/// This is a bitmask of capabilities this seat has; if a member is
/// set, then it is present on the seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlSeatCapability {
    value: u32,
}

impl From<u32> for WlSeatCapability {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl WlSeatCapability {
    /// the seat has pointer devices
    pub const POINTER:Self = WlSeatCapability{ value: 1 };
    /// the seat has one or more keyboards
    pub const KEYBOARD:Self = WlSeatCapability{ value: 2 };
    /// the seat has touch devices
    pub const TOUCH:Self = WlSeatCapability{ value: 4 };

    pub fn new() -> Self {
        WlSeatCapability { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for WlSeatCapability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for WlSeatCapability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for WlSeatCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// wl_seat:error enum
/// These errors can be emitted in response to wl_seat requests.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability
    MissingCapability = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlSeatError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::MissingCapability,
            _ => Self::Unexpected,
        }
    }
}

/// wl_pointer:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlPointerError {
    /// given wl_surface has another role
    Role = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlPointerError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Role,
            _ => Self::Unexpected,
        }
    }
}

/// wl_pointer:button_state enum
/// Describes the physical state of a button that produced the button
/// event.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlPointerButtonState {
    /// the button is not pressed
    Released = 0u32,
    /// the button is pressed
    Pressed = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlPointerButtonState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Released,
            1 => Self::Pressed,
            _ => Self::Unexpected,
        }
    }
}

/// wl_pointer:axis enum
/// Describes the axis types of scroll events.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlPointerAxis {
    /// vertical axis
    VerticalScroll = 0u32,
    /// horizontal axis
    HorizontalScroll = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlPointerAxis {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::VerticalScroll,
            1 => Self::HorizontalScroll,
            _ => Self::Unexpected,
        }
    }
}

/// wl_pointer:axis_source enum
/// Describes the source types for axis events. This indicates to the
/// client how an axis event was physically generated; a client may
/// adjust the user interface accordingly. For example, scroll events
/// from a "finger" source may be in a smooth coordinate space with
/// kinetic scrolling whereas a "wheel" source may be in discrete steps
/// of a number of lines.
///
/// The "continuous" axis source is a device generating events in a
/// continuous coordinate space, but using something other than a
/// finger. One example for this source is button-based scrolling where
/// the vertical motion of a device is converted to scroll events while
/// a button is held down.
///
/// The "wheel tilt" axis source indicates that the actual device is a
/// wheel but the scroll event is not caused by a rotation but a
/// (usually sideways) tilt of the wheel.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlPointerAxisSource {
    /// a physical wheel rotation
    Wheel = 0u32,
    /// finger on a touch surface
    Finger = 1u32,
    /// continuous coordinate space
    Continuous = 2u32,
    /// a physical wheel tilt
    WheelTilt = 3u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlPointerAxisSource {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Wheel,
            1 => Self::Finger,
            2 => Self::Continuous,
            3 => Self::WheelTilt,
            _ => Self::Unexpected,
        }
    }
}

/// wl_pointer:axis_relative_direction enum
/// This specifies the direction of the physical motion that caused a
/// wl_pointer.axis event, relative to the wl_pointer.axis direction.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlPointerAxisRelativeDirection {
    /// physical motion matches axis direction
    Identical = 0u32,
    /// physical motion is the inverse of the axis direction
    Inverted = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlPointerAxisRelativeDirection {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Identical,
            1 => Self::Inverted,
            _ => Self::Unexpected,
        }
    }
}

/// wl_keyboard:keymap_format enum
/// This specifies the format of the keymap provided to the
/// client with the wl_keyboard.keymap event.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlKeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode
    NoKeymap = 0u32,
    /// libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode
    XkbV1 = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlKeyboardKeymapFormat {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::NoKeymap,
            1 => Self::XkbV1,
            _ => Self::Unexpected,
        }
    }
}

/// wl_keyboard:key_state enum
/// Describes the physical state of a key that produced the key event.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlKeyboardKeyState {
    /// key is not pressed
    Released = 0u32,
    /// key is pressed
    Pressed = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlKeyboardKeyState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Released,
            1 => Self::Pressed,
            _ => Self::Unexpected,
        }
    }
}

/// wl_output:subpixel enum
/// This enumeration describes how the physical
/// pixels on an output are laid out.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlOutputSubpixel {
    /// unknown geometry
    Unknown = 0u32,
    /// no geometry
    None = 1u32,
    /// horizontal RGB
    HorizontalRgb = 2u32,
    /// horizontal BGR
    HorizontalBgr = 3u32,
    /// vertical RGB
    VerticalRgb = 4u32,
    /// vertical BGR
    VerticalBgr = 5u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlOutputSubpixel {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::HorizontalRgb,
            3 => Self::HorizontalBgr,
            4 => Self::VerticalRgb,
            5 => Self::VerticalBgr,
            _ => Self::Unexpected,
        }
    }
}

/// wl_output:transform enum
/// This describes the transform that a compositor will apply to a
/// surface to compensate for the rotation or mirroring of an
/// output device.
///
/// The flipped values correspond to an initial flip around a
/// vertical axis followed by rotation.
///
/// The purpose is mainly to allow clients to render accordingly and
/// tell the compositor, so that for fullscreen surfaces, the
/// compositor will still be able to scan out directly from client
/// surfaces.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlOutputTransform {
    /// no transform
    TransformNormal = 0u32,
    /// 90 degrees counter-clockwise
    Transform90 = 1u32,
    /// 180 degrees counter-clockwise
    Transform180 = 2u32,
    /// 270 degrees counter-clockwise
    Transform270 = 3u32,
    /// 180 degree flip around a vertical axis
    TransformFlipped = 4u32,
    /// flip and rotate 90 degrees counter-clockwise
    TransformFlipped90 = 5u32,
    /// flip and rotate 180 degrees counter-clockwise
    TransformFlipped180 = 6u32,
    /// flip and rotate 270 degrees counter-clockwise
    TransformFlipped270 = 7u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlOutputTransform {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::TransformNormal,
            1 => Self::Transform90,
            2 => Self::Transform180,
            3 => Self::Transform270,
            4 => Self::TransformFlipped,
            5 => Self::TransformFlipped90,
            6 => Self::TransformFlipped180,
            7 => Self::TransformFlipped270,
            _ => Self::Unexpected,
        }
    }
}

/// wl_output:mode enum
/// These flags describe properties of an output mode.
/// They are used in the flags bitfield of the mode event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlOutputMode {
    value: u32,
}

impl From<u32> for WlOutputMode {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl WlOutputMode {
    /// indicates this is the current mode
    pub const CURRENT:Self = WlOutputMode{ value: 0x1 };
    /// indicates this is the preferred mode
    pub const PREFERRED:Self = WlOutputMode{ value: 0x2 };

    pub fn new() -> Self {
        WlOutputMode { value: 0 }
    }

    pub fn contains(&self, flag: Self) -> bool {
        self.value & flag.value != 0
    }

    pub fn insert(&mut self, flag: Self) {
        self.value |= flag.value;
    }

    pub fn remove(&mut self, flag: Self) {
        self.value &= !flag.value;
    }
}

impl BitAnd for WlOutputMode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}

impl BitOr for WlOutputMode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for WlOutputMode {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

/// wl_subcompositor:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlSubcompositorError {
    /// the to-be sub-surface is invalid
    BadSurface = 0u32,
    /// the to-be sub-surface parent is invalid
    BadParent = 1u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlSubcompositorError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::BadSurface,
            1 => Self::BadParent,
            _ => Self::Unexpected,
        }
    }
}

/// wl_subsurface:error enum
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlSubsurfaceError {
    /// wl_surface is not a sibling or the parent
    BadSurface = 0u32,
    /// Unexpected value was receieved on the wire
    Unexpected,
}

impl From<u32> for WlSubsurfaceError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::BadSurface,
            _ => Self::Unexpected,
        }
    }
}
