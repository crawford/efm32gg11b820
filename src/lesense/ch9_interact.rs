#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH9_INTERACT {
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
pub struct THRESR {
    bits: u16,
}
impl THRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLER {
    #[doc = "Counter output will be used in evaluation"]
    ACMPCOUNT,
    #[doc = "ACMP output will be used in evaluation"]
    ACMP,
    #[doc = "ADC output will be used in evaluation"]
    ADC,
    #[doc = "Differential ADC output will be used in evaluation"]
    ADCDIFF,
}
impl SAMPLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPLER::ACMPCOUNT => 0,
            SAMPLER::ACMP => 1,
            SAMPLER::ADC => 2,
            SAMPLER::ADCDIFF => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPLER {
        match value {
            0 => SAMPLER::ACMPCOUNT,
            1 => SAMPLER::ACMP,
            2 => SAMPLER::ADC,
            3 => SAMPLER::ADCDIFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACMPCOUNT`"]
    #[inline]
    pub fn is_acmpcount(&self) -> bool {
        *self == SAMPLER::ACMPCOUNT
    }
    #[doc = "Checks if the value of the field is `ACMP`"]
    #[inline]
    pub fn is_acmp(&self) -> bool {
        *self == SAMPLER::ACMP
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline]
    pub fn is_adc(&self) -> bool {
        *self == SAMPLER::ADC
    }
    #[doc = "Checks if the value of the field is `ADCDIFF`"]
    #[inline]
    pub fn is_adcdiff(&self) -> bool {
        *self == SAMPLER::ADCDIFF
    }
}
#[doc = "Possible values of the field `SETIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETIFR {
    #[doc = "No interrupt is generated"]
    NONE,
    #[doc = "Set interrupt flag if the sensor triggers."]
    LEVEL,
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    POSEDGE,
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    NEGEDGE,
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    BOTHEDGES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETIFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETIFR::NONE => 0,
            SETIFR::LEVEL => 1,
            SETIFR::POSEDGE => 2,
            SETIFR::NEGEDGE => 3,
            SETIFR::BOTHEDGES => 4,
            SETIFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETIFR {
        match value {
            0 => SETIFR::NONE,
            1 => SETIFR::LEVEL,
            2 => SETIFR::POSEDGE,
            3 => SETIFR::NEGEDGE,
            4 => SETIFR::BOTHEDGES,
            i => SETIFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SETIFR::NONE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == SETIFR::LEVEL
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline]
    pub fn is_posedge(&self) -> bool {
        *self == SETIFR::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == SETIFR::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_bothedges(&self) -> bool {
        *self == SETIFR::BOTHEDGES
    }
}
#[doc = "Possible values of the field `EXMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXMODER {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "Push Pull, GPIO is driven high"]
    HIGH,
    #[doc = "Push Pull, GPIO is driven low"]
    LOW,
    #[doc = "VDAC output"]
    DACOUT,
}
impl EXMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXMODER::DISABLE => 0,
            EXMODER::HIGH => 1,
            EXMODER::LOW => 2,
            EXMODER::DACOUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXMODER {
        match value {
            0 => EXMODER::DISABLE,
            1 => EXMODER::HIGH,
            2 => EXMODER::LOW,
            3 => EXMODER::DACOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EXMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == EXMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == EXMODER::LOW
    }
    #[doc = "Checks if the value of the field is `DACOUT`"]
    #[inline]
    pub fn is_dacout(&self) -> bool {
        *self == EXMODER::DACOUT
    }
}
#[doc = r" Value of the field"]
pub struct EXCLKR {
    bits: bool,
}
impl EXCLKR {
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
pub struct SAMPLECLKR {
    bits: bool,
}
impl SAMPLECLKR {
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
pub struct ALTEXR {
    bits: bool,
}
impl ALTEXR {
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
pub struct _THRESW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAMPLE`"]
pub enum SAMPLEW {
    #[doc = "Counter output will be used in evaluation"]
    ACMPCOUNT,
    #[doc = "ACMP output will be used in evaluation"]
    ACMP,
    #[doc = "ADC output will be used in evaluation"]
    ADC,
    #[doc = "Differential ADC output will be used in evaluation"]
    ADCDIFF,
}
impl SAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPLEW::ACMPCOUNT => 0,
            SAMPLEW::ACMP => 1,
            SAMPLEW::ADC => 2,
            SAMPLEW::ADCDIFF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter output will be used in evaluation"]
    #[inline]
    pub fn acmpcount(self) -> &'a mut W {
        self.variant(SAMPLEW::ACMPCOUNT)
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline]
    pub fn acmp(self) -> &'a mut W {
        self.variant(SAMPLEW::ACMP)
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline]
    pub fn adc(self) -> &'a mut W {
        self.variant(SAMPLEW::ADC)
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline]
    pub fn adcdiff(self) -> &'a mut W {
        self.variant(SAMPLEW::ADCDIFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETIF`"]
pub enum SETIFW {
    #[doc = "No interrupt is generated"]
    NONE,
    #[doc = "Set interrupt flag if the sensor triggers."]
    LEVEL,
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    POSEDGE,
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    NEGEDGE,
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    BOTHEDGES,
}
impl SETIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETIFW::NONE => 0,
            SETIFW::LEVEL => 1,
            SETIFW::POSEDGE => 2,
            SETIFW::NEGEDGE => 3,
            SETIFW::BOTHEDGES => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SETIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETIFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No interrupt is generated"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SETIFW::NONE)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(SETIFW::LEVEL)
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline]
    pub fn posedge(self) -> &'a mut W {
        self.variant(SETIFW::POSEDGE)
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(SETIFW::NEGEDGE)
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(SETIFW::BOTHEDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXMODE`"]
pub enum EXMODEW {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "Push Pull, GPIO is driven high"]
    HIGH,
    #[doc = "Push Pull, GPIO is driven low"]
    LOW,
    #[doc = "VDAC output"]
    DACOUT,
}
impl EXMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXMODEW::DISABLE => 0,
            EXMODEW::HIGH => 1,
            EXMODEW::LOW => 2,
            EXMODEW::DACOUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXMODEW::DISABLE)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(EXMODEW::HIGH)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(EXMODEW::LOW)
    }
    #[doc = "VDAC output"]
    #[inline]
    pub fn dacout(self) -> &'a mut W {
        self.variant(EXMODEW::DACOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXCLKW<'a> {
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
pub struct _SAMPLECLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLECLKW<'a> {
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
pub struct _ALTEXW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTEXW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline]
    pub fn thres(&self) -> THRESR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        THRESR { bits }
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        SAMPLER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline]
    pub fn setif(&self) -> SETIFR {
        SETIFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline]
    pub fn exmode(&self) -> EXMODER {
        EXMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline]
    pub fn exclk(&self) -> EXCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXCLKR { bits }
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline]
    pub fn sampleclk(&self) -> SAMPLECLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAMPLECLKR { bits }
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline]
    pub fn altex(&self) -> ALTEXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTEXR { bits }
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
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline]
    pub fn thres(&mut self) -> _THRESW {
        _THRESW { w: self }
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline]
    pub fn sample(&mut self) -> _SAMPLEW {
        _SAMPLEW { w: self }
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline]
    pub fn setif(&mut self) -> _SETIFW {
        _SETIFW { w: self }
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline]
    pub fn exmode(&mut self) -> _EXMODEW {
        _EXMODEW { w: self }
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline]
    pub fn exclk(&mut self) -> _EXCLKW {
        _EXCLKW { w: self }
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline]
    pub fn sampleclk(&mut self) -> _SAMPLECLKW {
        _SAMPLECLKW { w: self }
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline]
    pub fn altex(&mut self) -> _ALTEXW {
        _ALTEXW { w: self }
    }
}
