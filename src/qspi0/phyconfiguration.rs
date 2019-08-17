#[doc = "Reader of register PHYCONFIGURATION"]
pub type R = crate::R<u32, super::PHYCONFIGURATION>;
#[doc = "Writer for register PHYCONFIGURATION"]
pub type W = crate::W<u32, super::PHYCONFIGURATION>;
#[doc = "Register PHYCONFIGURATION `reset()`'s with value 0"]
impl crate::ResetValue for super::PHYCONFIGURATION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHYCONFIGRXDLLDELAY`"]
pub type PHYCONFIGRXDLLDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYCONFIGRXDLLDELAY`"]
pub struct PHYCONFIGRXDLLDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCONFIGRXDLLDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PHYCONFIGTXDLLDELAY`"]
pub type PHYCONFIGTXDLLDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYCONFIGTXDLLDELAY`"]
pub struct PHYCONFIGTXDLLDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCONFIGTXDLLDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `PHYCONFIGRESYNC`"]
pub struct PHYCONFIGRESYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCONFIGRESYNC_W<'a> {
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
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&self) -> PHYCONFIGRXDLLDELAY_R {
        PHYCONFIGRXDLLDELAY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&self) -> PHYCONFIGTXDLLDELAY_R {
        PHYCONFIGTXDLLDELAY_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&mut self) -> PHYCONFIGRXDLLDELAY_W {
        PHYCONFIGRXDLLDELAY_W { w: self }
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&mut self) -> PHYCONFIGTXDLLDELAY_W {
        PHYCONFIGTXDLLDELAY_W { w: self }
    }
    #[doc = "Bit 31 - PHY Config Resync"]
    #[inline(always)]
    pub fn phyconfigresync(&mut self) -> PHYCONFIGRESYNC_W {
        PHYCONFIGRESYNC_W { w: self }
    }
}
