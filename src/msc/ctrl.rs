#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x21"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x21
    }
}
#[doc = "Reader of field `ADDRFAULTEN`"]
pub type ADDRFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDRFAULTEN`"]
pub struct ADDRFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRFAULTEN_W<'a> {
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
#[doc = "Reader of field `CLKDISFAULTEN`"]
pub type CLKDISFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKDISFAULTEN`"]
pub struct CLKDISFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDISFAULTEN_W<'a> {
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
#[doc = "Reader of field `PWRUPONDEMAND`"]
pub type PWRUPONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRUPONDEMAND`"]
pub struct PWRUPONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUPONDEMAND_W<'a> {
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
#[doc = "Reader of field `IFCREADCLEAR`"]
pub type IFCREADCLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFCREADCLEAR`"]
pub struct IFCREADCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCREADCLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIMEOUTFAULTEN`"]
pub type TIMEOUTFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMEOUTFAULTEN`"]
pub struct TIMEOUTFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTFAULTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RAMECCERRFAULTEN`"]
pub type RAMECCERRFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMECCERRFAULTEN`"]
pub struct RAMECCERRFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMECCERRFAULTEN_W<'a> {
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
#[doc = "Reader of field `EBIFAULTEN`"]
pub type EBIFAULTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIFAULTEN`"]
pub struct EBIFAULTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIFAULTEN_W<'a> {
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
#[doc = "Reader of field `WAITMODE`"]
pub type WAITMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITMODE`"]
pub struct WAITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITMODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PWRUPONDEMAND_R {
        PWRUPONDEMAND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IFCREADCLEAR_R {
        IFCREADCLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&self) -> TIMEOUTFAULTEN_R {
        TIMEOUTFAULTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    pub fn rameccerrfaulten(&self) -> RAMECCERRFAULTEN_R {
        RAMECCERRFAULTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    pub fn ebifaulten(&self) -> EBIFAULTEN_R {
        EBIFAULTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    pub fn waitmode(&self) -> WAITMODE_R {
        WAITMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W {
        ADDRFAULTEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W {
        CLKDISFAULTEN_W { w: self }
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&mut self) -> PWRUPONDEMAND_W {
        PWRUPONDEMAND_W { w: self }
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&mut self) -> IFCREADCLEAR_W {
        IFCREADCLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&mut self) -> TIMEOUTFAULTEN_W {
        TIMEOUTFAULTEN_W { w: self }
    }
    #[doc = "Bit 5 - Two Bit ECC Error Bus Fault Response Enable"]
    #[inline(always)]
    pub fn rameccerrfaulten(&mut self) -> RAMECCERRFAULTEN_W {
        RAMECCERRFAULTEN_W { w: self }
    }
    #[doc = "Bit 6 - EBI Bus Fault Response Enable"]
    #[inline(always)]
    pub fn ebifaulten(&mut self) -> EBIFAULTEN_W {
        EBIFAULTEN_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral Access Wait Mode"]
    #[inline(always)]
    pub fn waitmode(&mut self) -> WAITMODE_W {
        WAITMODE_W { w: self }
    }
}
