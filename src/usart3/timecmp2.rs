#[doc = "Reader of register TIMECMP2"]
pub type R = crate::R<u32, super::TIMECMP2>;
#[doc = "Writer for register TIMECMP2"]
pub type W = crate::W<u32, super::TIMECMP2>;
#[doc = "Register TIMECMP2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMECMP2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCMPVAL`"]
pub type TCMPVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCMPVAL`"]
pub struct TCMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Timer Start Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTART_A {
    #[doc = "0: Comparator 2 is disabled"]
    DISABLE,
    #[doc = "1: Comparator 2 and timer are started at TX end of frame"]
    TXEOF,
    #[doc = "2: Comparator 2 and timer are started at TX Complete"]
    TXC,
    #[doc = "3: Comparator 2 and timer are started at RX going going Active (default: low)"]
    RXACT,
    #[doc = "4: Comparator 2 and timer are started at RX end of frame"]
    RXEOF,
}
impl From<TSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        match variant {
            TSTART_A::DISABLE => 0,
            TSTART_A::TXEOF => 1,
            TSTART_A::TXC => 2,
            TSTART_A::RXACT => 3,
            TSTART_A::RXEOF => 4,
        }
    }
}
#[doc = "Reader of field `TSTART`"]
pub type TSTART_R = crate::R<u8, TSTART_A>;
impl TSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSTART_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSTART_A::DISABLE),
            1 => Val(TSTART_A::TXEOF),
            2 => Val(TSTART_A::TXC),
            3 => Val(TSTART_A::RXACT),
            4 => Val(TSTART_A::RXEOF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TXEOF`"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART_A::TXEOF
    }
    #[doc = "Checks if the value of the field is `TXC`"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART_A::TXC
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXEOF`"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART_A::RXEOF
    }
}
#[doc = "Write proxy for field `TSTART`"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTART_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Comparator 2 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTART_A::DISABLE)
    }
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut W {
        self.variant(TSTART_A::TXEOF)
    }
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut W {
        self.variant(TSTART_A::TXC)
    }
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTART_A::RXACT)
    }
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut W {
        self.variant(TSTART_A::RXEOF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Source Used to Disable Comparator 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOP_A {
    #[doc = "0: Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    TCMP2,
    #[doc = "1: Comparator 2 is disabled at TX start TX Engine"]
    TXST,
    #[doc = "2: Comparator 2 is disabled on RX going going Active (default: low)"]
    RXACT,
    #[doc = "3: Comparator 2 is disabled on RX going Inactive"]
    RXACTN,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        match variant {
            TSTOP_A::TCMP2 => 0,
            TSTOP_A::TXST => 1,
            TSTOP_A::RXACT => 2,
            TSTOP_A::RXACTN => 3,
        }
    }
}
#[doc = "Reader of field `TSTOP`"]
pub type TSTOP_R = crate::R<u8, TSTOP_A>;
impl TSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSTOP_A::TCMP2),
            1 => Val(TSTOP_A::TXST),
            2 => Val(TSTOP_A::RXACT),
            3 => Val(TSTOP_A::RXACTN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TSTOP_A::TCMP2
    }
    #[doc = "Checks if the value of the field is `TXST`"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP_A::TXST
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXACTN`"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP_A::RXACTN
    }
}
#[doc = "Write proxy for field `TSTOP`"]
pub struct TSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(TSTOP_A::TCMP2)
    }
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut W {
        self.variant(TSTOP_A::TXST)
    }
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACT)
    }
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACTN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `RESTARTEN`"]
pub type RESTARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESTARTEN`"]
pub struct RESTARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTARTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline(always)]
    pub fn tcmpval(&self) -> TCMPVAL_R {
        TCMPVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline(always)]
    pub fn restarten(&self) -> RESTARTEN_R {
        RESTARTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline(always)]
    pub fn tcmpval(&mut self) -> TCMPVAL_W {
        TCMPVAL_W { w: self }
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TSTOP_W {
        TSTOP_W { w: self }
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline(always)]
    pub fn restarten(&mut self) -> RESTARTEN_W {
        RESTARTEN_W { w: self }
    }
}
