use anyhow::Result;
use opencv::{
    boxed_ref::BoxedRef,
    core::{Mat, MatTraitConst, Vec4b},
};
use platforms::capture::Frame;

/// A [`Mat`] that owns the external buffer.
#[derive(Debug)]
pub struct OwnedMat {
    rows: i32,
    cols: i32,
    bytes: Vec<u8>,
}

impl OwnedMat {
    #[inline]
    pub fn new(frame: Frame) -> Result<Self> {
        let owned = Self {
            rows: frame.height,
            cols: frame.width,
            bytes: frame.data,
        };
        let _ = owned.as_mat_inner()?;

        Ok(owned)
    }

    pub fn as_mat(&self) -> BoxedRef<'_, Mat> {
        self.as_mat_inner().unwrap()
    }

    fn as_mat_inner(&self) -> Result<BoxedRef<'_, Mat>> {
        Ok(Mat::new_rows_cols_with_bytes::<Vec4b>(
            self.rows,
            self.cols,
            &self.bytes,
        )?)
    }
}

#[cfg(debug_assertions)]
impl From<Mat> for OwnedMat {
    fn from(value: Mat) -> Self {
        use opencv::core::MatTraitConstManual;

        Self {
            rows: value.rows(),
            cols: value.cols(),
            bytes: value.data_bytes().unwrap().to_vec(),
        }
    }
}
