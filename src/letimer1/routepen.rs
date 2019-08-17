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
#[doc = "Reader of field `OUT0PEN`"]
pub type OUT0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT0PEN`"]
pub struct OUT0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0PEN_W<'a> {
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
#[doc = "Reader of field `OUT1PEN`"]
pub type OUT1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT1PEN`"]
pub struct OUT1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1PEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&self) -> OUT0PEN_R {
        OUT0PEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&self) -> OUT1PEN_R {
        OUT1PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> OUT0PEN_W {
        OUT0PEN_W { w: self }
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> OUT1PEN_W {
        OUT1PEN_W { w: self }
    }
}
