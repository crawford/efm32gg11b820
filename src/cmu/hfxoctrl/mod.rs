#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOCTRL {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    ACBUFEXTCLK,
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    DCBUFEXTCLK,
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    DIGEXTCLK,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::XTAL => 0,
            MODER::ACBUFEXTCLK => 1,
            MODER::DCBUFEXTCLK => 2,
            MODER::DIGEXTCLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::XTAL,
            1 => MODER::ACBUFEXTCLK,
            2 => MODER::DCBUFEXTCLK,
            3 => MODER::DIGEXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == MODER::XTAL
    }
    #[doc = "Checks if the value of the field is `ACBUFEXTCLK`"]
    #[inline]
    pub fn is_acbufextclk(&self) -> bool {
        *self == MODER::ACBUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DCBUFEXTCLK`"]
    #[inline]
    pub fn is_dcbufextclk(&self) -> bool {
        *self == MODER::DCBUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline]
    pub fn is_digextclk(&self) -> bool {
        *self == MODER::DIGEXTCLK
    }
}
#[doc = r" Value of the field"]
pub struct HFXOX2ENR {
    bits: bool,
}
impl HFXOX2ENR {
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
#[doc = "Possible values of the field `PEAKDETMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETMODER {
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    ONCECMD,
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    AUTOCMD,
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    CMD,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL,
}
impl PEAKDETMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEAKDETMODER::ONCECMD => 0,
            PEAKDETMODER::AUTOCMD => 1,
            PEAKDETMODER::CMD => 2,
            PEAKDETMODER::MANUAL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEAKDETMODER {
        match value {
            0 => PEAKDETMODER::ONCECMD,
            1 => PEAKDETMODER::AUTOCMD,
            2 => PEAKDETMODER::CMD,
            3 => PEAKDETMODER::MANUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONCECMD`"]
    #[inline]
    pub fn is_oncecmd(&self) -> bool {
        *self == PEAKDETMODER::ONCECMD
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETMODER::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETMODER::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETMODER::MANUAL
    }
}
#[doc = "Possible values of the field `LFTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFTIMEOUTR {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    _0CYCLES,
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 64 cycles"]
    _64CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
}
impl LFTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFTIMEOUTR::_0CYCLES => 0,
            LFTIMEOUTR::_2CYCLES => 1,
            LFTIMEOUTR::_4CYCLES => 2,
            LFTIMEOUTR::_16CYCLES => 3,
            LFTIMEOUTR::_32CYCLES => 4,
            LFTIMEOUTR::_64CYCLES => 5,
            LFTIMEOUTR::_1KCYCLES => 6,
            LFTIMEOUTR::_4KCYCLES => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFTIMEOUTR {
        match value {
            0 => LFTIMEOUTR::_0CYCLES,
            1 => LFTIMEOUTR::_2CYCLES,
            2 => LFTIMEOUTR::_4CYCLES,
            3 => LFTIMEOUTR::_16CYCLES,
            4 => LFTIMEOUTR::_32CYCLES,
            5 => LFTIMEOUTR::_64CYCLES,
            6 => LFTIMEOUTR::_1KCYCLES,
            7 => LFTIMEOUTR::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUTR::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUTR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUTR::_4KCYCLES
    }
}
#[doc = r" Value of the field"]
pub struct AUTOSTARTEM0EM1R {
    bits: bool,
}
impl AUTOSTARTEM0EM1R {
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
pub struct AUTOSTARTSELEM0EM1R {
    bits: bool,
}
impl AUTOSTARTSELEM0EM1R {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    ACBUFEXTCLK,
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    DCBUFEXTCLK,
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    DIGEXTCLK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::XTAL => 0,
            MODEW::ACBUFEXTCLK => 1,
            MODEW::DCBUFEXTCLK => 2,
            MODEW::DIGEXTCLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODEW::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline]
    pub fn acbufextclk(self) -> &'a mut W {
        self.variant(MODEW::ACBUFEXTCLK)
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline]
    pub fn dcbufextclk(self) -> &'a mut W {
        self.variant(MODEW::DCBUFEXTCLK)
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODEW::DIGEXTCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFXOX2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOX2ENW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEAKDETMODE`"]
pub enum PEAKDETMODEW {
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    ONCECMD,
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    AUTOCMD,
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    CMD,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL,
}
impl PEAKDETMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PEAKDETMODEW::ONCECMD => 0,
            PEAKDETMODEW::AUTOCMD => 1,
            PEAKDETMODEW::CMD => 2,
            PEAKDETMODEW::MANUAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEAKDETMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline]
    pub fn oncecmd(self) -> &'a mut W {
        self.variant(PEAKDETMODEW::ONCECMD)
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETMODEW::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETMODEW::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETMODEW::MANUAL)
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
#[doc = "Values that can be written to the field `LFTIMEOUT`"]
pub enum LFTIMEOUTW {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    _0CYCLES,
    #[doc = "Timeout period of 2 cycles"]
    _2CYCLES,
    #[doc = "Timeout period of 4 cycles"]
    _4CYCLES,
    #[doc = "Timeout period of 16 cycles"]
    _16CYCLES,
    #[doc = "Timeout period of 32 cycles"]
    _32CYCLES,
    #[doc = "Timeout period of 64 cycles"]
    _64CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
}
impl LFTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFTIMEOUTW::_0CYCLES => 0,
            LFTIMEOUTW::_2CYCLES => 1,
            LFTIMEOUTW::_4CYCLES => 2,
            LFTIMEOUTW::_16CYCLES => 3,
            LFTIMEOUTW::_32CYCLES => 4,
            LFTIMEOUTW::_64CYCLES => 5,
            LFTIMEOUTW::_1KCYCLES => 6,
            LFTIMEOUTW::_4KCYCLES => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LFTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFTIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUTW::_4KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOSTARTEM0EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTARTEM0EM1W<'a> {
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
pub struct _AUTOSTARTSELEM0EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTARTSELEM0EM1W<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline]
    pub fn hfxox2en(&self) -> HFXOX2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFXOX2ENR { bits }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline]
    pub fn peakdetmode(&self) -> PEAKDETMODER {
        PEAKDETMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline]
    pub fn lftimeout(&self) -> LFTIMEOUTR {
        LFTIMEOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSTARTEM0EM1R { bits }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSTARTSELEM0EM1R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline]
    pub fn hfxox2en(&mut self) -> _HFXOX2ENW {
        _HFXOX2ENW { w: self }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline]
    pub fn peakdetmode(&mut self) -> _PEAKDETMODEW {
        _PEAKDETMODEW { w: self }
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline]
    pub fn lftimeout(&mut self) -> _LFTIMEOUTW {
        _LFTIMEOUTW { w: self }
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartem0em1(&mut self) -> _AUTOSTARTEM0EM1W {
        _AUTOSTARTEM0EM1W { w: self }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline]
    pub fn autostartselem0em1(&mut self) -> _AUTOSTARTSELEM0EM1W {
        _AUTOSTARTSELEM0EM1W { w: self }
    }
}
