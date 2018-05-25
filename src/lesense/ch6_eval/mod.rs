#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH6_EVAL {
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
pub struct COMPTHRESR {
    bits: u16,
}
impl COMPTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPR {
    bits: bool,
}
impl COMPR {
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
pub struct DECODER {
    bits: bool,
}
impl DECODER {
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
#[doc = "Possible values of the field `STRSAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRSAMPLER {
    #[doc = "Nothing will be stored in the result buffer."]
    DISABLE,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    DATA,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    DATASRC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STRSAMPLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STRSAMPLER::DISABLE => 0,
            STRSAMPLER::DATA => 1,
            STRSAMPLER::DATASRC => 2,
            STRSAMPLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STRSAMPLER {
        match value {
            0 => STRSAMPLER::DISABLE,
            1 => STRSAMPLER::DATA,
            2 => STRSAMPLER::DATASRC,
            i => STRSAMPLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == STRSAMPLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == STRSAMPLER::DATA
    }
    #[doc = "Checks if the value of the field is `DATASRC`"]
    #[inline]
    pub fn is_datasrc(&self) -> bool {
        *self == STRSAMPLER::DATASRC
    }
}
#[doc = r" Value of the field"]
pub struct SCANRESINVR {
    bits: bool,
}
impl SCANRESINVR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    THRES,
    #[doc = "Sliding window is used to evaluate sensor result"]
    SLIDINGWIN,
    #[doc = "Step detection is used to evaluate sensor result"]
    STEPDET,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::THRES => 0,
            MODER::SLIDINGWIN => 1,
            MODER::STEPDET => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::THRES,
            1 => MODER::SLIDINGWIN,
            2 => MODER::STEPDET,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `THRES`"]
    #[inline]
    pub fn is_thres(&self) -> bool {
        *self == MODER::THRES
    }
    #[doc = "Checks if the value of the field is `SLIDINGWIN`"]
    #[inline]
    pub fn is_slidingwin(&self) -> bool {
        *self == MODER::SLIDINGWIN
    }
    #[doc = "Checks if the value of the field is `STEPDET`"]
    #[inline]
    pub fn is_stepdet(&self) -> bool {
        *self == MODER::STEPDET
    }
}
#[doc = r" Proxy"]
pub struct _COMPTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPTHRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DECODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DECODEW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STRSAMPLE`"]
pub enum STRSAMPLEW {
    #[doc = "Nothing will be stored in the result buffer."]
    DISABLE,
    #[doc = "The sensor sample data will be stored in the result buffer."]
    DATA,
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    DATASRC,
}
impl STRSAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STRSAMPLEW::DISABLE => 0,
            STRSAMPLEW::DATA => 1,
            STRSAMPLEW::DATASRC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRSAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _STRSAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRSAMPLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(STRSAMPLEW::DISABLE)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline]
    pub fn data(self) -> &'a mut W {
        self.variant(STRSAMPLEW::DATA)
    }
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    #[inline]
    pub fn datasrc(self) -> &'a mut W {
        self.variant(STRSAMPLEW::DATASRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCANRESINVW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANRESINVW<'a> {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    THRES,
    #[doc = "Sliding window is used to evaluate sensor result"]
    SLIDINGWIN,
    #[doc = "Step detection is used to evaluate sensor result"]
    STEPDET,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::THRES => 0,
            MODEW::SLIDINGWIN => 1,
            MODEW::STEPDET => 2,
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
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline]
    pub fn thres(self) -> &'a mut W {
        self.variant(MODEW::THRES)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline]
    pub fn slidingwin(self) -> &'a mut W {
        self.variant(MODEW::SLIDINGWIN)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline]
    pub fn stepdet(self) -> &'a mut W {
        self.variant(MODEW::STEPDET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline]
    pub fn compthres(&self) -> COMPTHRESR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        COMPTHRESR { bits }
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline]
    pub fn comp(&self) -> COMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPR { bits }
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline]
    pub fn decode(&self) -> DECODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECODER { bits }
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline]
    pub fn strsample(&self) -> STRSAMPLER {
        STRSAMPLER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline]
    pub fn scanresinv(&self) -> SCANRESINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANRESINVR { bits }
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline]
    pub fn compthres(&mut self) -> _COMPTHRESW {
        _COMPTHRESW { w: self }
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline]
    pub fn comp(&mut self) -> _COMPW {
        _COMPW { w: self }
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline]
    pub fn decode(&mut self) -> _DECODEW {
        _DECODEW { w: self }
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline]
    pub fn strsample(&mut self) -> _STRSAMPLEW {
        _STRSAMPLEW { w: self }
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline]
    pub fn scanresinv(&mut self) -> _SCANRESINVW {
        _SCANRESINVW { w: self }
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
