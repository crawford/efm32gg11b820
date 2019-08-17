#[doc = "Reader of register DOEP4_CTL"]
pub type R = crate::R<u32, super::DOEP4_CTL>;
#[doc = "Writer for register DOEP4_CTL"]
pub type W = crate::W<u32, super::DOEP4_CTL>;
#[doc = "Register DOEP4_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEP4_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPS`"]
pub type MPS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MPS`"]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `USBACTEP`"]
pub type USBACTEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBACTEP`"]
pub struct USBACTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBACTEP_W<'a> {
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
#[doc = "Reader of field `DPID`"]
pub type DPID_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKSTS`"]
pub type NAKSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTYPE`"]
pub type EPTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPTYPE`"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SNP`"]
pub type SNP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SNP`"]
pub struct SNP_W<'a> {
    w: &'a mut W,
}
impl<'a> SNP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `CNAK`"]
pub struct CNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `SNAK`"]
pub struct SNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAK_W<'a> {
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
#[doc = "Write proxy for field `SETD0PIDEF`"]
pub struct SETD0PIDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETD0PIDEF_W<'a> {
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
#[doc = "Write proxy for field `SETD1PIDOF`"]
pub struct SETD1PIDOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETD1PIDOF_W<'a> {
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
#[doc = "Reader of field `EPDIS`"]
pub type EPDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDIS`"]
pub struct EPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `EPENA`"]
pub type EPENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPENA`"]
pub struct EPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPENA_W<'a> {
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&mut self) -> USBACTEP_W {
        USBACTEP_W { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&mut self) -> SNP_W {
        SNP_W { w: self }
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    pub fn setd0pidef(&mut self) -> SETD0PIDEF_W {
        SETD0PIDEF_W { w: self }
    }
    #[doc = "Bit 29 - Set DATA1 PID"]
    #[inline(always)]
    pub fn setd1pidof(&mut self) -> SETD1PIDOF_W {
        SETD1PIDOF_W { w: self }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W { w: self }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W {
        EPENA_W { w: self }
    }
}
