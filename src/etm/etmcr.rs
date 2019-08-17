#[doc = "Reader of register ETMCR"]
pub type R = crate::R<u32, super::ETMCR>;
#[doc = "Writer for register ETMCR"]
pub type W = crate::W<u32, super::ETMCR>;
#[doc = "Register ETMCR `reset()`'s with value 0x0411"]
impl crate::ResetValue for super::ETMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0411
    }
}
#[doc = "Reader of field `POWERDWN`"]
pub type POWERDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWERDWN`"]
pub struct POWERDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERDWN_W<'a> {
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
#[doc = "Reader of field `PORTSIZE`"]
pub type PORTSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORTSIZE`"]
pub struct PORTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Reader of field `BRANCHOUTPUT`"]
pub type BRANCHOUTPUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRANCHOUTPUT`"]
pub struct BRANCHOUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRANCHOUTPUT_W<'a> {
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
#[doc = "Reader of field `DBGREQCTRL`"]
pub type DBGREQCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGREQCTRL`"]
pub struct DBGREQCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGREQCTRL_W<'a> {
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
#[doc = "Reader of field `ETMPROG`"]
pub type ETMPROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETMPROG`"]
pub struct ETMPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPROG_W<'a> {
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
#[doc = "Reader of field `ETMPORTSEL`"]
pub type ETMPORTSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETMPORTSEL`"]
pub struct ETMPORTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPORTSEL_W<'a> {
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
#[doc = "Reader of field `PORTMODE2`"]
pub type PORTMODE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTMODE2`"]
pub struct PORTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE2_W<'a> {
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
#[doc = "Reader of field `PORTMODE`"]
pub type PORTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORTMODE`"]
pub struct PORTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `EPORTSIZE`"]
pub type EPORTSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPORTSIZE`"]
pub struct EPORTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPORTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `TSTAMPEN`"]
pub type TSTAMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTAMPEN`"]
pub struct TSTAMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTAMPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&self) -> POWERDWN_R {
        POWERDWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&self) -> BRANCHOUTPUT_R {
        BRANCHOUTPUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&self) -> DBGREQCTRL_R {
        DBGREQCTRL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&self) -> ETMPROG_R {
        ETMPROG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&self) -> ETMPORTSEL_R {
        ETMPORTSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&self) -> EPORTSIZE_R {
        EPORTSIZE_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&self) -> TSTAMPEN_R {
        TSTAMPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Control in low power mode"]
    #[inline(always)]
    pub fn powerdwn(&mut self) -> POWERDWN_W {
        POWERDWN_W { w: self }
    }
    #[doc = "Bits 4:6 - ETM Port Size"]
    #[inline(always)]
    pub fn portsize(&mut self) -> PORTSIZE_W {
        PORTSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn branchoutput(&mut self) -> BRANCHOUTPUT_W {
        BRANCHOUTPUT_W { w: self }
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgreqctrl(&mut self) -> DBGREQCTRL_W {
        DBGREQCTRL_W { w: self }
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn etmprog(&mut self) -> ETMPROG_W {
        ETMPROG_W { w: self }
    }
    #[doc = "Bit 11 - ETM Port Selection"]
    #[inline(always)]
    pub fn etmportsel(&mut self) -> ETMPORTSEL_W {
        ETMPORTSEL_W { w: self }
    }
    #[doc = "Bit 13 - Port Mode\\[2\\]"]
    #[inline(always)]
    pub fn portmode2(&mut self) -> PORTMODE2_W {
        PORTMODE2_W { w: self }
    }
    #[doc = "Bits 16:17 - Port Mode Control"]
    #[inline(always)]
    pub fn portmode(&mut self) -> PORTMODE_W {
        PORTMODE_W { w: self }
    }
    #[doc = "Bits 21:22 - Port Size\\[3\\]"]
    #[inline(always)]
    pub fn eportsize(&mut self) -> EPORTSIZE_W {
        EPORTSIZE_W { w: self }
    }
    #[doc = "Bit 28 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tstampen(&mut self) -> TSTAMPEN_W {
        TSTAMPEN_W { w: self }
    }
}
