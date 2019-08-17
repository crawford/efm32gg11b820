#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTPEN`"]
pub type OUTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPEN`"]
pub struct OUTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPEN_W<'a> {
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
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn outpen(&self) -> OUTPEN_R {
        OUTPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn outpen(&mut self) -> OUTPEN_W {
        OUTPEN_W { w: self }
    }
}
