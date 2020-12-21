#[doc = "Reader of register LFXOCTRL"]
pub type R = crate::R<u32, super::LFXOCTRL>;
#[doc = "Writer for register LFXOCTRL"]
pub type W = crate::W<u32, super::LFXOCTRL>;
#[doc = "Register LFXOCTRL `reset()`'s with value 0x0700_9000"]
impl crate::ResetValue for super::LFXOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700_9000
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 32768 Hz crystal oscillator"]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    BUFEXTCLK = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::XTAL),
            1 => Val(MODE_A::BUFEXTCLK),
            2 => Val(MODE_A::DIGEXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE_A::DIGEXTCLK
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(MODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODE_A::DIGEXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `HIGHAMPL`"]
pub type HIGHAMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGHAMPL`"]
pub struct HIGHAMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHAMPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `AGC`"]
pub type AGC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AGC`"]
pub struct AGC_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_W<'a> {
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
#[doc = "Reader of field `CUR`"]
pub type CUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR`"]
pub struct CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `BUFCUR`"]
pub type BUFCUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFCUR`"]
pub struct BUFCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFCUR_W<'a> {
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
#[doc = "LFXO Timeout\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    _256CYCLES = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    _1KCYCLES = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    _2KCYCLES = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    _4KCYCLES = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    _8KCYCLES = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    _16KCYCLES = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
    _32KCYCLES = 7,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::_2CYCLES,
            1 => TIMEOUT_A::_256CYCLES,
            2 => TIMEOUT_A::_1KCYCLES,
            3 => TIMEOUT_A::_2KCYCLES,
            4 => TIMEOUT_A::_4KCYCLES,
            5 => TIMEOUT_A::_8KCYCLES,
            6 => TIMEOUT_A::_16KCYCLES,
            7 => TIMEOUT_A::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == TIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == TIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == TIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == TIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == TIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == TIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == TIMEOUT_A::_32KCYCLES
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HIGHAMPL_R {
        HIGHAMPL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&self) -> BUFCUR_R {
        BUFCUR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&mut self) -> HIGHAMPL_W {
        HIGHAMPL_W { w: self }
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&mut self) -> AGC_W {
        AGC_W { w: self }
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&mut self) -> CUR_W {
        CUR_W { w: self }
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&mut self) -> BUFCUR_W {
        BUFCUR_W { w: self }
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
}
