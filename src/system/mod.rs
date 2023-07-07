use {
    crate::*,
    std::{
        fmt::{
            Display,
            Formatter,
            Result,
        },
    },
};

#[derive(Copy,Clone,Debug)]
pub enum KeyEvent {
    Press { code: u32, },
    Release { code: u32, },
}

impl Display for KeyEvent {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            KeyEvent::Press { code, } => write!(f,"Press {{ code: {}, }}",code),
            KeyEvent::Release { code, } => write!(f,"Release {{ code: {}, }}",code),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Button {
    Left,
    Right,
    Middle,
}

impl Display for Button {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            Button::Left => write!(f,"Button::Left"),
            Button::Right => write!(f,"Button::Right"),
            Button::Middle => write!(f,"Button::Middle"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum PointerEvent {
    Down { position: Vec2<f32>, button: Button, },  // the pointer made contact with the device
    Up { position: Vec2<f32>, button: Button, },  // the pointer has stopped making contact with the device
    Move { position: Vec2<f32>, buttons: Vec<Button>, hover: bool, },  // the pointer has moved with respect to the device while the pointer is in contact with the device
    Cancel { position: Vec2<f32>, buttons: Vec<Button>, hover: bool, },  // the input from the pointer is no longer directed towards this receiver
    Start { position: Vec2<f32>, },  // a pan/zoom gesture was started
    Update { position: Vec2<f32>, scale: f32, },  // a pan/zoom gesture was updated
    End { position: Vec2<f32>, }, // a pan/zoom gesture was ended
    Scroll { position: Vec2<f32>, buttons: Vec<Button>, delta: Vec2<f32>, },  // a scroll indication was generated for this pointer (by, for instance, a mouse wheel)
}

impl Display for PointerEvent {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            PointerEvent::Down { position, button, } => write!(f,"Down {{ position: {},buttons: {}, }}",position,button),
            PointerEvent::Up { position, button, } => write!(f,"Up {{ position: {},buttons: {}, }}",position,button),
            PointerEvent::Move { position, hover, .. } => write!(f,"Move {{ position: {},buttons: TODO,hover: {}, }}",position,hover),
            PointerEvent::Cancel { position, hover, .. } => write!(f,"Leave {{ position: {},buttons: TODO,hover: {}, }}",position,hover),
            PointerEvent::Start { position, } => write!(f,"Start {{ position: {}, }}",position),
            PointerEvent::Update { position, scale, } => write!(f,"Update {{ position: {}, scale: {}, }}",position,scale),
            PointerEvent::End { position, } => write!(f,"End {{ position: {}, }}",position),
            PointerEvent::Scroll { position, delta, .. } => write!(f,"Scroll {{ position: {}, buttons: TODO, delta: {}, }}",position,delta),
        }
    }
}

pub enum MouseCursor {
    Arrow,
    VArrow,
    Hourglass,
    Crosshair,
    Finger,
    OpenHand,
    GrabbingHand,
    MagnifyingGlass,
    Caret,
    SlashedCircle,
    SizeNSEW,
    SizeNESW,
    SizeNWSE,
    SizeWE,
    SizeNS,
}

#[derive(Clone,Debug)]
pub enum Event {
    Key(KeyEvent),
    Pointer(PointerEvent),
    Configure(Rect<i32>),
    Expose(Rect<i32>),
    Close,
}

impl Display for Event {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            Event::Key(event) => write!(f,"{}",event),
            Event::Pointer(event) => write!(f,"{}",event),
            Event::Configure(rect) => write!(f,"Configure({})",rect),
            Event::Expose(rect) => write!(f,"Expose({})",rect),
            Event::Close => write!(f,"Close"),
        }
    }
}

#[cfg(system="linux")]
mod linux;
#[cfg(system="linux")]
pub use linux::*;
