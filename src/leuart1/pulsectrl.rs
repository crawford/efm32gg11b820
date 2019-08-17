#[doc = "Reader of register PULSECTRL"]
pub type R = crate::R<u32, super::PULSECTRL>;
#[doc = "Writer for register PULSECTRL"]
pub type W = crate::W<u32, super::PULSECTRL>;
#[doc = "Register PULSECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PULSECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PULSEW`"]
pub type PULSEW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PULSEW`"]
pub struct PULSEW_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PULSEEN`"]
pub type PULSEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULSEEN`"]
pub struct PULSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PULSEFILT`"]
pub type PULSEFILT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULSEFILT`"]
pub struct PULSEFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEFILT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PULSEW_R {
        PULSEW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PULSEEN_R {
        PULSEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PULSEFILT_R {
        PULSEFILT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&mut self) -> PULSEW_W {
        PULSEW_W { w: self }
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&mut self) -> PULSEEN_W {
        PULSEEN_W { w: self }
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&mut self) -> PULSEFILT_W {
        PULSEFILT_W { w: self }
    }
}
