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
#[doc = "Possible values of the field `SCANMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANMODER {
    #[doc = "A new scan is started each time the period counter overflows"]
    PERIODIC,
    #[doc = "A single scan is performed when START in CMD is set"]
    ONESHOT,
    #[doc = "Pulse on PRS channel"]
    PRS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCANMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCANMODER::PERIODIC => 0,
            SCANMODER::ONESHOT => 1,
            SCANMODER::PRS => 2,
            SCANMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCANMODER {
        match value {
            0 => SCANMODER::PERIODIC,
            1 => SCANMODER::ONESHOT,
            2 => SCANMODER::PRS,
            i => SCANMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline]
    pub fn is_periodic(&self) -> bool {
        *self == SCANMODER::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline]
    pub fn is_oneshot(&self) -> bool {
        *self == SCANMODER::ONESHOT
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == SCANMODER::PRS
    }
}
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as input"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as input"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as input"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as input"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as input"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as input"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as input"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as input"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as input"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as input"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as input"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as input"]
    PRSCH23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSELR::PRSCH0 => 0,
            PRSSELR::PRSCH1 => 1,
            PRSSELR::PRSCH2 => 2,
            PRSSELR::PRSCH3 => 3,
            PRSSELR::PRSCH4 => 4,
            PRSSELR::PRSCH5 => 5,
            PRSSELR::PRSCH6 => 6,
            PRSSELR::PRSCH7 => 7,
            PRSSELR::PRSCH8 => 8,
            PRSSELR::PRSCH9 => 9,
            PRSSELR::PRSCH10 => 10,
            PRSSELR::PRSCH11 => 11,
            PRSSELR::PRSCH12 => 12,
            PRSSELR::PRSCH13 => 13,
            PRSSELR::PRSCH14 => 14,
            PRSSELR::PRSCH15 => 15,
            PRSSELR::PRSCH16 => 16,
            PRSSELR::PRSCH17 => 17,
            PRSSELR::PRSCH18 => 18,
            PRSSELR::PRSCH19 => 19,
            PRSSELR::PRSCH20 => 20,
            PRSSELR::PRSCH21 => 21,
            PRSSELR::PRSCH22 => 22,
            PRSSELR::PRSCH23 => 23,
            PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSELR {
        match value {
            0 => PRSSELR::PRSCH0,
            1 => PRSSELR::PRSCH1,
            2 => PRSSELR::PRSCH2,
            3 => PRSSELR::PRSCH3,
            4 => PRSSELR::PRSCH4,
            5 => PRSSELR::PRSCH5,
            6 => PRSSELR::PRSCH6,
            7 => PRSSELR::PRSCH7,
            8 => PRSSELR::PRSCH8,
            9 => PRSSELR::PRSCH9,
            10 => PRSSELR::PRSCH10,
            11 => PRSSELR::PRSCH11,
            12 => PRSSELR::PRSCH12,
            13 => PRSSELR::PRSCH13,
            14 => PRSSELR::PRSCH14,
            15 => PRSSELR::PRSCH15,
            16 => PRSSELR::PRSCH16,
            17 => PRSSELR::PRSCH17,
            18 => PRSSELR::PRSCH18,
            19 => PRSSELR::PRSCH19,
            20 => PRSSELR::PRSCH20,
            21 => PRSSELR::PRSCH21,
            22 => PRSSELR::PRSCH22,
            23 => PRSSELR::PRSCH23,
            i => PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSELR::PRSCH23
    }
}
#[doc = "Possible values of the field `SCANCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANCONFR {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP,
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    INVMAP,
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    TOGGLE,
    #[doc = "The decoder state defines the CONF registers to be used."]
    DECDEF,
}
impl SCANCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCANCONFR::DIRMAP => 0,
            SCANCONFR::INVMAP => 1,
            SCANCONFR::TOGGLE => 2,
            SCANCONFR::DECDEF => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCANCONFR {
        match value {
            0 => SCANCONFR::DIRMAP,
            1 => SCANCONFR::INVMAP,
            2 => SCANCONFR::TOGGLE,
            3 => SCANCONFR::DECDEF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIRMAP`"]
    #[inline]
    pub fn is_dirmap(&self) -> bool {
        *self == SCANCONFR::DIRMAP
    }
    #[doc = "Checks if the value of the field is `INVMAP`"]
    #[inline]
    pub fn is_invmap(&self) -> bool {
        *self == SCANCONFR::INVMAP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == SCANCONFR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `DECDEF`"]
    #[inline]
    pub fn is_decdef(&self) -> bool {
        *self == SCANCONFR::DECDEF
    }
}
#[doc = r" Value of the field"]
pub struct ALTEXMAPR {
    bits: bool,
}
impl ALTEXMAPR {
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
pub struct DUALSAMPLER {
    bits: bool,
}
impl DUALSAMPLER {
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
pub struct BUFOWR {
    bits: bool,
}
impl BUFOWR {
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
pub struct STRSCANRESR {
    bits: bool,
}
impl STRSCANRESR {
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
pub struct BUFIDLR {
    bits: bool,
}
impl BUFIDLR {
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
#[doc = "Possible values of the field `DMAWU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAWUR {
    #[doc = "No DMA wake-up from EM2"]
    DISABLE,
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    BUFDATAV,
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    BUFLEVEL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMAWUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAWUR::DISABLE => 0,
            DMAWUR::BUFDATAV => 1,
            DMAWUR::BUFLEVEL => 2,
            DMAWUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAWUR {
        match value {
            0 => DMAWUR::DISABLE,
            1 => DMAWUR::BUFDATAV,
            2 => DMAWUR::BUFLEVEL,
            i => DMAWUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMAWUR::DISABLE
    }
    #[doc = "Checks if the value of the field is `BUFDATAV`"]
    #[inline]
    pub fn is_bufdatav(&self) -> bool {
        *self == DMAWUR::BUFDATAV
    }
    #[doc = "Checks if the value of the field is `BUFLEVEL`"]
    #[inline]
    pub fn is_buflevel(&self) -> bool {
        *self == DMAWUR::BUFLEVEL
    }
}
#[doc = r" Value of the field"]
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
#[doc = "Values that can be written to the field `SCANMODE`"]
pub enum SCANMODEW {
    #[doc = "A new scan is started each time the period counter overflows"]
    PERIODIC,
    #[doc = "A single scan is performed when START in CMD is set"]
    ONESHOT,
    #[doc = "Pulse on PRS channel"]
    PRS,
}
impl SCANMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCANMODEW::PERIODIC => 0,
            SCANMODEW::ONESHOT => 1,
            SCANMODEW::PRS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCANMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCANMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline]
    pub fn periodic(self) -> &'a mut W {
        self.variant(SCANMODEW::PERIODIC)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(SCANMODEW::ONESHOT)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(SCANMODEW::PRS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as input"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as input"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as input"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as input"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as input"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as input"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as input"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as input"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as input"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as input"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as input"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as input"]
    PRSCH23,
}
impl PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSELW::PRSCH0 => 0,
            PRSSELW::PRSCH1 => 1,
            PRSSELW::PRSCH2 => 2,
            PRSSELW::PRSCH3 => 3,
            PRSSELW::PRSCH4 => 4,
            PRSSELW::PRSCH5 => 5,
            PRSSELW::PRSCH6 => 6,
            PRSSELW::PRSCH7 => 7,
            PRSSELW::PRSCH8 => 8,
            PRSSELW::PRSCH9 => 9,
            PRSSELW::PRSCH10 => 10,
            PRSSELW::PRSCH11 => 11,
            PRSSELW::PRSCH12 => 12,
            PRSSELW::PRSCH13 => 13,
            PRSSELW::PRSCH14 => 14,
            PRSSELW::PRSCH15 => 15,
            PRSSELW::PRSCH16 => 16,
            PRSSELW::PRSCH17 => 17,
            PRSSELW::PRSCH18 => 18,
            PRSSELW::PRSCH19 => 19,
            PRSSELW::PRSCH20 => 20,
            PRSSELW::PRSCH21 => 21,
            PRSSELW::PRSCH22 => 22,
            PRSSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCANCONF`"]
pub enum SCANCONFW {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP,
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    INVMAP,
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    TOGGLE,
    #[doc = "The decoder state defines the CONF registers to be used."]
    DECDEF,
}
impl SCANCONFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCANCONFW::DIRMAP => 0,
            SCANCONFW::INVMAP => 1,
            SCANCONFW::TOGGLE => 2,
            SCANCONFW::DECDEF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCANCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANCONFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCANCONFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline]
    pub fn dirmap(self) -> &'a mut W {
        self.variant(SCANCONFW::DIRMAP)
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline]
    pub fn invmap(self) -> &'a mut W {
        self.variant(SCANCONFW::INVMAP)
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(SCANCONFW::TOGGLE)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline]
    pub fn decdef(self) -> &'a mut W {
        self.variant(SCANCONFW::DECDEF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALTEXMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTEXMAPW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DUALSAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALSAMPLEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUFOWW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFOWW<'a> {
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
pub struct _STRSCANRESW<'a> {
    w: &'a mut W,
}
impl<'a> _STRSCANRESW<'a> {
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
#[doc = r" Proxy"]
pub struct _BUFIDLW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFIDLW<'a> {
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
#[doc = "Values that can be written to the field `DMAWU`"]
pub enum DMAWUW {
    #[doc = "No DMA wake-up from EM2"]
    DISABLE,
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    BUFDATAV,
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    BUFLEVEL,
}
impl DMAWUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAWUW::DISABLE => 0,
            DMAWUW::BUFDATAV => 1,
            DMAWUW::BUFLEVEL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAWUW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAWUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAWUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No DMA wake-up from EM2"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAWUW::DISABLE)
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline]
    pub fn bufdatav(self) -> &'a mut W {
        self.variant(DMAWUW::BUFDATAV)
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline]
    pub fn buflevel(self) -> &'a mut W {
        self.variant(DMAWUW::BUFLEVEL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline]
    pub fn scanmode(&self) -> SCANMODER {
        SCANMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:6 - Scan Start PRS Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline]
    pub fn scanconf(&self) -> SCANCONFR {
        SCANCONFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline]
    pub fn altexmap(&self) -> ALTEXMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTEXMAPR { bits }
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline]
    pub fn dualsample(&self) -> DUALSAMPLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DUALSAMPLER { bits }
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline]
    pub fn bufow(&self) -> BUFOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUFOWR { bits }
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline]
    pub fn strscanres(&self) -> STRSCANRESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STRSCANRESR { bits }
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline]
    pub fn bufidl(&self) -> BUFIDLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUFIDLR { bits }
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline]
    pub fn dmawu(&self) -> DMAWUR {
        DMAWUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
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
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline]
    pub fn scanmode(&mut self) -> _SCANMODEW {
        _SCANMODEW { w: self }
    }
    #[doc = "Bits 2:6 - Scan Start PRS Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline]
    pub fn scanconf(&mut self) -> _SCANCONFW {
        _SCANCONFW { w: self }
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline]
    pub fn altexmap(&mut self) -> _ALTEXMAPW {
        _ALTEXMAPW { w: self }
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline]
    pub fn dualsample(&mut self) -> _DUALSAMPLEW {
        _DUALSAMPLEW { w: self }
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline]
    pub fn bufow(&mut self) -> _BUFOWW {
        _BUFOWW { w: self }
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline]
    pub fn strscanres(&mut self) -> _STRSCANRESW {
        _STRSCANRESW { w: self }
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline]
    pub fn bufidl(&mut self) -> _BUFIDLW {
        _BUFIDLW { w: self }
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline]
    pub fn dmawu(&mut self) -> _DMAWUW {
        _DMAWUW { w: self }
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
}
