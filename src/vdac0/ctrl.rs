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
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFF`"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
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
#[doc = "Reader of field `SINEMODE`"]
pub type SINEMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINEMODE`"]
pub struct SINEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINEMODE_W<'a> {
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
#[doc = "Reader of field `OUTENPRS`"]
pub type OUTENPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTENPRS`"]
pub struct OUTENPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTENPRS_W<'a> {
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
#[doc = "Reader of field `CH0PRESCRST`"]
pub type CH0PRESCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0PRESCRST`"]
pub struct CH0PRESCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PRESCRST_W<'a> {
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
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: Internal low noise 1.25 V bandgap reference"]
    _1V25LN,
    #[doc = "1: Internal low noise 2.5 V bandgap reference"]
    _2V5LN,
    #[doc = "2: Internal 1.25 V bandgap reference"]
    _1V25,
    #[doc = "3: Internal 2.5 V bandgap reference"]
    _2V5,
    #[doc = "4: AVDD reference"]
    VDD,
    #[doc = "6: External pin reference"]
    EXT,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        match variant {
            REFSEL_A::_1V25LN => 0,
            REFSEL_A::_2V5LN => 1,
            REFSEL_A::_1V25 => 2,
            REFSEL_A::_2V5 => 3,
            REFSEL_A::VDD => 4,
            REFSEL_A::EXT => 6,
        }
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::_1V25LN),
            1 => Val(REFSEL_A::_2V5LN),
            2 => Val(REFSEL_A::_1V25),
            3 => Val(REFSEL_A::_2V5),
            4 => Val(REFSEL_A::VDD),
            6 => Val(REFSEL_A::EXT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25LN`"]
    #[inline(always)]
    pub fn is_1v25ln(&self) -> bool {
        *self == REFSEL_A::_1V25LN
    }
    #[doc = "Checks if the value of the field is `_2V5LN`"]
    #[inline(always)]
    pub fn is_2v5ln(&self) -> bool {
        *self == REFSEL_A::_2V5LN
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REFSEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REFSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == REFSEL_A::EXT
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25ln(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V25LN)
    }
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5ln(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V5LN)
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V5)
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(REFSEL_A::EXT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Prescaler Setting for DAC Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        match variant {
            PRESC_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Refresh Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFRESHPERIOD_A {
    #[doc = "0: All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    _8CYCLES,
    #[doc = "1: All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    _16CYCLES,
    #[doc = "2: All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    _32CYCLES,
    #[doc = "3: All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    _64CYCLES,
}
impl From<REFRESHPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESHPERIOD_A) -> Self {
        match variant {
            REFRESHPERIOD_A::_8CYCLES => 0,
            REFRESHPERIOD_A::_16CYCLES => 1,
            REFRESHPERIOD_A::_32CYCLES => 2,
            REFRESHPERIOD_A::_64CYCLES => 3,
        }
    }
}
#[doc = "Reader of field `REFRESHPERIOD`"]
pub type REFRESHPERIOD_R = crate::R<u8, REFRESHPERIOD_A>;
impl REFRESHPERIOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRESHPERIOD_A {
        match self.bits {
            0 => REFRESHPERIOD_A::_8CYCLES,
            1 => REFRESHPERIOD_A::_16CYCLES,
            2 => REFRESHPERIOD_A::_32CYCLES,
            3 => REFRESHPERIOD_A::_64CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_64CYCLES
    }
}
#[doc = "Write proxy for field `REFRESHPERIOD`"]
pub struct REFRESHPERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESHPERIOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFRESHPERIOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_8CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_16CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_32CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_64CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `WARMUPMODE`"]
pub type WARMUPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WARMUPMODE`"]
pub struct WARMUPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPMODE_W<'a> {
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
#[doc = "Reader of field `DACCLKMODE`"]
pub type DACCLKMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCLKMODE`"]
pub struct DACCLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCLKMODE_W<'a> {
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
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SINEMODE_R {
        SINEMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OUTENPRS_R {
        OUTENPRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> CH0PRESCRST_R {
        CH0PRESCRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&self) -> REFRESHPERIOD_R {
        REFRESHPERIOD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&self) -> DACCLKMODE_R {
        DACCLKMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&mut self) -> SINEMODE_W {
        SINEMODE_W { w: self }
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&mut self) -> OUTENPRS_W {
        OUTENPRS_W { w: self }
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&mut self) -> CH0PRESCRST_W {
        CH0PRESCRST_W { w: self }
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&mut self) -> REFRESHPERIOD_W {
        REFRESHPERIOD_W { w: self }
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W { w: self }
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&mut self) -> DACCLKMODE_W {
        DACCLKMODE_W { w: self }
    }
}
