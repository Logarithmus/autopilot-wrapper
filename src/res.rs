use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref RES_PATH: PathBuf =
        Path::new("/home/ITRANSITION.CORP/a.sinilo/MyProjects/Rust/JDE/jde_udc_autoimport/res")
            .to_path_buf();
}

#[macro_export]
macro_rules! gray_image {
    ($name:expr) => {
        Bitmap::new(
            image::open(RES_PATH.join($name))
                .expect("Can't open $name")
                .grayscale(),
            None,
        )
    };
}
