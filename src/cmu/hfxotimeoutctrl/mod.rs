#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOTIMEOUTCTRL {
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
#[doc = "Possible values of the field `STARTUPTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPTIMEOUTR {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTUPTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTUPTIMEOUTR::_2CYCLES => 0,
            STARTUPTIMEOUTR::_4CYCLES => 1,
            STARTUPTIMEOUTR::_16CYCLES => 2,
            STARTUPTIMEOUTR::_32CYCLES => 3,
            STARTUPTIMEOUTR::_64CYCLES => 4,
            STARTUPTIMEOUTR::_128CYCLES => 5,
            STARTUPTIMEOUTR::_256CYCLES => 6,
            STARTUPTIMEOUTR::_1KCYCLES => 7,
            STARTUPTIMEOUTR::_2KCYCLES => 8,
            STARTUPTIMEOUTR::_4KCYCLES => 9,
            STARTUPTIMEOUTR::_8KCYCLES => 10,
            STARTUPTIMEOUTR::_16KCYCLES => 11,
            STARTUPTIMEOUTR::_32KCYCLES => 12,
            STARTUPTIMEOUTR::_64KCYCLES => 13,
            STARTUPTIMEOUTR::_128KCYCLES => 14,
            STARTUPTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPTIMEOUTR {
        match value {
            0 => STARTUPTIMEOUTR::_2CYCLES,
            1 => STARTUPTIMEOUTR::_4CYCLES,
            2 => STARTUPTIMEOUTR::_16CYCLES,
            3 => STARTUPTIMEOUTR::_32CYCLES,
            4 => STARTUPTIMEOUTR::_64CYCLES,
            5 => STARTUPTIMEOUTR::_128CYCLES,
            6 => STARTUPTIMEOUTR::_256CYCLES,
            7 => STARTUPTIMEOUTR::_1KCYCLES,
            8 => STARTUPTIMEOUTR::_2KCYCLES,
            9 => STARTUPTIMEOUTR::_4KCYCLES,
            10 => STARTUPTIMEOUTR::_8KCYCLES,
            11 => STARTUPTIMEOUTR::_16KCYCLES,
            12 => STARTUPTIMEOUTR::_32KCYCLES,
            13 => STARTUPTIMEOUTR::_64KCYCLES,
            14 => STARTUPTIMEOUTR::_128KCYCLES,
            i => STARTUPTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_32KCYCLES
    }
    #[doc = "Checks if the value of the field is `_64KCYCLES`"]
    #[inline]
    pub fn is_64kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_64KCYCLES
    }
    #[doc = "Checks if the value of the field is `_128KCYCLES`"]
    #[inline]
    pub fn is_128kcycles(&self) -> bool {
        *self == STARTUPTIMEOUTR::_128KCYCLES
    }
}
#[doc = "Possible values of the field `STEADYTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEADYTIMEOUTR {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STEADYTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STEADYTIMEOUTR::_2CYCLES => 0,
            STEADYTIMEOUTR::_4CYCLES => 1,
            STEADYTIMEOUTR::_16CYCLES => 2,
            STEADYTIMEOUTR::_32CYCLES => 3,
            STEADYTIMEOUTR::_64CYCLES => 4,
            STEADYTIMEOUTR::_128CYCLES => 5,
            STEADYTIMEOUTR::_256CYCLES => 6,
            STEADYTIMEOUTR::_1KCYCLES => 7,
            STEADYTIMEOUTR::_2KCYCLES => 8,
            STEADYTIMEOUTR::_4KCYCLES => 9,
            STEADYTIMEOUTR::_8KCYCLES => 10,
            STEADYTIMEOUTR::_16KCYCLES => 11,
            STEADYTIMEOUTR::_32KCYCLES => 12,
            STEADYTIMEOUTR::_64KCYCLES => 13,
            STEADYTIMEOUTR::_128KCYCLES => 14,
            STEADYTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STEADYTIMEOUTR {
        match value {
            0 => STEADYTIMEOUTR::_2CYCLES,
            1 => STEADYTIMEOUTR::_4CYCLES,
            2 => STEADYTIMEOUTR::_16CYCLES,
            3 => STEADYTIMEOUTR::_32CYCLES,
            4 => STEADYTIMEOUTR::_64CYCLES,
            5 => STEADYTIMEOUTR::_128CYCLES,
            6 => STEADYTIMEOUTR::_256CYCLES,
            7 => STEADYTIMEOUTR::_1KCYCLES,
            8 => STEADYTIMEOUTR::_2KCYCLES,
            9 => STEADYTIMEOUTR::_4KCYCLES,
            10 => STEADYTIMEOUTR::_8KCYCLES,
            11 => STEADYTIMEOUTR::_16KCYCLES,
            12 => STEADYTIMEOUTR::_32KCYCLES,
            13 => STEADYTIMEOUTR::_64KCYCLES,
            14 => STEADYTIMEOUTR::_128KCYCLES,
            i => STEADYTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_32KCYCLES
    }
    #[doc = "Checks if the value of the field is `_64KCYCLES`"]
    #[inline]
    pub fn is_64kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_64KCYCLES
    }
    #[doc = "Checks if the value of the field is `_128KCYCLES`"]
    #[inline]
    pub fn is_128kcycles(&self) -> bool {
        *self == STEADYTIMEOUTR::_128KCYCLES
    }
}
#[doc = "Possible values of the field `PEAKDETTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETTIMEOUTR {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PEAKDETTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEAKDETTIMEOUTR::_2CYCLES => 0,
            PEAKDETTIMEOUTR::_4CYCLES => 1,
            PEAKDETTIMEOUTR::_16CYCLES => 2,
            PEAKDETTIMEOUTR::_32CYCLES => 3,
            PEAKDETTIMEOUTR::_64CYCLES => 4,
            PEAKDETTIMEOUTR::_128CYCLES => 5,
            PEAKDETTIMEOUTR::_256CYCLES => 6,
            PEAKDETTIMEOUTR::_1KCYCLES => 7,
            PEAKDETTIMEOUTR::_2KCYCLES => 8,
            PEAKDETTIMEOUTR::_4KCYCLES => 9,
            PEAKDETTIMEOUTR::_8KCYCLES => 10,
            PEAKDETTIMEOUTR::_16KCYCLES => 11,
            PEAKDETTIMEOUTR::_32KCYCLES => 12,
            PEAKDETTIMEOUTR::_64KCYCLES => 13,
            PEAKDETTIMEOUTR::_128KCYCLES => 14,
            PEAKDETTIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEAKDETTIMEOUTR {
        match value {
            0 => PEAKDETTIMEOUTR::_2CYCLES,
            1 => PEAKDETTIMEOUTR::_4CYCLES,
            2 => PEAKDETTIMEOUTR::_16CYCLES,
            3 => PEAKDETTIMEOUTR::_32CYCLES,
            4 => PEAKDETTIMEOUTR::_64CYCLES,
            5 => PEAKDETTIMEOUTR::_128CYCLES,
            6 => PEAKDETTIMEOUTR::_256CYCLES,
            7 => PEAKDETTIMEOUTR::_1KCYCLES,
            8 => PEAKDETTIMEOUTR::_2KCYCLES,
            9 => PEAKDETTIMEOUTR::_4KCYCLES,
            10 => PEAKDETTIMEOUTR::_8KCYCLES,
            11 => PEAKDETTIMEOUTR::_16KCYCLES,
            12 => PEAKDETTIMEOUTR::_32KCYCLES,
            13 => PEAKDETTIMEOUTR::_64KCYCLES,
            14 => PEAKDETTIMEOUTR::_128KCYCLES,
            i => PEAKDETTIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline]
    pub fn is_2kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline]
    pub fn is_4kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline]
    pub fn is_8kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_32KCYCLES
    }
    #[doc = "Checks if the value of the field is `_64KCYCLES`"]
    #[inline]
    pub fn is_64kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_64KCYCLES
    }
    #[doc = "Checks if the value of the field is `_128KCYCLES`"]
    #[inline]
    pub fn is_128kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUTR::_128KCYCLES
    }
}
#[doc = "Values that can be written to the field `STARTUPTIMEOUT`"]
pub enum STARTUPTIMEOUTW {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
}
impl STARTUPTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPTIMEOUTW::_2CYCLES => 0,
            STARTUPTIMEOUTW::_4CYCLES => 1,
            STARTUPTIMEOUTW::_16CYCLES => 2,
            STARTUPTIMEOUTW::_32CYCLES => 3,
            STARTUPTIMEOUTW::_64CYCLES => 4,
            STARTUPTIMEOUTW::_128CYCLES => 5,
            STARTUPTIMEOUTW::_256CYCLES => 6,
            STARTUPTIMEOUTW::_1KCYCLES => 7,
            STARTUPTIMEOUTW::_2KCYCLES => 8,
            STARTUPTIMEOUTW::_4KCYCLES => 9,
            STARTUPTIMEOUTW::_8KCYCLES => 10,
            STARTUPTIMEOUTW::_16KCYCLES => 11,
            STARTUPTIMEOUTW::_32KCYCLES => 12,
            STARTUPTIMEOUTW::_64KCYCLES => 13,
            STARTUPTIMEOUTW::_128KCYCLES => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTUPTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTUPTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_64CYCLES)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_128CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_32KCYCLES)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline]
    pub fn _64kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_64KCYCLES)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline]
    pub fn _128kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUTW::_128KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STEADYTIMEOUT`"]
pub enum STEADYTIMEOUTW {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
}
impl STEADYTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STEADYTIMEOUTW::_2CYCLES => 0,
            STEADYTIMEOUTW::_4CYCLES => 1,
            STEADYTIMEOUTW::_16CYCLES => 2,
            STEADYTIMEOUTW::_32CYCLES => 3,
            STEADYTIMEOUTW::_64CYCLES => 4,
            STEADYTIMEOUTW::_128CYCLES => 5,
            STEADYTIMEOUTW::_256CYCLES => 6,
            STEADYTIMEOUTW::_1KCYCLES => 7,
            STEADYTIMEOUTW::_2KCYCLES => 8,
            STEADYTIMEOUTW::_4KCYCLES => 9,
            STEADYTIMEOUTW::_8KCYCLES => 10,
            STEADYTIMEOUTW::_16KCYCLES => 11,
            STEADYTIMEOUTW::_32KCYCLES => 12,
            STEADYTIMEOUTW::_64KCYCLES => 13,
            STEADYTIMEOUTW::_128KCYCLES => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STEADYTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _STEADYTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STEADYTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_64CYCLES)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_128CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_32KCYCLES)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline]
    pub fn _64kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_64KCYCLES)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline]
    pub fn _128kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUTW::_128KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEAKDETTIMEOUT`"]
