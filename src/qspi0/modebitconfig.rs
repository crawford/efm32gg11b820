#[doc = "Reader of register MODEBITCONFIG"]
pub type R = crate::R<u32, super::MODEBITCONFIG>;
#[doc = "Writer for register MODEBITCONFIG"]
pub type W = crate::W<u32, super::MODEBITCONFIG>;
#[doc = "Register MODEBITCONFIG `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::MODEBITCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CHUNKSIZE`"]
pub type CHUNKSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHUNKSIZE`"]
pub struct CHUNKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHUNKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CRCOUTENABLE`"]
pub type CRCOUTENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCOUTENABLE`"]
pub struct CRCOUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCOUTENABLE_W<'a> {
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
#[doc = "Reader of field `RXCRCDATAUP`"]
pub type RXCRCDATAUP_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXCRCDATALOW`"]
pub type RXCRCDATALOW_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&self) -> CHUNKSIZE_R {
        CHUNKSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&self) -> CRCOUTENABLE_R {
        CRCOUTENABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - RX CRC Data (upper)"]
    #[inline(always)]
    pub fn rxcrcdataup(&self) -> RXCRCDATAUP_R {
        RXCRCDATAUP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX CRC Data (lower)"]
    #[inline(always)]
    pub fn rxcrcdatalow(&self) -> RXCRCDATALOW_R {
        RXCRCDATALOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&mut self) -> CHUNKSIZE_W {
        CHUNKSIZE_W { w: self }
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&mut self) -> CRCOUTENABLE_W {
        CRCOUTENABLE_W { w: self }
    }
}
