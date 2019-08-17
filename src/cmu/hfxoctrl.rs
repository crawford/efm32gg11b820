#[doc = "Reader of register HFXOCTRL"]
pub type R = crate::R<u32, super::HFXOCTRL>;
#[doc = "Writer for register HFXOCTRL"]
pub type W = crate::W<u32, super::HFXOCTRL>;
#[doc = "Register HFXOCTRL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::HFXOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    ACBUFEXTCLK,
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    DCBUFEXTCLK,
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    DIGEXTCLK,
}
impl crate::ToBits<u8> for MODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE_A::XTAL => 0,
            MODE_A::ACBUFEXTCLK => 1,
            MODE_A::DCBUFEXTCLK => 2,
            MODE_A::DIGEXTCLK => 3,
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
            0 => MODE_A::XTAL,
            1 => MODE_A::ACBUFEXTCLK,
            2 => MODE_A::DCBUFEXTCLK,
            3 => MODE_A::DIGEXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `ACBUFEXTCLK`"]
    #[inline(always)]
    pub fn is_acbufextclk(&self) -> bool {
        *self == MODE_A::ACBUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DCBUFEXTCLK`"]
    #[inline(always)]
    pub fn is_dcbufextclk(&self) -> bool {
        *self == MODE_A::DCBUFEXTCLK
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
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 MHz - 50 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn acbufextclk(self) -> &'a mut W {
        self.variant(MODE_A::ACBUFEXTCLK)
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn dcbufextclk(self) -> &'a mut W {
        self.variant(MODE_A::DCBUFEXTCLK)
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODE_A::DIGEXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `HFXOX2EN`"]
pub type HFXOX2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXOX2EN`"]
pub struct HFXOX2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOX2EN_W<'a> {
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
#[doc = "Possible values of the field `PEAKDETMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETMODE_A {
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    ONCECMD,
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    AUTOCMD,
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    CMD,
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL,
}
impl crate::ToBits<u8> for PEAKDETMODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PEAKDETMODE_A::ONCECMD => 0,
            PEAKDETMODE_A::AUTOCMD => 1,
            PEAKDETMODE_A::CMD => 2,
            PEAKDETMODE_A::MANUAL => 3,
        }
    }
}
#[doc = "Reader of field `PEAKDETMODE`"]
pub type PEAKDETMODE_R = crate::R<u8, PEAKDETMODE_A>;
impl PEAKDETMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEAKDETMODE_A {
        match self.bits {
            0 => PEAKDETMODE_A::ONCECMD,
            1 => PEAKDETMODE_A::AUTOCMD,
            2 => PEAKDETMODE_A::CMD,
            3 => PEAKDETMODE_A::MANUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONCECMD`"]
    #[inline(always)]
    pub fn is_oncecmd(&self) -> bool {
        *self == PEAKDETMODE_A::ONCECMD
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETMODE_A::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETMODE_A::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETMODE_A::MANUAL
    }
}
#[doc = "Write proxy for field `PEAKDETMODE`"]
pub struct PEAKDETMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEAKDETMODE_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn oncecmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::ONCECMD)
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::MANUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `LFTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFTIMEOUT_A {
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
impl crate::ToBits<u8> for LFTIMEOUT_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LFTIMEOUT_A::_0CYCLES => 0,
            LFTIMEOUT_A::_2CYCLES => 1,
            LFTIMEOUT_A::_4CYCLES => 2,
            LFTIMEOUT_A::_16CYCLES => 3,
            LFTIMEOUT_A::_32CYCLES => 4,
            LFTIMEOUT_A::_64CYCLES => 5,
            LFTIMEOUT_A::_1KCYCLES => 6,
            LFTIMEOUT_A::_4KCYCLES => 7,
        }
    }
}
#[doc = "Reader of field `LFTIMEOUT`"]
pub type LFTIMEOUT_R = crate::R<u8, LFTIMEOUT_A>;
impl LFTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Write proxy for field `LFTIMEOUT`"]
pub struct LFTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFTIMEOUT_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `AUTOSTARTEM0EM1`"]
pub type AUTOSTARTEM0EM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSTARTEM0EM1`"]
pub struct AUTOSTARTEM0EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTARTEM0EM1_W<'a> {
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
#[doc = "Reader of field `AUTOSTARTSELEM0EM1`"]
pub type AUTOSTARTSELEM0EM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSTARTSELEM0EM1`"]
pub struct AUTOSTARTSELEM0EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTARTSELEM0EM1_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    pub fn hfxox2en(&self) -> HFXOX2EN_R {
        HFXOX2EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&self) -> PEAKDETMODE_R {
        PEAKDETMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Enable Double Frequency on HFXOX2 Clock (compared to HFXO Clock)"]
    #[inline(always)]
    pub fn hfxox2en(&mut self) -> HFXOX2EN_W {
        HFXOX2EN_W { w: self }
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&mut self) -> PEAKDETMODE_W {
        PEAKDETMODE_W { w: self }
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W {
        LFTIMEOUT_W { w: self }
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W {
        AUTOSTARTEM0EM1_W { w: self }
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W {
        AUTOSTARTSELEM0EM1_W { w: self }
    }
}
