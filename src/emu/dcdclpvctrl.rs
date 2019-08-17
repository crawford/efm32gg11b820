#[doc = "Reader of register DCDCLPVCTRL"]
pub type R = crate::R<u32, super::DCDCLPVCTRL>;
#[doc = "Writer for register DCDCLPVCTRL"]
pub type W = crate::W<u32, super::DCDCLPVCTRL>;
#[doc = "Register DCDCLPVCTRL `reset()`'s with value 0x0168"]
impl crate::ResetValue for super::DCDCLPVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0168
    }
}
#[doc = "Reader of field `LPATT`"]
pub type LPATT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPATT`"]
pub struct LPATT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPATT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LPVREF`"]
pub type LPVREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPVREF`"]
pub struct LPVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | (((value as u32) & 0xff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    pub fn lpatt(&self) -> LPATT_R {
        LPATT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpvref(&self) -> LPVREF_R {
        LPVREF_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    pub fn lpatt(&mut self) -> LPATT_W {
        LPATT_W { w: self }
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpvref(&mut self) -> LPVREF_W {
        LPVREF_W { w: self }
    }
}
