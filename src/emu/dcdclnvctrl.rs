#[doc = "Reader of register DCDCLNVCTRL"]
pub type R = crate::R<u32, super::DCDCLNVCTRL>;
#[doc = "Writer for register DCDCLNVCTRL"]
pub type W = crate::W<u32, super::DCDCLNVCTRL>;
#[doc = "Register DCDCLNVCTRL `reset()`'s with value 0x7100"]
impl crate::ResetValue for super::DCDCLNVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7100
    }
}
#[doc = "Reader of field `LNATT`"]
pub type LNATT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNATT`"]
pub struct LNATT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNATT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LNVREF`"]
pub type LNVREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNVREF`"]
pub struct LNVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LNVREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    pub fn lnatt(&self) -> LNATT_R {
        LNATT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    pub fn lnvref(&self) -> LNVREF_R {
        LNVREF_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    pub fn lnatt(&mut self) -> LNATT_W {
        LNATT_W { w: self }
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    pub fn lnvref(&mut self) -> LNVREF_W {
        LNVREF_W { w: self }
    }
}
