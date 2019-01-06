#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPUT {
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
#[doc = "Possible values of the field `RXPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPRSSELR {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected"]
    PRSCH23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXPRSSELR::PRSCH0 => 0,
            RXPRSSELR::PRSCH1 => 1,
            RXPRSSELR::PRSCH2 => 2,
            RXPRSSELR::PRSCH3 => 3,
            RXPRSSELR::PRSCH4 => 4,
            RXPRSSELR::PRSCH5 => 5,
            RXPRSSELR::PRSCH6 => 6,
            RXPRSSELR::PRSCH7 => 7,
            RXPRSSELR::PRSCH8 => 8,
            RXPRSSELR::PRSCH9 => 9,
            RXPRSSELR::PRSCH10 => 10,
            RXPRSSELR::PRSCH11 => 11,
            RXPRSSELR::PRSCH12 => 12,
            RXPRSSELR::PRSCH13 => 13,
            RXPRSSELR::PRSCH14 => 14,
            RXPRSSELR::PRSCH15 => 15,
            RXPRSSELR::PRSCH16 => 16,
            RXPRSSELR::PRSCH17 => 17,
            RXPRSSELR::PRSCH18 => 18,
            RXPRSSELR::PRSCH19 => 19,
            RXPRSSELR::PRSCH20 => 20,
            RXPRSSELR::PRSCH21 => 21,
            RXPRSSELR::PRSCH22 => 22,
            RXPRSSELR::PRSCH23 => 23,
            RXPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXPRSSELR {
        match value {
            0 => RXPRSSELR::PRSCH0,
            1 => RXPRSSELR::PRSCH1,
            2 => RXPRSSELR::PRSCH2,
            3 => RXPRSSELR::PRSCH3,
            4 => RXPRSSELR::PRSCH4,
            5 => RXPRSSELR::PRSCH5,
            6 => RXPRSSELR::PRSCH6,
            7 => RXPRSSELR::PRSCH7,
            8 => RXPRSSELR::PRSCH8,
            9 => RXPRSSELR::PRSCH9,
            10 => RXPRSSELR::PRSCH10,
            11 => RXPRSSELR::PRSCH11,
            12 => RXPRSSELR::PRSCH12,
            13 => RXPRSSELR::PRSCH13,
            14 => RXPRSSELR::PRSCH14,
            15 => RXPRSSELR::PRSCH15,
            16 => RXPRSSELR::PRSCH16,
            17 => RXPRSSELR::PRSCH17,
            18 => RXPRSSELR::PRSCH18,
            19 => RXPRSSELR::PRSCH19,
            20 => RXPRSSELR::PRSCH20,
            21 => RXPRSSELR::PRSCH21,
            22 => RXPRSSELR::PRSCH22,
            23 => RXPRSSELR::PRSCH23,
            i => RXPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == RXPRSSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == RXPRSSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == RXPRSSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == RXPRSSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == RXPRSSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == RXPRSSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == RXPRSSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == RXPRSSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == RXPRSSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == RXPRSSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == RXPRSSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == RXPRSSELR::PRSCH23
    }
}
#[doc = r" Value of the field"]
pub struct RXPRSR {
    bits: bool,
}
impl RXPRSR {
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
#[doc = "Possible values of the field `CLKPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPRSSELR {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected"]
    PRSCH23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKPRSSELR::PRSCH0 => 0,
            CLKPRSSELR::PRSCH1 => 1,
            CLKPRSSELR::PRSCH2 => 2,
            CLKPRSSELR::PRSCH3 => 3,
            CLKPRSSELR::PRSCH4 => 4,
            CLKPRSSELR::PRSCH5 => 5,
            CLKPRSSELR::PRSCH6 => 6,
            CLKPRSSELR::PRSCH7 => 7,
            CLKPRSSELR::PRSCH8 => 8,
            CLKPRSSELR::PRSCH9 => 9,
            CLKPRSSELR::PRSCH10 => 10,
            CLKPRSSELR::PRSCH11 => 11,
            CLKPRSSELR::PRSCH12 => 12,
            CLKPRSSELR::PRSCH13 => 13,
            CLKPRSSELR::PRSCH14 => 14,
            CLKPRSSELR::PRSCH15 => 15,
            CLKPRSSELR::PRSCH16 => 16,
            CLKPRSSELR::PRSCH17 => 17,
            CLKPRSSELR::PRSCH18 => 18,
            CLKPRSSELR::PRSCH19 => 19,
            CLKPRSSELR::PRSCH20 => 20,
            CLKPRSSELR::PRSCH21 => 21,
            CLKPRSSELR::PRSCH22 => 22,
            CLKPRSSELR::PRSCH23 => 23,
            CLKPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKPRSSELR {
        match value {
            0 => CLKPRSSELR::PRSCH0,
            1 => CLKPRSSELR::PRSCH1,
            2 => CLKPRSSELR::PRSCH2,
            3 => CLKPRSSELR::PRSCH3,
            4 => CLKPRSSELR::PRSCH4,
            5 => CLKPRSSELR::PRSCH5,
            6 => CLKPRSSELR::PRSCH6,
            7 => CLKPRSSELR::PRSCH7,
            8 => CLKPRSSELR::PRSCH8,
            9 => CLKPRSSELR::PRSCH9,
            10 => CLKPRSSELR::PRSCH10,
            11 => CLKPRSSELR::PRSCH11,
            12 => CLKPRSSELR::PRSCH12,
            13 => CLKPRSSELR::PRSCH13,
            14 => CLKPRSSELR::PRSCH14,
            15 => CLKPRSSELR::PRSCH15,
            16 => CLKPRSSELR::PRSCH16,
            17 => CLKPRSSELR::PRSCH17,
            18 => CLKPRSSELR::PRSCH18,
            19 => CLKPRSSELR::PRSCH19,
            20 => CLKPRSSELR::PRSCH20,
            21 => CLKPRSSELR::PRSCH21,
            22 => CLKPRSSELR::PRSCH22,
            23 => CLKPRSSELR::PRSCH23,
            i => CLKPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == CLKPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == CLKPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == CLKPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == CLKPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == CLKPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == CLKPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == CLKPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == CLKPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == CLKPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == CLKPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == CLKPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == CLKPRSSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == CLKPRSSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == CLKPRSSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == CLKPRSSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == CLKPRSSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == CLKPRSSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == CLKPRSSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == CLKPRSSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == CLKPRSSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == CLKPRSSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == CLKPRSSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == CLKPRSSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == CLKPRSSELR::PRSCH23
    }
}
#[doc = r" Value of the field"]
pub struct CLKPRSR {
    bits: bool,
}
impl CLKPRSR {
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
#[doc = "Values that can be written to the field `RXPRSSEL`"]
pub enum RXPRSSELW {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected"]
    PRSCH23,
}
impl RXPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXPRSSELW::PRSCH0 => 0,
            RXPRSSELW::PRSCH1 => 1,
            RXPRSSELW::PRSCH2 => 2,
            RXPRSSELW::PRSCH3 => 3,
            RXPRSSELW::PRSCH4 => 4,
            RXPRSSELW::PRSCH5 => 5,
            RXPRSSELW::PRSCH6 => 6,
            RXPRSSELW::PRSCH7 => 7,
            RXPRSSELW::PRSCH8 => 8,
            RXPRSSELW::PRSCH9 => 9,
            RXPRSSELW::PRSCH10 => 10,
            RXPRSSELW::PRSCH11 => 11,
            RXPRSSELW::PRSCH12 => 12,
            RXPRSSELW::PRSCH13 => 13,
            RXPRSSELW::PRSCH14 => 14,
            RXPRSSELW::PRSCH15 => 15,
            RXPRSSELW::PRSCH16 => 16,
            RXPRSSELW::PRSCH17 => 17,
            RXPRSSELW::PRSCH18 => 18,
            RXPRSSELW::PRSCH19 => 19,
            RXPRSSELW::PRSCH20 => 20,
            RXPRSSELW::PRSCH21 => 21,
            RXPRSSELW::PRSCH22 => 22,
            RXPRSSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPRSW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKPRSSEL`"]
pub enum CLKPRSSELW {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected"]
    PRSCH23,
}
impl CLKPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKPRSSELW::PRSCH0 => 0,
            CLKPRSSELW::PRSCH1 => 1,
            CLKPRSSELW::PRSCH2 => 2,
            CLKPRSSELW::PRSCH3 => 3,
            CLKPRSSELW::PRSCH4 => 4,
            CLKPRSSELW::PRSCH5 => 5,
            CLKPRSSELW::PRSCH6 => 6,
            CLKPRSSELW::PRSCH7 => 7,
            CLKPRSSELW::PRSCH8 => 8,
            CLKPRSSELW::PRSCH9 => 9,
            CLKPRSSELW::PRSCH10 => 10,
            CLKPRSSELW::PRSCH11 => 11,
            CLKPRSSELW::PRSCH12 => 12,
            CLKPRSSELW::PRSCH13 => 13,
            CLKPRSSELW::PRSCH14 => 14,
            CLKPRSSELW::PRSCH15 => 15,
            CLKPRSSELW::PRSCH16 => 16,
            CLKPRSSELW::PRSCH17 => 17,
            CLKPRSSELW::PRSCH18 => 18,
            CLKPRSSELW::PRSCH19 => 19,
            CLKPRSSELW::PRSCH20 => 20,
            CLKPRSSELW::PRSCH21 => 21,
            CLKPRSSELW::PRSCH22 => 22,
            CLKPRSSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(CLKPRSSELW::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPRSW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline]
    pub fn rxprssel(&self) -> RXPRSSELR {
        RXPRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline]
    pub fn rxprs(&self) -> RXPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXPRSR { bits }
    }
    #[doc = "Bits 8:12 - CLK PRS Channel Select"]
    #[inline]
    pub fn clkprssel(&self) -> CLKPRSSELR {
        CLKPRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline]
    pub fn clkprs(&self) -> CLKPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKPRSR { bits }
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
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline]
    pub fn rxprssel(&mut self) -> _RXPRSSELW {
        _RXPRSSELW { w: self }
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline]
    pub fn rxprs(&mut self) -> _RXPRSW {
        _RXPRSW { w: self }
    }
    #[doc = "Bits 8:12 - CLK PRS Channel Select"]
    #[inline]
    pub fn clkprssel(&mut self) -> _CLKPRSSELW {
        _CLKPRSSELW { w: self }
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline]
    pub fn clkprs(&mut self) -> _CLKPRSW {
        _CLKPRSW { w: self }
    }
}
