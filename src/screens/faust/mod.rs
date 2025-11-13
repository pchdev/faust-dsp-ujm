
pub mod intro;
pub mod basics;
pub mod functions;
pub mod synthesis;
pub mod time;
pub mod control;

macro_rules! example {
    ($path:literal) => {
        include_str!(
            concat!("../../../examples/", $path)
        )
    };
}

pub(crate) use example;


