#[doc = "Reader of register HC12_CHAR"]
pub type R = crate::R<u32, super::HC12_CHAR>;
#[doc = "Writer for register HC12_CHAR"]
pub type W = crate::W<u32, super::HC12_CHAR>;
#[doc = "Register HC12_CHAR `reset()`'s with value 0"]
impl crate::ResetValue for super::HC12_CHAR {
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
#[doc = "Reader of field `EPNUM`"]
pub type EPNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPNUM`"]
pub struct EPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `EPDIR`"]
pub type EPDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDIR`"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
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
#[doc = "Reader of field `LSPDDEV`"]
pub type LSPDDEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSPDDEV`"]
pub struct LSPDDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPDDEV_W<'a> {
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
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x7f << 22)) | (((value as u32) & 0x7f) << 22);
        self.w
    }
}
#[doc = "Reader of field `ODDFRM`"]
pub type ODDFRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODDFRM`"]
pub struct ODDFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ODDFRM_W<'a> {
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
#[doc = "Reader of field `CHDIS`"]
pub type CHDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHDIS`"]
pub struct CHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIS_W<'a> {
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
#[doc = "Reader of field `CHENA`"]
pub type CHENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHENA`"]
pub struct CHENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CHENA_W<'a> {
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
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&mut self) -> LSPDDEV_W {
        LSPDDEV_W { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W {
        ODDFRM_W { w: self }
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W {
        CHDIS_W { w: self }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W {
        CHENA_W { w: self }
    }
}
