#[doc = "Reader of register ITTRIGOUT"]
pub type R = crate::R<u32, super::ITTRIGOUT>;
#[doc = "Writer for register ITTRIGOUT"]
pub type W = crate::W<u32, super::ITTRIGOUT>;
#[doc = "Register ITTRIGOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::ITTRIGOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIGGEROUT`"]
pub type TRIGGEROUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIGGEROUT`"]
pub struct TRIGGEROUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGEROUT_W<'a> {
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
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&self) -> TRIGGEROUT_R {
        TRIGGEROUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&mut self) -> TRIGGEROUT_W {
        TRIGGEROUT_W { w: self }
    }
}
