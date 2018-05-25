#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DIFFR {
    bits: bool,
}
impl DIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SINEMODER {
    bits: bool,
}
impl SINEMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OUTENPRSR {
    bits: bool,
}
impl OUTENPRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CH0PRESCRSTR {
    bits: bool,
}
impl CH0PRESCRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    _1V25LN,
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    _2V5LN,
    #[doc = "Internal 1.25 V bandgap reference"]
    _1V25,
    #[doc = "Internal 2.5 V bandgap reference"]
    _2V5,
    #[doc = "AVDD reference"]
    VDD,
    #[doc = "External pin reference"]
    EXT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::_1V25LN => 0,
            REFSELR::_2V5LN => 1,
            REFSELR::_1V25 => 2,
            REFSELR::_2V5 => 3,
            REFSELR::VDD => 4,
            REFSELR::EXT => 6,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::_1V25LN,
            1 => REFSELR::_2V5LN,
            2 => REFSELR::_1V25,
            3 => REFSELR::_2V5,
            4 => REFSELR::VDD,
            6 => REFSELR::EXT,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25LN`"]
    #[inline]
    pub fn is_1v25ln(&self) -> bool {
        *self == REFSELR::_1V25LN
    }
    #[doc = "Checks if the value of the field is `_2V5LN`"]
    #[inline]
    pub fn is_2v5ln(&self) -> bool {
        *self == REFSELR::_2V5LN
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == REFSELR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == REFSELR::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == REFSELR::VDD
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline]
    pub fn is_ext(&self) -> bool {
        *self == REFSELR::EXT
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::NODIVISION => 0,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::NODIVISION,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESCR::NODIVISION
    }
}
#[doc = "Possible values of the field `REFRESHPERIOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFRESHPERIODR {
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    _8CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    _16CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    _32CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    _64CYCLES,
}
impl REFRESHPERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFRESHPERIODR::_8CYCLES => 0,
            REFRESHPERIODR::_16CYCLES => 1,
            REFRESHPERIODR::_32CYCLES => 2,
            REFRESHPERIODR::_64CYCLES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFRESHPERIODR {
        match value {
            0 => REFRESHPERIODR::_8CYCLES,
            1 => REFRESHPERIODR::_16CYCLES,
            2 => REFRESHPERIODR::_32CYCLES,
            3 => REFRESHPERIODR::_64CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == REFRESHPERIODR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == REFRESHPERIODR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == REFRESHPERIODR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == REFRESHPERIODR::_64CYCLES
    }
}
#[doc = r" Value of the field"]
pub struct WARMUPMODER {
    bits: bool,
}
impl WARMUPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACCLKMODER {
    bits: bool,
}
impl DACCLKMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINEMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINEMODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTENPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTENPRSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0PRESCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0PRESCRSTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    _1V25LN,
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    _2V5LN,
    #[doc = "Internal 1.25 V bandgap reference"]
    _1V25,
    #[doc = "Internal 2.5 V bandgap reference"]
    _2V5,
    #[doc = "AVDD reference"]
    VDD,
    #[doc = "External pin reference"]
    EXT,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::_1V25LN => 0,
            REFSELW::_2V5LN => 1,
            REFSELW::_1V25 => 2,
            REFSELW::_2V5 => 3,
            REFSELW::VDD => 4,
            REFSELW::EXT => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    #[inline]
    pub fn _1v25ln(self) -> &'a mut W {
        self.variant(REFSELW::_1V25LN)
    }
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    #[inline]
    pub fn _2v5ln(self) -> &'a mut W {
        self.variant(REFSELW::_2V5LN)
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFSELW::_1V25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFSELW::_2V5)
    }
    #[doc = "AVDD reference"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSELW::VDD)
    }
    #[doc = "External pin reference"]
    #[inline]
    pub fn ext(self) -> &'a mut W {
        self.variant(REFSELW::EXT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "\"\""]
    NODIVISION,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESCW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFRESHPERIOD`"]
pub enum REFRESHPERIODW {
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    _8CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    _16CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    _32CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    _64CYCLES,
}
impl REFRESHPERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFRESHPERIODW::_8CYCLES => 0,
            REFRESHPERIODW::_16CYCLES => 1,
            REFRESHPERIODW::_32CYCLES => 2,
            REFRESHPERIODW::_64CYCLES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFRESHPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _REFRESHPERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFRESHPERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIODW::_8CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIODW::_16CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIODW::_32CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIODW::_64CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPMODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACCLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCLKMODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIFFR { bits }
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline]
    pub fn sinemode(&self) -> SINEMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINEMODER { bits }
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline]
    pub fn outenprs(&self) -> OUTENPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTENPRSR { bits }
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline]
    pub fn ch0prescrst(&self) -> CH0PRESCRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0PRESCRSTR { bits }
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline]
    pub fn refreshperiod(&self) -> REFRESHPERIODR {
        REFRESHPERIODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WARMUPMODER { bits }
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline]
    pub fn dacclkmode(&self) -> DACCLKMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCLKMODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline]
    pub fn sinemode(&mut self) -> _SINEMODEW {
        _SINEMODEW { w: self }
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline]
    pub fn outenprs(&mut self) -> _OUTENPRSW {
        _OUTENPRSW { w: self }
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline]
    pub fn ch0prescrst(&mut self) -> _CH0PRESCRSTW {
        _CH0PRESCRSTW { w: self }
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline]
    pub fn refreshperiod(&mut self) -> _REFRESHPERIODW {
        _REFRESHPERIODW { w: self }
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline]
    pub fn dacclkmode(&mut self) -> _DACCLKMODEW {
        _DACCLKMODEW { w: self }
    }
}
