//! HTML Emitter module for the Blog Builder.

mod emitter;
mod metadata;

pub use emitter::Emitter;

pub use metadata::{
    Metadata,
    CommandOption,
};