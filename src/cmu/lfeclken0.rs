#[doc = "Reader of register LFECLKEN0"]
pub type R = crate::R<u32, super::LFECLKEN0>;
#[doc = "Writer for register LFECLKEN0"]
pub type W = crate::W<u32, super::LFECLKEN0>;
#[doc = "Register LFECLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFECLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCC`"]
pub struct RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W { w: self }
    }
}
