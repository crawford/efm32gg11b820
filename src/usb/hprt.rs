#[doc = "Reader of register HPRT"]
pub type R = crate::R<u32, super::HPRT>;
#[doc = "Writer for register HPRT"]
pub type W = crate::W<u32, super::HPRT>;
#[doc = "Register HPRT `reset()`'s with value 0"]
impl crate::ResetValue for super::HPRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRTCONNSTS`"]
pub type PRTCONNSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRTCONNDET`"]
pub type PRTCONNDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTCONNDET`"]
pub struct PRTCONNDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCONNDET_W<'a> {
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
#[doc = "Reader of field `PRTENA`"]
pub type PRTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTENA`"]
pub struct PRTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENA_W<'a> {
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
#[doc = "Reader of field `PRTENCHNG`"]
pub type PRTENCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTENCHNG`"]
pub struct PRTENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENCHNG_W<'a> {
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
#[doc = "Reader of field `PRTOVRCURRACT`"]
pub type PRTOVRCURRACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRTOVRCURRCHNG`"]
pub type PRTOVRCURRCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTOVRCURRCHNG`"]
pub struct PRTOVRCURRCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOVRCURRCHNG_W<'a> {
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
#[doc = "Reader of field `PRTRES`"]
pub type PRTRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTRES`"]
pub struct PRTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRES_W<'a> {
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
#[doc = "Reader of field `PRTSUSP`"]
pub type PRTSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTSUSP`"]
pub struct PRTSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTSUSP_W<'a> {
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
#[doc = "Reader of field `PRTRST`"]
pub type PRTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTRST`"]
pub struct PRTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRST_W<'a> {
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
#[doc = "Reader of field `PRTLNSTS`"]
pub type PRTLNSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRTPWR`"]
pub type PRTPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTPWR`"]
pub struct PRTPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTPWR_W<'a> {
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
#[doc = "Reader of field `PRTTSTCTL`"]
pub type PRTTSTCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRTTSTCTL`"]
pub struct PRTTSTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTTSTCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
#[doc = "Reader of field `PRTSPD`"]
pub type PRTSPD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W {
        PRTCONNDET_W { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&mut self) -> PRTENA_W {
        PRTENA_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W {
        PRTENCHNG_W { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W {
        PRTOVRCURRCHNG_W { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&mut self) -> PRTRES_W {
        PRTRES_W { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&mut self) -> PRTSUSP_W {
        PRTSUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&mut self) -> PRTRST_W {
        PRTRST_W { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&mut self) -> PRTPWR_W {
        PRTPWR_W { w: self }
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W {
        PRTTSTCTL_W { w: self }
    }
}