pub enum PEAKDETTIMEOUTW {
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
    #[doc = "Timeout period of 128 cycles"]
    _128CYCLES,
    #[doc = "Timeout period of 256 cycles"]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles"]
    _1KCYCLES,
    #[doc = "Timeout period of 2048 cycles"]
    _2KCYCLES,
    #[doc = "Timeout period of 4096 cycles"]
    _4KCYCLES,
    #[doc = "Timeout period of 8192 cycles"]
    _8KCYCLES,
    #[doc = "Timeout period of 16384 cycles"]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles"]
    _32KCYCLES,
    #[doc = "Timeout period of 65536 cycles"]
    _64KCYCLES,
    #[doc = "Timeout period of 131072 cycles"]
    _128KCYCLES,
}
impl PEAKDETTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PEAKDETTIMEOUTW::_2CYCLES => 0,
            PEAKDETTIMEOUTW::_4CYCLES => 1,
            PEAKDETTIMEOUTW::_16CYCLES => 2,
            PEAKDETTIMEOUTW::_32CYCLES => 3,
            PEAKDETTIMEOUTW::_64CYCLES => 4,
            PEAKDETTIMEOUTW::_128CYCLES => 5,
            PEAKDETTIMEOUTW::_256CYCLES => 6,
            PEAKDETTIMEOUTW::_1KCYCLES => 7,
            PEAKDETTIMEOUTW::_2KCYCLES => 8,
            PEAKDETTIMEOUTW::_4KCYCLES => 9,
            PEAKDETTIMEOUTW::_8KCYCLES => 10,
            PEAKDETTIMEOUTW::_16KCYCLES => 11,
            PEAKDETTIMEOUTW::_32KCYCLES => 12,
            PEAKDETTIMEOUTW::_64KCYCLES => 13,
            PEAKDETTIMEOUTW::_128KCYCLES => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEAKDETTIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_64CYCLES)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_128CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_32KCYCLES)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline]
    pub fn _64kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_64KCYCLES)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline]
    pub fn _128kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUTW::_128KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline]
    pub fn startuptimeout(&self) -> STARTUPTIMEOUTR {
        STARTUPTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline]
    pub fn steadytimeout(&self) -> STEADYTIMEOUTR {
        STEADYTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline]
    pub fn peakdettimeout(&self) -> PEAKDETTIMEOUTR {
        PEAKDETTIMEOUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 53326 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline]
    pub fn startuptimeout(&mut self) -> _STARTUPTIMEOUTW {
        _STARTUPTIMEOUTW { w: self }
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline]
    pub fn steadytimeout(&mut self) -> _STEADYTIMEOUTW {
        _STEADYTIMEOUTW { w: self }
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline]
    pub fn peakdettimeout(&mut self) -> _PEAKDETTIMEOUTW {
        _PEAKDETTIMEOUTW { w: self }
    }
}
