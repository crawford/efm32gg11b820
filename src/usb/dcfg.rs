#[doc = "Reader of register DCFG"]
pub type R = crate::R<u32, super::DCFG>;
#[doc = "Writer for register DCFG"]
pub type W = crate::W<u32, super::DCFG>;
#[doc = "Register DCFG `reset()`'s with value 0x0800_0000"]
impl crate::ResetValue for super::DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0000
    }
}
#[doc = "Reader of field `DEVSPD`"]
pub type DEVSPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVSPD`"]
pub struct DEVSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NZSTSOUTHSHK`"]
pub type NZSTSOUTHSHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NZSTSOUTHSHK`"]
pub struct NZSTSOUTHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZSTSOUTHSHK_W<'a> {
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
#[doc = "Reader of field `ENA32KHZSUSP`"]
pub type ENA32KHZSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA32KHZSUSP`"]
pub struct ENA32KHZSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA32KHZSUSP_W<'a> {
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
#[doc = "Reader of field `DEVADDR`"]
pub type DEVADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVADDR`"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PERFRINT`"]
pub type PERFRINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERFRINT`"]
pub struct PERFRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFRINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `ENDEVOUTNAK`"]
pub type ENDEVOUTNAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDEVOUTNAK`"]
pub struct ENDEVOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEVOUTNAK_W<'a> {
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
#[doc = "Reader of field `XCVRDLY`"]
pub type XCVRDLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCVRDLY`"]
pub struct XCVRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRDLY_W<'a> {
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
#[doc = "Reader of field `ERRATICINTMSK`"]
pub type ERRATICINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRATICINTMSK`"]
pub struct ERRATICINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRATICINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSP_R {
        ENA32KHZSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Enable Device OUT NAK"]
    #[inline(always)]
    pub fn endevoutnak(&self) -> ENDEVOUTNAK_R {
        ENDEVOUTNAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DEVSPD_W {
        DEVSPD_W { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W {
        NZSTSOUTHSHK_W { w: self }
    }
    #[doc = "Bit 3 - Enable 32 kHz Suspend Mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> ENA32KHZSUSP_W {
        ENA32KHZSUSP_W { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PERFRINT_W {
        PERFRINT_W { w: self }
    }
    #[doc = "Bit 13 - Enable Device OUT NAK"]
    #[inline(always)]
    pub fn endevoutnak(&mut self) -> ENDEVOUTNAK_W {
        ENDEVOUTNAK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W {
        XCVRDLY_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W {
        ERRATICINTMSK_W { w: self }
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W {
        RESVALID_W { w: self }
    }
}
