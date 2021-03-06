use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{SkColorMatrix, SkColorMatrix_Axis};

pub type ColorMatrix = Handle<SkColorMatrix>;

impl NativeDrop for SkColorMatrix {
    fn drop(&mut self) {}
}

impl NativePartialEq for SkColorMatrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkColorMatrix_equals(self, rhs) }
    }
}

impl Default for Handle<SkColorMatrix> {
    fn default() -> Self {
        ColorMatrix::construct(|cm| unsafe { (*cm).setIdentity() })
    }
}

impl Handle<SkColorMatrix> {
    pub fn set_identity(&mut self) {
        unsafe { self.native_mut().setIdentity() }
    }

    pub fn set_scale(
        &mut self,
        r_scale: f32,
        g_scale: f32,
        b_scale: f32,
        a_scale: impl Into<Option<f32>>,
    ) {
        unsafe {
            self.native_mut()
                .setScale(r_scale, g_scale, b_scale, a_scale.into().unwrap_or(1.0))
        }
    }

    pub fn set_row_major(&mut self, src: &[f32; 20]) {
        self.set_20(src)
    }

    pub fn get_row_major(&mut self, dst: &mut [f32; 20]) {
        self.get_20(dst);
    }

    pub fn set_rotate(&mut self, axis: Axis, degrees: f32) {
        unsafe { self.native_mut().setRotate(axis.into_native(), degrees) }
    }

    pub fn set_sin_cos(&mut self, axis: Axis, sine: f32, cosine: f32) {
        unsafe {
            self.native_mut()
                .setSinCos(axis.into_native(), sine, cosine)
        }
    }

    pub fn pre_rotate(&mut self, axis: Axis, degrees: f32) {
        unsafe { self.native_mut().preRotate(axis.into_native(), degrees) }
    }

    pub fn post_rotate(&mut self, axis: Axis, degrees: f32) {
        unsafe { self.native_mut().postRotate(axis.into_native(), degrees) }
    }

    pub fn post_translate(&mut self, dr: f32, dg: f32, db: f32, da: f32) {
        unsafe { self.native_mut().postTranslate(dr, dg, db, da) }
    }

    pub fn set_concat(&mut self, a: &ColorMatrix, b: &ColorMatrix) {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
    }

    pub fn pre_concat(&mut self, mat: &ColorMatrix) {
        let self_ptr = self.native() as *const _;
        unsafe { self.native_mut().setConcat(self_ptr, mat.native()) }
    }

    pub fn post_concat(&mut self, mat: &ColorMatrix) {
        let self_ptr = self.native() as *const _;
        unsafe { self.native_mut().setConcat(mat.native(), self_ptr) }
    }

    pub fn set_saturation(&mut self, sat: f32) {
        unsafe { self.native_mut().setSaturation(sat) }
    }

    pub fn set_rgb_2_yuv(&mut self) {
        unsafe { self.native_mut().setRGB2YUV() }
    }

    pub fn set_yuv_2_rgb(&mut self) {
        unsafe { self.native_mut().setYUV2RGB() }
    }

    pub fn get_20<'a>(&self, m: &'a mut [f32; 20]) -> &'a mut [f32; 20] {
        unsafe { sb::C_SkColorMatrix_get20(self.native(), m.as_mut_ptr()) };
        m
    }

    pub fn set_20(&mut self, m: &[f32; 20]) {
        unsafe { sb::C_SkColorMatrix_set20(self.native_mut(), m.as_ptr()) };
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Axis {
    R = SkColorMatrix_Axis::kR_Axis as _,
    G = SkColorMatrix_Axis::kG_Axis as _,
    B = SkColorMatrix_Axis::kB_Axis as _,
}

impl NativeTransmutable<SkColorMatrix_Axis> for Axis {}

#[test]
fn test_axis_layout() {
    Axis::test_layout();
}
