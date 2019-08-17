#[doc = "Reader of register READCTRL"]
pub type R = crate::R<u32, super::READCTRL>;
#[doc = "Writer for register READCTRL"]
pub type W = crate::W<u32, super::READCTRL>;
#[doc = "Register READCTRL `reset()`'s with value 0x0100_0100"]
impl crate::ResetValue for super::READCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0100
    }
}
#[doc = "Reader of field `IFCDIS`"]
pub type IFCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFCDIS`"]
pub struct IFCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCDIS_W<'a> {
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
#[doc = "Reader of field `AIDIS`"]
pub type AIDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIDIS`"]
pub struct AIDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ICCDIS`"]
pub type ICCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICCDIS`"]
pub struct ICCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EBICDIS`"]
pub type EBICDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBICDIS`"]
pub struct EBICDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EBICDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PREFETCH`"]
pub type PREFETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFETCH`"]
pub struct PREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_W<'a> {
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
#[doc = "Reader of field `USEHPROT`"]
pub type USEHPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEHPROT`"]
pub struct USEHPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> USEHPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `QSPICDIS`"]
pub type QSPICDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPICDIS`"]
pub struct QSPICDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPICDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    WS0,
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    WS1,
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    WS2,
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    WS3,
}
impl crate::ToBits<u8> for MODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE_A::WS0 => 0,
            MODE_A::WS1 => 1,
            MODE_A::WS2 => 2,
            MODE_A::WS3 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::WS0,
            1 => MODE_A::WS1,
            2 => MODE_A::WS2,
            3 => MODE_A::WS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == MODE_A::WS2
    }
    #[doc = "Checks if the value of the field is `WS3`"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == MODE_A::WS3
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(MODE_A::WS1)
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(MODE_A::WS2)
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(MODE_A::WS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SCBTP`"]
pub type SCBTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCBTP`"]
pub struct SCBTP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCBTP_W<'a> {
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
impl R {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IFCDIS_R {
        IFCDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AIDIS_R {
        AIDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> ICCDIS_R {
        ICCDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    pub fn ebicdis(&self) -> EBICDIS_R {
        EBICDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&self) -> USEHPROT_R {
        USEHPROT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - QSPI Cache Disable"]
    #[inline(always)]
    pub fn qspicdis(&self) -> QSPICDIS_R {
        QSPICDIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&self) -> SCBTP_R {
        SCBTP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&mut self) -> IFCDIS_W {
        IFCDIS_W { w: self }
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&mut self) -> AIDIS_W {
        AIDIS_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&mut self) -> ICCDIS_W {
        ICCDIS_W { w: self }
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    pub fn ebicdis(&mut self) -> EBICDIS_W {
        EBICDIS_W { w: self }
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PREFETCH_W {
        PREFETCH_W { w: self }
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&mut self) -> USEHPROT_W {
        USEHPROT_W { w: self }
    }
    #[doc = "Bit 10 - QSPI Cache Disable"]
    #[inline(always)]
    pub fn qspicdis(&mut self) -> QSPICDIS_W {
        QSPICDIS_W { w: self }
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&mut self) -> SCBTP_W {
        SCBTP_W { w: self }
    }
}
