#[doc = "Reader of register POWERDOWN"]
pub type R = crate::R<u32, super::POWERDOWN>;
#[doc = "Writer for register POWERDOWN"]
pub type W = crate::W<u32, super::POWERDOWN>;
#[doc = "Register POWERDOWN `reset()`'s with value 0"]
impl crate::ResetValue for super::POWERDOWN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM`"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
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
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
}
