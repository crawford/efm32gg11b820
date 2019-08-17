#[doc = "Reader of register DEVINSTRWRCONFIG"]
pub type R = crate::R<u32, super::DEVINSTRWRCONFIG>;
#[doc = "Writer for register DEVINSTRWRCONFIG"]
pub type W = crate::W<u32, super::DEVINSTRWRCONFIG>;
#[doc = "Register DEVINSTRWRCONFIG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::DEVINSTRWRCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `WROPCODE`"]
pub type WROPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WROPCODE`"]
pub struct WROPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WROPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WELDIS`"]
pub type WELDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WELDIS`"]
pub struct WELDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WELDIS_W<'a> {
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
#[doc = "Reader of field `ADDRXFERTYPESTDMODE`"]
pub type ADDRXFERTYPESTDMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRXFERTYPESTDMODE`"]
pub struct ADDRXFERTYPESTDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRXFERTYPESTDMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DATAXFERTYPEEXTMODE`"]
pub type DATAXFERTYPEEXTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAXFERTYPEEXTMODE`"]
pub struct DATAXFERTYPEEXTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAXFERTYPEEXTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DUMMYWRCLKCYCLES`"]
pub type DUMMYWRCLKCYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUMMYWRCLKCYCLES`"]
pub struct DUMMYWRCLKCYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMYWRCLKCYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&self) -> WROPCODE_R {
        WROPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&self) -> WELDIS_R {
        WELDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&self) -> ADDRXFERTYPESTDMODE_R {
        ADDRXFERTYPESTDMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&self) -> DATAXFERTYPEEXTMODE_R {
        DATAXFERTYPEEXTMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&self) -> DUMMYWRCLKCYCLES_R {
        DUMMYWRCLKCYCLES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&mut self) -> WROPCODE_W {
        WROPCODE_W { w: self }
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&mut self) -> WELDIS_W {
        WELDIS_W { w: self }
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&mut self) -> ADDRXFERTYPESTDMODE_W {
        ADDRXFERTYPESTDMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&mut self) -> DATAXFERTYPEEXTMODE_W {
        DATAXFERTYPEEXTMODE_W { w: self }
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&mut self) -> DUMMYWRCLKCYCLES_W {
        DUMMYWRCLKCYCLES_W { w: self }
    }
}
