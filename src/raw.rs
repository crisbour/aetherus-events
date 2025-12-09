
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipeline {
    Emission   = 1,
    Mcrt       = 3,
    Detection  = 5,
    Processing = 7,
    // Other codes are free to be used for custom pipeline stages
}

impl TryFrom<u8> for Pipeline {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Emission),
            3 => Ok(Self::Mcrt),
            5 => Ok(Self::Detection),
            7 => Ok(Self::Processing),
            _ => Err("Invalid Pipeline value"),
        }
    }
}

// TODO: Perhaps should make it interop with u32, to allow for extension
// Then new would return Result<Self> in order to raise error when id doesn't feet in the
// underlying type
trait MatSurfId {
    fn new(id: u16) -> Self;
    fn id(&self) -> u16;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MatId(u16);

impl MatId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
    pub fn id(&self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfId(u16);

impl SurfId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
    pub fn id(&self) -> u16 {
        self.0
    }
}

// SuperType represents the 2-bit super type category [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McrtSuper {
    Interface = 0,
    Reflector = 1,
    Material  = 2,
    Custom    = 3,
}

// SubType for Interface events (6 bits, but simplified enum) [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    Reflection = 0,
    Refraction = 1,
    // Custom 2-63
}

// SubType for Reflector events [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reflect {
    Diffuse         = 0b000010,  // 00001x
    Specular        = 0b000100,  // 00010x
    Composite       = 0b000110,  // 00011x
    RetroReflective = 0b001000,
    CompRetroRef    = 0b001001,
    // Custom others
}

// MaterialInteraction encodes the interaction type (2 bits) [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    Absorption = 0b00,
    Inelastic  = 0b01,
    Elastic    = 0b10,
    Custom     = 0b11,
}

// ScatterType for scattering events (2 bits) [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inelastic {
    Raman        = 0b00,
    Fluorescence = 0b01,
}

// ScatterType for scattering events (2 bits) [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elastic {
    HenyeyGreenstein = 0b00,
    Mie              = 0b01,
    Rayleigh         = 0b10,
    SphericalCdf     = 0b11,
}

// Direction for scattering (2 bits) [file:1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Any      = 0b00,
    Forward  = 0b01,
    Side     = 0b10,
    Backward = 0b11,
}

// EventType trait for all event types [file:1].
pub trait EventType {
    fn from_raw(raw: u32) -> Self where Self: Sized; // Decodes from 32-bit format
    fn to_raw(&self) -> u32; // Encodes to 32-bit format
}

// MCEvent represents the 32-bit MCRT event encoding [file:1].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct McrtEvent<MSId: MatSurfId> {
    pub pipeline:   Pipeline,
    pub event_type: u8,
    pub inter_id:   MSId,
}

