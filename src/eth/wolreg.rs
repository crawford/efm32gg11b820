#[doc = "Reader of register WOLREG"]
pub type R = crate::R<u32, super::WOLREG>;
#[doc = "Writer for register WOLREG"]
pub type W = crate::W<u32, super::WOLREG>;
#[doc = "Register WOLREG `reset()`'s with value 0"]
impl crate::ResetValue for super::WOLREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WOLMASK0`"]
pub type WOLMASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOLMASK0`"]
pub struct WOLMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> WOLMASK0_W<'a> {
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
#[doc = "Reader of field `WOLMASK1`"]
pub type WOLMASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOLMASK1`"]
pub struct WOLMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> WOLMASK1_W<'a> {
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
#[doc = "Reader of field `WOLMASK2`"]
pub type WOLMASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOLMASK2`"]
pub struct WOLMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> WOLMASK2_W<'a> {
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
#[doc = "Reader of field `WOLMASK3`"]
pub type WOLMASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOLMASK3`"]
pub struct WOLMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> WOLMASK3_W<'a> {
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
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&self) -> WOLMASK0_R {
        WOLMASK0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&self) -> WOLMASK1_R {
        WOLMASK1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&self) -> WOLMASK2_R {
        WOLMASK2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&self) -> WOLMASK3_R {
        WOLMASK3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&mut self) -> WOLMASK0_W {
        WOLMASK0_W { w: self }
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&mut self) -> WOLMASK1_W {
        WOLMASK1_W { w: self }
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&mut self) -> WOLMASK2_W {
        WOLMASK2_W { w: self }
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&mut self) -> WOLMASK3_W {
        WOLMASK3_W { w: self }
    }
}
