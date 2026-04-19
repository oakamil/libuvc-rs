use crate::device::DeviceHandle;
use crate::error::{Error, Result};
use uvc_sys::*;

#[derive(Copy, Clone, Debug)]
pub enum ScanningMode {
    Interlaced,
    Progressive,
}

#[derive(Copy, Clone, Debug)]
pub enum AutoExposureMode {
    Manual,
    Auto,
    ShutterPriority,
    AperturePriority,
}

#[derive(Copy, Clone, Debug)]
pub enum AutoExposurePriority {
    Constant,
    Variable,
}

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
    pub step: T,
    pub default: T,
}

impl<'a> DeviceHandle<'a> {
    pub fn scanning_mode(&self) -> Result<ScanningMode> {
        unsafe {
            let mut mode = std::mem::MaybeUninit::uninit();
            let err = uvc_get_scanning_mode(
                self.devh.as_ptr(),
                mode.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }
            match mode.assume_init() {
                0 => Ok(ScanningMode::Interlaced),
                1 => Ok(ScanningMode::Progressive),
                _ => Err(Error::Other),
            }
        }
    }

    pub fn ae_mode(&self) -> Result<AutoExposureMode> {
        unsafe {
            let mut mode = std::mem::MaybeUninit::uninit();
            let err = uvc_get_ae_mode(
                self.devh.as_ptr(),
                mode.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }
            match mode.assume_init() {
                1 => Ok(AutoExposureMode::Manual),
                2 => Ok(AutoExposureMode::Auto),
                4 => Ok(AutoExposureMode::ShutterPriority),
                8 => Ok(AutoExposureMode::AperturePriority),
                _ => Err(Error::Other),
            }
        }
    }

