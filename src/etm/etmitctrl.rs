#[doc = "Reader of register ETMITCTRL"]
pub type R = crate::R<u32, super::ETMITCTRL>;
#[doc = "Writer for register ETMITCTRL"]
pub type W = crate::W<u32, super::ETMITCTRL>;
#[doc = "Register ETMITCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMITCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITEN`"]
pub type ITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITEN`"]
pub struct ITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITEN_W<'a> {
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
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&mut self) -> ITEN_W {
        ITEN_W { w: self }
    }
}
