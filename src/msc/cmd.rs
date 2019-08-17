#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PWRUP`"]
pub struct PWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_W<'a> {
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
#[doc = "Write proxy for field `SWITCHINGBANK`"]
pub struct SWITCHINGBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCHINGBANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Flash Power Up Command"]
    #[inline(always)]
    pub fn pwrup(&mut self) -> PWRUP_W {
        PWRUP_W { w: self }
    }
    #[doc = "Bit 1 - BANK SWITCHING COMMAND"]
    #[inline(always)]
    pub fn switchingbank(&mut self) -> SWITCHINGBANK_W {
        SWITCHINGBANK_W { w: self }
    }
}
