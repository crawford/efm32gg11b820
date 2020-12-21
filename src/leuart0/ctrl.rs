#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTOTRI`"]
pub type AUTOTRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTRI`"]
pub struct AUTOTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTRI_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DATABITS`"]
pub type DATABITS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATABITS`"]
pub struct DATABITS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABITS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Parity bits are not used"]
    NONE = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD = 3,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<u8, PARITY_A>;
impl PARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PARITY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PARITY_A::NONE),
            2 => Val(PARITY_A::EVEN),
            3 => Val(PARITY_A::ODD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
}
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITY_A::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_A::ODD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `STOPBITS`"]
pub type STOPBITS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPBITS`"]
pub struct STOPBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPBITS_W<'a> {
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
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
#[doc = "Reader of field `ERRSDMA`"]
pub type ERRSDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSDMA`"]
pub struct ERRSDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSDMA_W<'a> {
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
#[doc = "Reader of field `LOOPBK`"]
pub type LOOPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBK`"]
pub struct LOOPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SFUBRX`"]
pub type SFUBRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFUBRX`"]
pub struct SFUBRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SFUBRX_W<'a> {
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
#[doc = "Reader of field `MPM`"]
pub type MPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPM`"]
pub struct MPM_W<'a> {
    w: &'a mut W,
}
impl<'a> MPM_W<'a> {
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
#[doc = "Reader of field `MPAB`"]
pub type MPAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPAB`"]
pub struct MPAB_W<'a> {
    w: &'a mut W,
}
impl<'a> MPAB_W<'a> {
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
#[doc = "Reader of field `BIT8DV`"]
pub type BIT8DV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT8DV`"]
pub struct BIT8DV_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT8DV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RXDMAWU`"]
pub type RXDMAWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAWU`"]
pub struct RXDMAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAWU_W<'a> {
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
#[doc = "Reader of field `TXDMAWU`"]
pub type TXDMAWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMAWU`"]
pub struct TXDMAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAWU_W<'a> {
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
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Frames are transmitted immediately"]
    NONE = 0,
    #[doc = "1: Transmission of new frames are delayed by a single bit period"]
    SINGLE = 1,
    #[doc = "2: Transmission of new frames are delayed by two bit periods"]
    DOUBLE = 2,
    #[doc = "3: Transmission of new frames are delayed by three bit periods"]
    TRIPLE = 3,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXDELAY`"]
pub type TXDELAY_R = crate::R<u8, TXDELAY_A>;
impl TXDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::NONE,
            1 => TXDELAY_A::SINGLE,
            2 => TXDELAY_A::DOUBLE,
            3 => TXDELAY_A::TRIPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY_A::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPLE`"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == TXDELAY_A::TRIPLE
    }
}
#[doc = "Write proxy for field `TXDELAY`"]
pub struct TXDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Frames are transmitted immediately"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDELAY_A::NONE)
    }
    #[doc = "Transmission of new frames are delayed by a single bit period"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDELAY_A::SINGLE)
    }
    #[doc = "Transmission of new frames are delayed by two bit periods"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(TXDELAY_A::DOUBLE)
    }
    #[doc = "Transmission of new frames are delayed by three bit periods"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut W {
        self.variant(TXDELAY_A::TRIPLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SFUBRX_R {
        SFUBRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RXDMAWU_R {
        RXDMAWU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TXDMAWU_R {
        TXDMAWU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AUTOTRI_W {
        AUTOTRI_W { w: self }
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DATABITS_W {
        DATABITS_W { w: self }
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W { w: self }
    }
    #[doc = "Bit 5 - Invert Input and Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 6 - Clear RX DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ERRSDMA_W {
        ERRSDMA_W { w: self }
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LOOPBK_W {
        LOOPBK_W { w: self }
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SFUBRX_W {
        SFUBRX_W { w: self }
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MPM_W {
        MPM_W { w: self }
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MPAB_W {
        MPAB_W { w: self }
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> BIT8DV_W {
        BIT8DV_W { w: self }
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&mut self) -> RXDMAWU_W {
        RXDMAWU_W { w: self }
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&mut self) -> TXDMAWU_W {
        TXDMAWU_W { w: self }
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TXDELAY_W {
        TXDELAY_W { w: self }
    }
}
