#[doc = "Reader of register SEQCTRL"]
pub type R = crate::R<u32, super::SEQCTRL>;
#[doc = "Writer for register SEQCTRL"]
pub type W = crate::W<u32, super::SEQCTRL>;
#[doc = "Register SEQCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENGTHA`"]
pub type LENGTHA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LENGTHA`"]
pub struct LENGTHA_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Size of Data Blocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLOCKSIZE_A {
    #[doc = "0: A block is 16 bytes long"]
    _16BYTES = 0,
    #[doc = "1: A block is 32 bytes long"]
    _32BYTES = 1,
    #[doc = "2: A block is 64 bytes long"]
    _64BYTES = 2,
}
impl From<BLOCKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLOCKSIZE`"]
pub type BLOCKSIZE_R = crate::R<u8, BLOCKSIZE_A>;
impl BLOCKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLOCKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLOCKSIZE_A::_16BYTES),
            1 => Val(BLOCKSIZE_A::_32BYTES),
            2 => Val(BLOCKSIZE_A::_64BYTES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16BYTES`"]
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_16BYTES
    }
    #[doc = "Checks if the value of the field is `_32BYTES`"]
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_32BYTES
    }
    #[doc = "Checks if the value of the field is `_64BYTES`"]
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_64BYTES
    }
}
#[doc = "Write proxy for field `BLOCKSIZE`"]
pub struct BLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A block is 16 bytes long"]
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_16BYTES)
    }
    #[doc = "A block is 32 bytes long"]
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_32BYTES)
    }
    #[doc = "A block is 64 bytes long"]
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_64BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DMA0SKIP`"]
pub type DMA0SKIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA0SKIP`"]
pub struct DMA0SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DMA1SKIP`"]
pub type DMA1SKIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA1SKIP`"]
pub struct DMA1SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DMA0PRESA`"]
pub type DMA0PRESA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA0PRESA`"]
pub struct DMA0PRESA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0PRESA_W<'a> {
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
#[doc = "Reader of field `DMA1PRESA`"]
pub type DMA1PRESA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1PRESA`"]
pub struct DMA1PRESA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1PRESA_W<'a> {
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
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
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
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    pub fn lengtha(&self) -> LENGTHA_R {
        LENGTHA_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    pub fn dma0skip(&self) -> DMA0SKIP_R {
        DMA0SKIP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    pub fn dma1skip(&self) -> DMA1SKIP_R {
        DMA1SKIP_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    pub fn dma0presa(&self) -> DMA0PRESA_R {
        DMA0PRESA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    pub fn dma1presa(&self) -> DMA1PRESA_R {
        DMA1PRESA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    pub fn lengtha(&mut self) -> LENGTHA_W {
        LENGTHA_W { w: self }
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W {
        BLOCKSIZE_W { w: self }
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    pub fn dma0skip(&mut self) -> DMA0SKIP_W {
        DMA0SKIP_W { w: self }
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    pub fn dma1skip(&mut self) -> DMA1SKIP_W {
        DMA1SKIP_W { w: self }
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    pub fn dma0presa(&mut self) -> DMA0PRESA_W {
        DMA0PRESA_W { w: self }
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    pub fn dma1presa(&mut self) -> DMA1PRESA_W {
        DMA1PRESA_W { w: self }
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
}
