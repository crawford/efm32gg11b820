#[doc = "Reader of register LFRCOCTRL"]
pub type R = crate::R<u32, super::LFRCOCTRL>;
#[doc = "Writer for register LFRCOCTRL"]
pub type W = crate::W<u32, super::LFRCOCTRL>;
#[doc = "Register LFRCOCTRL `reset()`'s with value 0x8106_0100"]
impl crate::ResetValue for super::LFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8106_0100
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `ENVREF`"]
pub type ENVREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENVREF`"]
pub struct ENVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVREF_W<'a> {
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
#[doc = "Reader of field `ENCHOP`"]
pub type ENCHOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCHOP`"]
pub struct ENCHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCHOP_W<'a> {
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
#[doc = "Reader of field `ENDEM`"]
pub type ENDEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDEM`"]
pub struct ENDEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEM_W<'a> {
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
#[doc = "Control Vref Update Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFUPDATE_A {
    #[doc = "0: 32 clocks."]
    _32CYCLES,
    #[doc = "1: 64 clocks."]
    _64CYCLES,
    #[doc = "2: 128 clocks."]
    _128CYCLES,
    #[doc = "3: 256 clocks."]
    _256CYCLES,
}
impl From<VREFUPDATE_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFUPDATE_A) -> Self {
        match variant {
            VREFUPDATE_A::_32CYCLES => 0,
            VREFUPDATE_A::_64CYCLES => 1,
            VREFUPDATE_A::_128CYCLES => 2,
            VREFUPDATE_A::_256CYCLES => 3,
        }
    }
}
#[doc = "Reader of field `VREFUPDATE`"]
pub type VREFUPDATE_R = crate::R<u8, VREFUPDATE_A>;
impl VREFUPDATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFUPDATE_A {
        match self.bits {
            0 => VREFUPDATE_A::_32CYCLES,
            1 => VREFUPDATE_A::_64CYCLES,
            2 => VREFUPDATE_A::_128CYCLES,
            3 => VREFUPDATE_A::_256CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == VREFUPDATE_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == VREFUPDATE_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == VREFUPDATE_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == VREFUPDATE_A::_256CYCLES
    }
}
#[doc = "Write proxy for field `VREFUPDATE`"]
pub struct VREFUPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFUPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFUPDATE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 clocks."]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_32CYCLES)
    }
    #[doc = "64 clocks."]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_64CYCLES)
    }
    #[doc = "128 clocks."]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_128CYCLES)
    }
    #[doc = "256 clocks."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_256CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "LFRCO Timeout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "1: Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "2: Timeout period of 32 cycles"]
    _32CYCLES,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        match variant {
            TIMEOUT_A::_2CYCLES => 0,
            TIMEOUT_A::_16CYCLES => 1,
            TIMEOUT_A::_32CYCLES => 2,
        }
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUT_A::_2CYCLES),
            1 => Val(TIMEOUT_A::_16CYCLES),
            2 => Val(TIMEOUT_A::_32CYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == TIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == TIMEOUT_A::_32CYCLES
    }
}
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_32CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `GMCCURTUNE`"]
pub type GMCCURTUNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GMCCURTUNE`"]
pub struct GMCCURTUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> GMCCURTUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&self) -> ENVREF_R {
        ENVREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&self) -> ENCHOP_R {
        ENCHOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&self) -> ENDEM_R {
        ENDEM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    pub fn vrefupdate(&self) -> VREFUPDATE_R {
        VREFUPDATE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&self) -> GMCCURTUNE_R {
        GMCCURTUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&mut self) -> ENVREF_W {
        ENVREF_W { w: self }
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&mut self) -> ENCHOP_W {
        ENCHOP_W { w: self }
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&mut self) -> ENDEM_W {
        ENDEM_W { w: self }
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    pub fn vrefupdate(&mut self) -> VREFUPDATE_W {
        VREFUPDATE_W { w: self }
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&mut self) -> GMCCURTUNE_W {
        GMCCURTUNE_W { w: self }
    }
}
