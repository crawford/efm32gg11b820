#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DISPCTRL {
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
#[doc = "Possible values of the field `MUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXR {
    #[doc = "Static"]
    STATIC,
    #[doc = "Duplex"]
    DUPLEX,
    #[doc = "Triplex"]
    TRIPLEX,
    #[doc = "Quadruplex"]
    QUADRUPLEX,
    #[doc = "Sextaplex"]
    SEXTAPLEX,
    #[doc = "Octaplex"]
    OCTAPLEX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXR::STATIC => 0,
            MUXR::DUPLEX => 1,
            MUXR::TRIPLEX => 2,
            MUXR::QUADRUPLEX => 3,
            MUXR::SEXTAPLEX => 5,
            MUXR::OCTAPLEX => 7,
            MUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXR {
        match value {
            0 => MUXR::STATIC,
            1 => MUXR::DUPLEX,
            2 => MUXR::TRIPLEX,
            3 => MUXR::QUADRUPLEX,
            5 => MUXR::SEXTAPLEX,
            7 => MUXR::OCTAPLEX,
            i => MUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == MUXR::STATIC
    }
    #[doc = "Checks if the value of the field is `DUPLEX`"]
    #[inline]
    pub fn is_duplex(&self) -> bool {
        *self == MUXR::DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRIPLEX`"]
    #[inline]
    pub fn is_triplex(&self) -> bool {
        *self == MUXR::TRIPLEX
    }
    #[doc = "Checks if the value of the field is `QUADRUPLEX`"]
    #[inline]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUXR::QUADRUPLEX
    }
    #[doc = "Checks if the value of the field is `SEXTAPLEX`"]
    #[inline]
    pub fn is_sextaplex(&self) -> bool {
        *self == MUXR::SEXTAPLEX
    }
    #[doc = "Checks if the value of the field is `OCTAPLEX`"]
    #[inline]
    pub fn is_octaplex(&self) -> bool {
        *self == MUXR::OCTAPLEX
    }
}
#[doc = r" Value of the field"]
pub struct WAVER {
    bits: bool,
}
impl WAVER {
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
pub struct CONTRASTR {
    bits: u8,
}
impl CONTRASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CHGRDST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHGRDSTR {
    #[doc = "Disable charge redistribution."]
    DISABLE,
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    ONE,
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    TWO,
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    THREE,
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    FOUR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHGRDSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHGRDSTR::DISABLE => 0,
            CHGRDSTR::ONE => 1,
            CHGRDSTR::TWO => 2,
            CHGRDSTR::THREE => 3,
            CHGRDSTR::FOUR => 4,
            CHGRDSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHGRDSTR {
        match value {
            0 => CHGRDSTR::DISABLE,
            1 => CHGRDSTR::ONE,
            2 => CHGRDSTR::TWO,
            3 => CHGRDSTR::THREE,
            4 => CHGRDSTR::FOUR,
            i => CHGRDSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHGRDSTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CHGRDSTR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == CHGRDSTR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == CHGRDSTR::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == CHGRDSTR::FOUR
    }
}
#[doc = "Possible values of the field `BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASR {
    #[doc = "Static"]
    STATIC,
    #[doc = "1/2 Bias"]
    ONEHALF,
    #[doc = "1/3 Bias"]
    ONETHIRD,
    #[doc = "1/4 Bias"]
    ONEFOURTH,
}
impl BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BIASR::STATIC => 0,
            BIASR::ONEHALF => 1,
            BIASR::ONETHIRD => 2,
            BIASR::ONEFOURTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BIASR {
        match value {
            0 => BIASR::STATIC,
            1 => BIASR::ONEHALF,
            2 => BIASR::ONETHIRD,
            3 => BIASR::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == BIASR::STATIC
    }
    #[doc = "Checks if the value of the field is `ONEHALF`"]
    #[inline]
    pub fn is_onehalf(&self) -> bool {
        *self == BIASR::ONEHALF
    }
    #[doc = "Checks if the value of the field is `ONETHIRD`"]
    #[inline]
    pub fn is_onethird(&self) -> bool {
        *self == BIASR::ONETHIRD
    }
    #[doc = "Checks if the value of the field is `ONEFOURTH`"]
    #[inline]
    pub fn is_onefourth(&self) -> bool {
        *self == BIASR::ONEFOURTH
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    NOEXTCAP,
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN,
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    CPINTOSC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NOEXTCAP => 0,
            MODER::STEPDOWN => 1,
            MODER::CPINTOSC => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NOEXTCAP,
            1 => MODER::STEPDOWN,
            2 => MODER::CPINTOSC,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOEXTCAP`"]
    #[inline]
    pub fn is_noextcap(&self) -> bool {
        *self == MODER::NOEXTCAP
    }
    #[doc = "Checks if the value of the field is `STEPDOWN`"]
    #[inline]
    pub fn is_stepdown(&self) -> bool {
        *self == MODER::STEPDOWN
    }
    #[doc = "Checks if the value of the field is `CPINTOSC`"]
    #[inline]
    pub fn is_cpintosc(&self) -> bool {
        *self == MODER::CPINTOSC
    }
}
#[doc = "Values that can be written to the field `MUX`"]
pub enum MUXW {
    #[doc = "Static"]
    STATIC,
    #[doc = "Duplex"]
    DUPLEX,
    #[doc = "Triplex"]
    TRIPLEX,
    #[doc = "Quadruplex"]
    QUADRUPLEX,
    #[doc = "Sextaplex"]
    SEXTAPLEX,
    #[doc = "Octaplex"]
    OCTAPLEX,
}
impl MUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXW::STATIC => 0,
            MUXW::DUPLEX => 1,
            MUXW::TRIPLEX => 2,
            MUXW::QUADRUPLEX => 3,
            MUXW::SEXTAPLEX => 5,
            MUXW::OCTAPLEX => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Static"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(MUXW::STATIC)
    }
    #[doc = "Duplex"]
    #[inline]
    pub fn duplex(self) -> &'a mut W {
        self.variant(MUXW::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline]
    pub fn triplex(self) -> &'a mut W {
        self.variant(MUXW::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline]
    pub fn quadruplex(self) -> &'a mut W {
        self.variant(MUXW::QUADRUPLEX)
    }
    #[doc = "Sextaplex"]
    #[inline]
    pub fn sextaplex(self) -> &'a mut W {
        self.variant(MUXW::SEXTAPLEX)
    }
    #[doc = "Octaplex"]
    #[inline]
    pub fn octaplex(self) -> &'a mut W {
        self.variant(MUXW::OCTAPLEX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEW<'a> {
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
pub struct _CONTRASTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTRASTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHGRDST`"]
pub enum CHGRDSTW {
    #[doc = "Disable charge redistribution."]
    DISABLE,
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    ONE,
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    TWO,
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    THREE,
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    FOUR,
}
impl CHGRDSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHGRDSTW::DISABLE => 0,
            CHGRDSTW::ONE => 1,
            CHGRDSTW::TWO => 2,
            CHGRDSTW::THREE => 3,
            CHGRDSTW::FOUR => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHGRDSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CHGRDSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHGRDSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable charge redistribution."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHGRDSTW::DISABLE)
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CHGRDSTW::ONE)
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(CHGRDSTW::TWO)
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(CHGRDSTW::THREE)
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(CHGRDSTW::FOUR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BIAS`"]
pub enum BIASW {
    #[doc = "Static"]
    STATIC,
    #[doc = "1/2 Bias"]
    ONEHALF,
    #[doc = "1/3 Bias"]
    ONETHIRD,
    #[doc = "1/4 Bias"]
    ONEFOURTH,
}
impl BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BIASW::STATIC => 0,
            BIASW::ONEHALF => 1,
            BIASW::ONETHIRD => 2,
            BIASW::ONEFOURTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Static"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIASW::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline]
    pub fn onehalf(self) -> &'a mut W {
        self.variant(BIASW::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline]
    pub fn onethird(self) -> &'a mut W {
        self.variant(BIASW::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline]
    pub fn onefourth(self) -> &'a mut W {
        self.variant(BIASW::ONEFOURTH)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    NOEXTCAP,
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN,
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    CPINTOSC,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NOEXTCAP => 0,
            MODEW::STEPDOWN => 1,
            MODEW::CPINTOSC => 2,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline]
    pub fn noextcap(self) -> &'a mut W {
        self.variant(MODEW::NOEXTCAP)
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline]
    pub fn stepdown(self) -> &'a mut W {
        self.variant(MODEW::STEPDOWN)
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline]
    pub fn cpintosc(self) -> &'a mut W {
        self.variant(MODEW::CPINTOSC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline]
    pub fn mux(&self) -> MUXR {
        MUXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAVER { bits }
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline]
    pub fn contrast(&self) -> CONTRASTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CONTRASTR { bits }
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline]
    pub fn chgrdst(&self) -> CHGRDSTR {
        CHGRDSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline]
    pub fn bias(&self) -> BIASR {
        BIASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1064704 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline]
    pub fn mux(&mut self) -> _MUXW {
        _MUXW { w: self }
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline]
    pub fn contrast(&mut self) -> _CONTRASTW {
        _CONTRASTW { w: self }
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline]
    pub fn chgrdst(&mut self) -> _CHGRDSTW {
        _CHGRDSTW { w: self }
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline]
    pub fn bias(&mut self) -> _BIASW {
        _BIASW { w: self }
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
