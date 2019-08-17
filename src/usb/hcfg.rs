#[doc = "Reader of register HCFG"]
pub type R = crate::R<u32, super::HCFG>;
#[doc = "Writer for register HCFG"]
pub type W = crate::W<u32, super::HCFG>;
#[doc = "Register HCFG `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::HCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `FSLSPCLKSEL`"]
pub type FSLSPCLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSLSPCLKSEL`"]
pub struct FSLSPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FSLSSUPP`"]
pub type FSLSSUPP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSLSSUPP`"]
pub struct FSLSSUPP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSSUPP_W<'a> {
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
#[doc = "Reader of field `ENA32KHZS`"]
pub type ENA32KHZS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA32KHZS`"]
pub struct ENA32KHZS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA32KHZS_W<'a> {
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
#[doc = "Reader of field `RESVALID`"]
pub type RESVALID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESVALID`"]
pub struct RESVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MODECHTIMEN`"]
pub type MODECHTIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODECHTIMEN`"]
pub struct MODECHTIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODECHTIMEN_W<'a> {
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
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzs(&self) -> ENA32KHZS_R {
        ENA32KHZS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn modechtimen(&self) -> MODECHTIMEN_R {
        MODECHTIMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W {
        FSLSPCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W {
        FSLSSUPP_W { w: self }
    }
    #[doc = "Bit 7 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzs(&mut self) -> ENA32KHZS_W {
        ENA32KHZS_W { w: self }
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W {
        RESVALID_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn modechtimen(&mut self) -> MODECHTIMEN_W {
        MODECHTIMEN_W { w: self }
    }
}
