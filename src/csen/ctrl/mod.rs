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
        R {
            bits: self.register.get(),
        }
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct CMPPOLR {
    bits: bool,
}
impl CMPPOLR {
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
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    SGL,
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    SCAN,
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    CONTSGL,
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    CONTSCAN,
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::SGL => 0,
            CMR::SCAN => 1,
            CMR::CONTSGL => 2,
            CMR::CONTSCAN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::SGL,
            1 => CMR::SCAN,
            2 => CMR::CONTSGL,
            3 => CMR::CONTSCAN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SGL`"]
    #[inline]
    pub fn is_sgl(&self) -> bool {
        *self == CMR::SGL
    }
    #[doc = "Checks if the value of the field is `SCAN`"]
    #[inline]
    pub fn is_scan(&self) -> bool {
        *self == CMR::SCAN
    }
    #[doc = "Checks if the value of the field is `CONTSGL`"]
    #[inline]
    pub fn is_contsgl(&self) -> bool {
        *self == CMR::CONTSGL
    }
    #[doc = "Checks if the value of the field is `CONTSCAN`"]
    #[inline]
    pub fn is_contscan(&self) -> bool {
        *self == CMR::CONTSCAN
    }
}
#[doc = "Possible values of the field `SARCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARCRR {
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    CLK10,
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    CLK12,
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    CLK14,
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    CLK16,
}
impl SARCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SARCRR::CLK10 => 0,
            SARCRR::CLK12 => 1,
            SARCRR::CLK14 => 2,
            SARCRR::CLK16 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SARCRR {
        match value {
            0 => SARCRR::CLK10,
            1 => SARCRR::CLK12,
            2 => SARCRR::CLK14,
            3 => SARCRR::CLK16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK10`"]
    #[inline]
    pub fn is_clk10(&self) -> bool {
        *self == SARCRR::CLK10
    }
    #[doc = "Checks if the value of the field is `CLK12`"]
    #[inline]
    pub fn is_clk12(&self) -> bool {
        *self == SARCRR::CLK12
    }
    #[doc = "Checks if the value of the field is `CLK14`"]
    #[inline]
    pub fn is_clk14(&self) -> bool {
        *self == SARCRR::CLK14
    }
    #[doc = "Checks if the value of the field is `CLK16`"]
    #[inline]
    pub fn is_clk16(&self) -> bool {
        *self == SARCRR::CLK16
    }
}
#[doc = "Possible values of the field `ACU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACUR {
    #[doc = "Accumulate 1 sample."]
    ACC1,
    #[doc = "Accumulate 2 sample."]
    ACC2,
    #[doc = "Accumulate 4 sample."]
    ACC4,
    #[doc = "Accumulate 8 sample."]
    ACC8,
    #[doc = "Accumulate 16 sample."]
    ACC16,
    #[doc = "Accumulate 32 sample."]
    ACC32,
    #[doc = "Accumulate 64 sample."]
    ACC64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACUR::ACC1 => 0,
            ACUR::ACC2 => 1,
            ACUR::ACC4 => 2,
            ACUR::ACC8 => 3,
            ACUR::ACC16 => 4,
            ACUR::ACC32 => 5,
            ACUR::ACC64 => 6,
            ACUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACUR {
        match value {
            0 => ACUR::ACC1,
            1 => ACUR::ACC2,
            2 => ACUR::ACC4,
            3 => ACUR::ACC8,
            4 => ACUR::ACC16,
            5 => ACUR::ACC32,
            6 => ACUR::ACC64,
            i => ACUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACC1`"]
    #[inline]
    pub fn is_acc1(&self) -> bool {
        *self == ACUR::ACC1
    }
    #[doc = "Checks if the value of the field is `ACC2`"]
    #[inline]
    pub fn is_acc2(&self) -> bool {
        *self == ACUR::ACC2
    }
    #[doc = "Checks if the value of the field is `ACC4`"]
    #[inline]
    pub fn is_acc4(&self) -> bool {
        *self == ACUR::ACC4
    }
    #[doc = "Checks if the value of the field is `ACC8`"]
    #[inline]
    pub fn is_acc8(&self) -> bool {
        *self == ACUR::ACC8
    }
    #[doc = "Checks if the value of the field is `ACC16`"]
    #[inline]
    pub fn is_acc16(&self) -> bool {
        *self == ACUR::ACC16
    }
    #[doc = "Checks if the value of the field is `ACC32`"]
    #[inline]
    pub fn is_acc32(&self) -> bool {
        *self == ACUR::ACC32
    }
    #[doc = "Checks if the value of the field is `ACC64`"]
    #[inline]
    pub fn is_acc64(&self) -> bool {
        *self == ACUR::ACC64
    }
}
#[doc = r" Value of the field"]
pub struct MCENR {
    bits: bool,
}
impl MCENR {
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
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    PRS,
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    TIMER,
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    START,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STMR::PRS => 0,
            STMR::TIMER => 1,
            STMR::START => 2,
            STMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STMR {
        match value {
            0 => STMR::PRS,
            1 => STMR::TIMER,
            2 => STMR::START,
            i => STMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == STMR::PRS
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == STMR::TIMER
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STMR::START
    }
}
#[doc = r" Value of the field"]
pub struct CMPENR {
    bits: bool,
}
impl CMPENR {
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
pub struct DRSFR {
    bits: bool,
}
impl DRSFR {
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
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
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
pub struct CONVSELR {
    bits: bool,
}
impl CONVSELR {
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
pub struct CHOPENR {
    bits: bool,
}
impl CHOPENR {
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
pub struct AUTOGNDR {
    bits: bool,
}
impl AUTOGNDR {
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
pub struct MXUCR {
    bits: bool,
}
impl MXUCR {
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
pub struct EMACMPENR {
    bits: bool,
}
impl EMACMPENR {
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
pub struct LOCALSENSR {
    bits: bool,
}
impl LOCALSENSR {
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
pub struct CPACCURACYR {
    bits: bool,
}
impl CPACCURACYR {
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPPOLW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    SGL,
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    SCAN,
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    CONTSGL,
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    CONTSCAN,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::SGL => 0,
            CMW::SCAN => 1,
            CMW::CONTSGL => 2,
            CMW::CONTSCAN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline]
    pub fn sgl(self) -> &'a mut W {
        self.variant(CMW::SGL)
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline]
    pub fn scan(self) -> &'a mut W {
        self.variant(CMW::SCAN)
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline]
    pub fn contsgl(self) -> &'a mut W {
        self.variant(CMW::CONTSGL)
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline]
    pub fn contscan(self) -> &'a mut W {
        self.variant(CMW::CONTSCAN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SARCR`"]
pub enum SARCRW {
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    CLK10,
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    CLK12,
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    CLK14,
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    CLK16,
}
impl SARCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SARCRW::CLK10 => 0,
            SARCRW::CLK12 => 1,
            SARCRW::CLK14 => 2,
            SARCRW::CLK16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SARCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SARCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SARCRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline]
    pub fn clk10(self) -> &'a mut W {
        self.variant(SARCRW::CLK10)
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline]
    pub fn clk12(self) -> &'a mut W {
        self.variant(SARCRW::CLK12)
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline]
    pub fn clk14(self) -> &'a mut W {
        self.variant(SARCRW::CLK14)
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline]
    pub fn clk16(self) -> &'a mut W {
        self.variant(SARCRW::CLK16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACU`"]
pub enum ACUW {
    #[doc = "Accumulate 1 sample."]
    ACC1,
    #[doc = "Accumulate 2 sample."]
    ACC2,
    #[doc = "Accumulate 4 sample."]
    ACC4,
    #[doc = "Accumulate 8 sample."]
    ACC8,
    #[doc = "Accumulate 16 sample."]
    ACC16,
    #[doc = "Accumulate 32 sample."]
    ACC32,
    #[doc = "Accumulate 64 sample."]
    ACC64,
}
impl ACUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACUW::ACC1 => 0,
            ACUW::ACC2 => 1,
            ACUW::ACC4 => 2,
            ACUW::ACC8 => 3,
            ACUW::ACC16 => 4,
            ACUW::ACC32 => 5,
            ACUW::ACC64 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACUW<'a> {
    w: &'a mut W,
}
impl<'a> _ACUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accumulate 1 sample."]
    #[inline]
    pub fn acc1(self) -> &'a mut W {
        self.variant(ACUW::ACC1)
    }
    #[doc = "Accumulate 2 sample."]
    #[inline]
    pub fn acc2(self) -> &'a mut W {
        self.variant(ACUW::ACC2)
    }
    #[doc = "Accumulate 4 sample."]
    #[inline]
    pub fn acc4(self) -> &'a mut W {
        self.variant(ACUW::ACC4)
    }
    #[doc = "Accumulate 8 sample."]
    #[inline]
    pub fn acc8(self) -> &'a mut W {
        self.variant(ACUW::ACC8)
    }
    #[doc = "Accumulate 16 sample."]
    #[inline]
    pub fn acc16(self) -> &'a mut W {
        self.variant(ACUW::ACC16)
    }
    #[doc = "Accumulate 32 sample."]
    #[inline]
    pub fn acc32(self) -> &'a mut W {
        self.variant(ACUW::ACC32)
    }
    #[doc = "Accumulate 64 sample."]
    #[inline]
    pub fn acc64(self) -> &'a mut W {
        self.variant(ACUW::ACC64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCENW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STM`"]
pub enum STMW {
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    PRS,
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    TIMER,
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    START,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STMW::PRS => 0,
            STMW::TIMER => 1,
            STMW::START => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(STMW::PRS)
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(STMW::TIMER)
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STMW::START)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPENW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRSFW<'a> {
    w: &'a mut W,
}
impl<'a> _DRSFW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVSELW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHOPENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOGNDW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOGNDW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MXUCW<'a> {
    w: &'a mut W,
}
impl<'a> _MXUCW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EMACMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMACMPENW<'a> {
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
        const OFFSET: u8 = 25;
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCALSENSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCALSENSW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPACCURACYW<'a> {
    w: &'a mut W,
}
impl<'a> _CPACCURACYW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline]
    pub fn cmppol(&self) -> CMPPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPPOLR { bits }
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline]
    pub fn sarcr(&self) -> SARCRR {
        SARCRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline]
    pub fn acu(&self) -> ACUR {
        ACUR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline]
    pub fn mcen(&self) -> MCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCENR { bits }
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline]
    pub fn stm(&self) -> STMR {
        STMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline]
    pub fn cmpen(&self) -> CMPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPENR { bits }
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline]
    pub fn drsf(&self) -> DRSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRSFR { bits }
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAENR { bits }
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline]
    pub fn convsel(&self) -> CONVSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONVSELR { bits }
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline]
    pub fn chopen(&self) -> CHOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHOPENR { bits }
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline]
    pub fn autognd(&self) -> AUTOGNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOGNDR { bits }
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline]
    pub fn mxuc(&self) -> MXUCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MXUCR { bits }
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline]
    pub fn emacmpen(&self) -> EMACMPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMACMPENR { bits }
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WARMUPMODER { bits }
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline]
    pub fn localsens(&self) -> LOCALSENSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCALSENSR { bits }
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline]
    pub fn cpaccuracy(&self) -> CPACCURACYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPACCURACYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 196608 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline]
    pub fn cmppol(&mut self) -> _CMPPOLW {
        _CMPPOLW { w: self }
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline]
    pub fn sarcr(&mut self) -> _SARCRW {
        _SARCRW { w: self }
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline]
    pub fn acu(&mut self) -> _ACUW {
        _ACUW { w: self }
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline]
    pub fn mcen(&mut self) -> _MCENW {
        _MCENW { w: self }
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline]
    pub fn cmpen(&mut self) -> _CMPENW {
        _CMPENW { w: self }
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline]
    pub fn drsf(&mut self) -> _DRSFW {
        _DRSFW { w: self }
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline]
    pub fn convsel(&mut self) -> _CONVSELW {
        _CONVSELW { w: self }
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline]
    pub fn chopen(&mut self) -> _CHOPENW {
        _CHOPENW { w: self }
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline]
    pub fn autognd(&mut self) -> _AUTOGNDW {
        _AUTOGNDW { w: self }
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline]
    pub fn mxuc(&mut self) -> _MXUCW {
        _MXUCW { w: self }
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline]
    pub fn emacmpen(&mut self) -> _EMACMPENW {
        _EMACMPENW { w: self }
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline]
    pub fn localsens(&mut self) -> _LOCALSENSW {
        _LOCALSENSW { w: self }
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline]
    pub fn cpaccuracy(&mut self) -> _CPACCURACYW {
        _CPACCURACYW { w: self }
    }
}
