#[doc = "Reader of register FREEZE"]
pub type R = crate::R<u32, super::FREEZE>;
#[doc = "Writer for register FREEZE"]
pub type W = crate::W<u32, super::FREEZE>;
#[doc = "Register FREEZE `reset()`'s with value 0"]
impl crate::ResetValue for super::FREEZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGFREEZE`"]
pub type REGFREEZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGFREEZE`"]
pub struct REGFREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> REGFREEZE_W<'a> {
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
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&self) -> REGFREEZE_R {
        REGFREEZE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Update Freeze"]
    #[inline(always)]
    pub fn regfreeze(&mut self) -> REGFREEZE_W {
        REGFREEZE_W { w: self }
    }
}
