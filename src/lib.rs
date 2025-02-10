mod rect;
pub use rect::Rect;

mod detection;
pub use detection::{Face, FaceDetector, RustFacesError, RustFacesResult};

mod ort;

mod nms;
pub use nms::Nms;

mod imaging;
pub use imaging::{ToArray3, ToRgb8};

#[cfg(test)]
pub mod testing;

pub mod priorboxes;

mod blazeface;
pub use blazeface::{BlazeFace, BlazeFaceParams};

mod mtcnn;
pub use mtcnn::{MtCnn, MtCnnParams};

mod builder;

#[cfg(feature = "viz")]
pub mod viz;

pub use builder::{FaceDetection, FaceDetectorBuilder, InferParams, Provider};

