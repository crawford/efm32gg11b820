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
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFRCORDY`"]
pub struct HFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCORDY_W<'a> {
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
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXORDY`"]
pub struct HFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXORDY_W<'a> {
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
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFRCORDY`"]
pub struct LFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCORDY_W<'a> {
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
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXORDY`"]
pub struct LFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXORDY_W<'a> {
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
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXHFRCORDY`"]
pub struct AUXHFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCORDY_W<'a> {
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
#[doc = "Reader of field `CALRDY`"]
pub type CALRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALRDY`"]
pub struct CALRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRDY_W<'a> {
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
#[doc = "Reader of field `CALOF`"]
pub type CALOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALOF`"]
pub struct CALOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOF_W<'a> {
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
#[doc = "Reader of field `USHFRCORDY`"]
pub type USHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USHFRCORDY`"]
pub struct USHFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCORDY_W<'a> {
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
#[doc = "Reader of field `HFXODISERR`"]
pub type HFXODISERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXODISERR`"]
pub struct HFXODISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODISERR_W<'a> {
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
#[doc = "Reader of field `HFXOAUTOSW`"]
pub type HFXOAUTOSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXOAUTOSW`"]
pub struct HFXOAUTOSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOAUTOSW_W<'a> {
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
#[doc = "Reader of field `HFXOPEAKDETRDY`"]
pub type HFXOPEAKDETRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXOPEAKDETRDY`"]
pub struct HFXOPEAKDETRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOPEAKDETRDY_W<'a> {
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
#[doc = "Reader of field `HFRCODIS`"]
pub type HFRCODIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFRCODIS`"]
pub struct HFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCODIS_W<'a> {
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
#[doc = "Reader of field `LFTIMEOUTERR`"]
pub type LFTIMEOUTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFTIMEOUTERR`"]
pub struct LFTIMEOUTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTIMEOUTERR_W<'a> {
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
#[doc = "Reader of field `DPLLRDY`"]
pub type DPLLRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLRDY`"]
pub struct DPLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DPLLLOCKFAILLOW`"]
pub type DPLLLOCKFAILLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLOCKFAILLOW`"]
pub struct DPLLLOCKFAILLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLOCKFAILLOW_W<'a> {
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
#[doc = "Reader of field `DPLLLOCKFAILHIGH`"]
pub type DPLLLOCKFAILHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLOCKFAILHIGH`"]
pub struct DPLLLOCKFAILHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLOCKFAILHIGH_W<'a> {
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
#[doc = "Reader of field `LFXOEDGE`"]
pub type LFXOEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXOEDGE`"]
pub struct LFXOEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `LFRCOEDGE`"]
pub type LFRCOEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFRCOEDGE`"]
pub struct LFRCOEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCOEDGE_W<'a> {
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
#[doc = "Reader of field `ULFRCOEDGE`"]
pub type ULFRCOEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULFRCOEDGE`"]
pub struct ULFRCOEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ULFRCOEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CMUERR`"]
pub type CMUERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMUERR`"]
pub struct CMUERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DPLLLOCKFAILLOW_R {
        DPLLLOCKFAILLOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DPLLLOCKFAILHIGH_R {
        DPLLLOCKFAILHIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LFXOEDGE_R {
        LFXOEDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LFRCOEDGE_R {
        LFRCOEDGE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> ULFRCOEDGE_R {
        ULFRCOEDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W {
        HFRCORDY_W { w: self }
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HFXORDY_W {
        HFXORDY_W { w: self }
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W {
        LFRCORDY_W { w: self }
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LFXORDY_W {
        LFXORDY_W { w: self }
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W {
        AUXHFRCORDY_W { w: self }
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CALRDY_W {
        CALRDY_W { w: self }
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CALOF_W {
        CALOF_W { w: self }
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W {
        USHFRCORDY_W { w: self }
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W {
        HFXODISERR_W { w: self }
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W {
        HFXOAUTOSW_W { w: self }
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W {
        HFXOPEAKDETRDY_W { w: self }
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W { w: self }
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W {
        LFTIMEOUTERR_W { w: self }
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W {
        DPLLRDY_W { w: self }
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W {
        DPLLLOCKFAILLOW_W { w: self }
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W {
        DPLLLOCKFAILHIGH_W { w: self }
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W {
        LFXOEDGE_W { w: self }
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W {
        LFRCOEDGE_W { w: self }
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W {
        ULFRCOEDGE_W { w: self }
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CMUERR_W {
        CMUERR_W { w: self }
    }
}
