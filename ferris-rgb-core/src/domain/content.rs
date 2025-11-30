pub enum ContentData {
    Text(String),
    Animation(Vec<u8>), // e.g. GIF data
    Image(Vec<u8>),     // e.g. PNG or JPEG data
}

pub struct Created {
    pub data: ContentData,
}

pub struct Content<S> {
    pub(crate) state: S,
}

impl<S> Content<S> {
    pub fn map_state<T>(self, f: impl FnOnce(S) -> T) -> Content<T> {
        Content { state: f(self.state) }
    }
    pub fn into_inner(self) -> S {
        self.state
    }
}

pub enum DecodedData {
    TextLayout(String), // e.g. formatted text
    Frames(Vec<u8>),    // e.g. decoded frames for animation
    RawImage(Vec<u8>),  // e.g. raw pixel data
}

pub struct Decoded {
    pub data: DecodedData,
}

pub struct Validated {
    pub data: DecodedData,
}

pub struct Renderable {
    pub framebuffer: Vec<u8>,
}
