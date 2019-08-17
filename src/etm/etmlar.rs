#[doc = "Reader of register ETMLAR"]
pub type R = crate::R<u32, super::ETMLAR>;
#[doc = "Writer for register ETMLAR"]
pub type W = crate::W<u32, super::ETMLAR>;
#[doc = "Register ETMLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
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
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Value"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
