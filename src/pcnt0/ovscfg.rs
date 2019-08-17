#[doc = "Reader of register OVSCFG"]
pub type R = crate::R<u32, super::OVSCFG>;
#[doc = "Writer for register OVSCFG"]
pub type W = crate::W<u32, super::OVSCFG>;
#[doc = "Register OVSCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::OVSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILTLEN`"]
pub type FILTLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTLEN`"]
pub struct FILTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FLUTTERRM`"]
pub type FLUTTERRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUTTERRM`"]
pub struct FLUTTERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUTTERRM_W<'a> {
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
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FLUTTERRM_R {
        FLUTTERRM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&mut self) -> FILTLEN_W {
        FILTLEN_W { w: self }
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&mut self) -> FLUTTERRM_W {
        FLUTTERRM_W { w: self }
    }
}
