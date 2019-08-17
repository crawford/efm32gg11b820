#[doc = "Writer for register WRITECMD"]
pub type W = crate::W<u32, super::WRITECMD>;
#[doc = "Register WRITECMD `reset()`'s with value 0"]
impl crate::ResetValue for super::WRITECMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LADDRIM`"]
pub struct LADDRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LADDRIM_W<'a> {
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
#[doc = "Write proxy for field `ERASEPAGE`"]
pub struct ERASEPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPAGE_W<'a> {
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
#[doc = "Write proxy for field `WRITEEND`"]
pub struct WRITEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEEND_W<'a> {
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
#[doc = "Write proxy for field `WRITEONCE`"]
pub struct WRITEONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEONCE_W<'a> {
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
#[doc = "Write proxy for field `WRITETRIG`"]
pub struct WRITETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITETRIG_W<'a> {
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
#[doc = "Write proxy for field `ERASEABORT`"]
pub struct ERASEABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEABORT_W<'a> {
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
#[doc = "Write proxy for field `ERASEMAIN0`"]
pub struct ERASEMAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEMAIN0_W<'a> {
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
#[doc = "Write proxy for field `ERASEMAIN1`"]
pub struct ERASEMAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEMAIN1_W<'a> {
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
#[doc = "Write proxy for field `CLEARWDATA`"]
pub struct CLEARWDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEARWDATA_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB Into ADDR"]
    #[inline(always)]
    pub fn laddrim(&mut self) -> LADDRIM_W {
        LADDRIM_W { w: self }
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ERASEPAGE_W {
        ERASEPAGE_W { w: self }
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    pub fn writeend(&mut self) -> WRITEEND_W {
        WRITEEND_W { w: self }
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    pub fn writeonce(&mut self) -> WRITEONCE_W {
        WRITEONCE_W { w: self }
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    pub fn writetrig(&mut self) -> WRITETRIG_W {
        WRITETRIG_W { w: self }
    }
    #[doc = "Bit 5 - Abort Erase Sequence"]
    #[inline(always)]
    pub fn eraseabort(&mut self) -> ERASEABORT_W {
        ERASEABORT_W { w: self }
    }
    #[doc = "Bit 8 - Mass Erase Region 0"]
    #[inline(always)]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W {
        ERASEMAIN0_W { w: self }
    }
    #[doc = "Bit 9 - Mass Erase Region 1"]
    #[inline(always)]
    pub fn erasemain1(&mut self) -> ERASEMAIN1_W {
        ERASEMAIN1_W { w: self }
    }
    #[doc = "Bit 12 - Clear WDATA State"]
    #[inline(always)]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W {
        CLEARWDATA_W { w: self }
    }
}