    pub fn set_ae_mode(&self, mode: AutoExposureMode) -> Result<()> {
        let mode_val: u8 = match mode {
            AutoExposureMode::Manual => 1,
            AutoExposureMode::Auto => 2,
            AutoExposureMode::ShutterPriority => 4,
            AutoExposureMode::AperturePriority => 8,
        };

        unsafe {
            let err = uvc_set_ae_mode(self.devh.as_ptr(), mode_val).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn ae_priority(&self) -> Result<AutoExposurePriority> {
        unsafe {
            let mut priority = std::mem::MaybeUninit::uninit();
            let err = uvc_get_ae_priority(
                self.devh.as_ptr(),
                priority.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }
            match priority.assume_init() {
                0 => Ok(AutoExposurePriority::Constant),
                1 => Ok(AutoExposurePriority::Variable),
                _ => Err(Error::Other),
            }
        }
    }

    pub fn exposure_abs(&self) -> Result<u32> {
        unsafe {
            let mut time = std::mem::MaybeUninit::uninit();
            let err = uvc_get_exposure_abs(
                self.devh.as_ptr(),
                time.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err == Error::Success {
                Ok(time.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_exposure_abs(&self, time: u32) -> Result<()> {
        unsafe {
            let err = uvc_set_exposure_abs(self.devh.as_ptr(), time).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn exposure_abs_range(&self) -> Result<Range<u32>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_exposure_abs(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_exposure_abs(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_exposure_abs(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_exposure_abs(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn exposure_rel(&self) -> Result<i8> {
        unsafe {
            let mut step = std::mem::MaybeUninit::uninit();
            let err = uvc_get_exposure_rel(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err == Error::Success {
                Ok(step.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn focus_abs(&self) -> Result<u16> {
        unsafe {
            let mut focus = std::mem::MaybeUninit::uninit();
            let err = uvc_get_focus_abs(
                self.devh.as_ptr(),
                focus.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err == Error::Success {
                Ok(focus.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn focus_rel(&self) -> Result<(i8, u8)> {
        unsafe {
            let mut focus_rel = std::mem::MaybeUninit::uninit();
            let mut speed = std::mem::MaybeUninit::uninit();
            let err = uvc_get_focus_rel(
                self.devh.as_ptr(),
                focus_rel.as_mut_ptr(),
                speed.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err == Error::Success {
                Ok((focus_rel.assume_init(), speed.assume_init()))
            } else {
                Err(err)
            }
        }
    }

    pub fn gain(&self) -> Result<u16> {
        unsafe {
            let mut gain = std::mem::MaybeUninit::uninit();
            let err = uvc_get_gain(
                self.devh.as_ptr(),
                gain.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(gain.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_gain(&self, gain: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_gain(self.devh.as_ptr(), gain).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn gain_range(&self) -> Result<Range<u16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            // We check the MIN call for success; if it fails, the control likely isn't supported.
            let err = uvc_get_gain(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_gain(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_gain(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_gain(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn backlight_compensation(&self) -> Result<bool> {
        unsafe {
            let mut comp = std::mem::MaybeUninit::uninit();
            let err = uvc_get_backlight_compensation(
                self.devh.as_ptr(),
                comp.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(comp.assume_init() != 0)
            } else {
                Err(err)
            }
        }
    }

    pub fn set_backlight_compensation(&self, comp: bool) -> Result<()> {
        let comp_val: u16 = if comp { 1 } else { 0 };

        unsafe {
            let err = uvc_set_backlight_compensation(self.devh.as_ptr(), comp_val).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn white_balance_temperature_auto(&self) -> Result<bool> {
        unsafe {
            let mut auto = std::mem::MaybeUninit::uninit();
            let err = uvc_get_white_balance_temperature_auto(
                self.devh.as_ptr(),
                auto.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(auto.assume_init() != 0)
            } else {
                Err(err)
            }
        }
    }

    pub fn set_white_balance_temperature_auto(&self, auto: bool) -> Result<()> {
        let auto_val: u8 = if auto { 1 } else { 0 };

        unsafe {
            let err = uvc_set_white_balance_temperature_auto(self.devh.as_ptr(), auto_val).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn white_balance_temperature(&self) -> Result<u16> {
        unsafe {
            let mut temp = std::mem::MaybeUninit::uninit();
            let err = uvc_get_white_balance_temperature(
                self.devh.as_ptr(),
                temp.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(temp.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_white_balance_temperature(&self, temp: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_white_balance_temperature(self.devh.as_ptr(), temp).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn white_balance_temperature_range(&self) -> Result<Range<u16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_white_balance_temperature(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_white_balance_temperature(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_white_balance_temperature(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_white_balance_temperature(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn sharpness(&self) -> Result<u16> {
        unsafe {
            let mut sharpness = std::mem::MaybeUninit::uninit();
            let err = uvc_get_sharpness(
                self.devh.as_ptr(),
                sharpness.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(sharpness.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_sharpness(&self, sharpness: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_sharpness(self.devh.as_ptr(), sharpness).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn sharpness_range(&self) -> Result<Range<u16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_sharpness(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_sharpness(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_sharpness(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_sharpness(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn contrast(&self) -> Result<u16> {
        unsafe {
            let mut contrast = std::mem::MaybeUninit::uninit();
            let err = uvc_get_contrast(
                self.devh.as_ptr(),
                contrast.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(contrast.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_contrast(&self, contrast: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_contrast(self.devh.as_ptr(), contrast).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn contrast_range(&self) -> Result<Range<u16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_contrast(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_contrast(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_contrast(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_contrast(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn saturation(&self) -> Result<u16> {
        unsafe {
            let mut saturation = std::mem::MaybeUninit::uninit();
            let err = uvc_get_saturation(
                self.devh.as_ptr(),
                saturation.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(saturation.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_saturation(&self, saturation: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_saturation(self.devh.as_ptr(), saturation).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn saturation_range(&self) -> Result<Range<u16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_saturation(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_saturation(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_saturation(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_saturation(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }

    pub fn gamma(&self) -> Result<u16> {
        unsafe {
            let mut gamma = std::mem::MaybeUninit::uninit();
            let err = uvc_get_gamma(
                self.devh.as_ptr(),
                gamma.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(gamma.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_gamma(&self, gamma: u16) -> Result<()> {
        unsafe {
            let err = uvc_set_gamma(self.devh.as_ptr(), gamma).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn brightness(&self) -> Result<i16> {
        unsafe {
            let mut brightness = std::mem::MaybeUninit::uninit();
            let err = uvc_get_brightness(
                self.devh.as_ptr(),
                brightness.as_mut_ptr(),
                uvc_req_code_UVC_GET_CUR,
            )
            .into();

            if err == Error::Success {
                Ok(brightness.assume_init())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_brightness(&self, brightness: i16) -> Result<()> {
        unsafe {
            let err = uvc_set_brightness(self.devh.as_ptr(), brightness).into();
            if err == Error::Success {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn brightness_range(&self) -> Result<Range<i16>> {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            let mut step = std::mem::MaybeUninit::uninit();
            let mut def = std::mem::MaybeUninit::uninit();

            let err = uvc_get_brightness(
                self.devh.as_ptr(),
                min.as_mut_ptr(),
                uvc_req_code_UVC_GET_MIN,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_brightness(
                self.devh.as_ptr(),
                max.as_mut_ptr(),
                uvc_req_code_UVC_GET_MAX,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_brightness(
                self.devh.as_ptr(),
                step.as_mut_ptr(),
                uvc_req_code_UVC_GET_RES,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            let err = uvc_get_brightness(
                self.devh.as_ptr(),
                def.as_mut_ptr(),
                uvc_req_code_UVC_GET_DEF,
            )
            .into();
            if err != Error::Success {
                return Err(err);
            }

            Ok(Range {
                min: min.assume_init(),
                max: max.assume_init(),
                step: step.assume_init(),
                default: def.assume_init(),
            })
        }
    }
}
