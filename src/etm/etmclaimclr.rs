#[doc = "Reader of register ETMCLAIMCLR"]
pub type R = crate::R<u32, super::ETMCLAIMCLR>;
#[doc = "Writer for register ETMCLAIMCLR"]
pub type W = crate::W<u32, super::ETMCLAIMCLR>;
#[doc = "Register ETMCLAIMCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMCLAIMCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRTAG`"]
pub type CLRTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRTAG`"]
pub struct CLRTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRTAG_W<'a> {
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
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&self) -> CLRTAG_R {
        CLRTAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&mut self) -> CLRTAG_W {
        CLRTAG_W { w: self }
    }
}
