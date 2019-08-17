#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Reader of field `CHOF`"]
pub type CHOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHOF`"]
pub struct CHOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOF_W<'a> {
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
#[doc = "Reader of field `CMOF`"]
pub type CMOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMOF`"]
pub struct CMOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMOF_W<'a> {
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
#[doc = "Reader of field `PWRUPF`"]
pub type PWRUPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRUPF`"]
pub struct PWRUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUPF_W<'a> {
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
#[doc = "Reader of field `ICACHERR`"]
pub type ICACHERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHERR`"]
pub struct ICACHERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHERR_W<'a> {
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
#[doc = "Reader of field `WDATAOV`"]
pub type WDATAOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDATAOV`"]
pub struct WDATAOV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATAOV_W<'a> {
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
#[doc = "Reader of field `LVEWRITE`"]
pub type LVEWRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVEWRITE`"]
pub struct LVEWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVEWRITE_W<'a> {
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
#[doc = "Reader of field `RAMERR1B`"]
pub type RAMERR1B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMERR1B`"]
pub struct RAMERR1B_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERR1B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RAMERR2B`"]
pub type RAMERR2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMERR2B`"]
pub struct RAMERR2B_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERR2B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RAM1ERR1B`"]
pub type RAM1ERR1B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1ERR1B`"]
pub struct RAM1ERR1B_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1ERR1B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RAM1ERR2B`"]
pub type RAM1ERR2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1ERR2B`"]
pub struct RAM1ERR2B_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1ERR2B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&self) -> WDATAOV_R {
        WDATAOV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LVEWRITE_R {
        LVEWRITE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAMERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> RAMERR1B_R {
        RAMERR1B_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RAMERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> RAMERR2B_R {
        RAMERR2B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RAM1ERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err1b(&self) -> RAM1ERR1B_R {
        RAM1ERR1B_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RAM1ERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err2b(&self) -> RAM1ERR2B_R {
        RAM1ERR2B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W { w: self }
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W { w: self }
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PWRUPF_W {
        PWRUPF_W { w: self }
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> ICACHERR_W {
        ICACHERR_W { w: self }
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WDATAOV_W {
        WDATAOV_W { w: self }
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LVEWRITE_W {
        LVEWRITE_W { w: self }
    }
    #[doc = "Bit 16 - RAMERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W {
        RAMERR1B_W { w: self }
    }
    #[doc = "Bit 17 - RAMERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W {
        RAMERR2B_W { w: self }
    }
    #[doc = "Bit 18 - RAM1ERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W {
        RAM1ERR1B_W { w: self }
    }
    #[doc = "Bit 19 - RAM1ERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W {
        RAM1ERR2B_W { w: self }
    }
}
