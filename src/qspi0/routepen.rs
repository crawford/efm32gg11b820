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
#[doc = "Reader of field `SCLKPEN`"]
pub type SCLKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLKPEN`"]
pub struct SCLKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKPEN_W<'a> {
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
#[doc = "Reader of field `CS0PEN`"]
pub type CS0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS0PEN`"]
pub struct CS0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0PEN_W<'a> {
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
#[doc = "Reader of field `CS1PEN`"]
pub type CS1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1PEN`"]
pub struct CS1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DQ0PEN`"]
pub type DQ0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ0PEN`"]
pub struct DQ0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ0PEN_W<'a> {
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
#[doc = "Reader of field `DQ1PEN`"]
pub type DQ1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ1PEN`"]
pub struct DQ1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ1PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DQ2PEN`"]
pub type DQ2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ2PEN`"]
pub struct DQ2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ2PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DQ3PEN`"]
pub type DQ3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ3PEN`"]
pub struct DQ3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ3PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DQ4PEN`"]
pub type DQ4PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ4PEN`"]
pub struct DQ4PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ4PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DQ5PEN`"]
pub type DQ5PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ5PEN`"]
pub struct DQ5PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ5PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DQ6PEN`"]
pub type DQ6PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ6PEN`"]
pub struct DQ6PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ6PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DQ7PEN`"]
pub type DQ7PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQ7PEN`"]
pub struct DQ7PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQ7PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DQSPEN`"]
pub type DQSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSPEN`"]
pub struct DQSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SCLKINPEN`"]
pub type SCLKINPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLKINPEN`"]
pub struct SCLKINPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKINPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    pub fn sclkpen(&self) -> SCLKPEN_R {
        SCLKPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    pub fn dq0pen(&self) -> DQ0PEN_R {
        DQ0PEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    pub fn dq1pen(&self) -> DQ1PEN_R {
        DQ1PEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    pub fn dq2pen(&self) -> DQ2PEN_R {
        DQ2PEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    pub fn dq3pen(&self) -> DQ3PEN_R {
        DQ3PEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    pub fn dq4pen(&self) -> DQ4PEN_R {
        DQ4PEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    pub fn dq5pen(&self) -> DQ5PEN_R {
        DQ5PEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    pub fn dq6pen(&self) -> DQ6PEN_R {
        DQ6PEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    pub fn dq7pen(&self) -> DQ7PEN_R {
        DQ7PEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    pub fn dqspen(&self) -> DQSPEN_R {
        DQSPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    pub fn sclkinpen(&self) -> SCLKINPEN_R {
        SCLKINPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    pub fn sclkpen(&mut self) -> SCLKPEN_W {
        SCLKPEN_W { w: self }
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&mut self) -> CS0PEN_W {
        CS0PEN_W { w: self }
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&mut self) -> CS1PEN_W {
        CS1PEN_W { w: self }
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    pub fn dq0pen(&mut self) -> DQ0PEN_W {
        DQ0PEN_W { w: self }
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    pub fn dq1pen(&mut self) -> DQ1PEN_W {
        DQ1PEN_W { w: self }
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    pub fn dq2pen(&mut self) -> DQ2PEN_W {
        DQ2PEN_W { w: self }
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    pub fn dq3pen(&mut self) -> DQ3PEN_W {
        DQ3PEN_W { w: self }
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    pub fn dq4pen(&mut self) -> DQ4PEN_W {
        DQ4PEN_W { w: self }
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    pub fn dq5pen(&mut self) -> DQ5PEN_W {
        DQ5PEN_W { w: self }
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    pub fn dq6pen(&mut self) -> DQ6PEN_W {
        DQ6PEN_W { w: self }
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    pub fn dq7pen(&mut self) -> DQ7PEN_W {
        DQ7PEN_W { w: self }
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    pub fn dqspen(&mut self) -> DQSPEN_W {
        DQSPEN_W { w: self }
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    pub fn sclkinpen(&mut self) -> SCLKINPEN_W {
        SCLKINPEN_W { w: self }
    }
}
