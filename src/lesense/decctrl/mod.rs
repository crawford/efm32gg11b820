#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DECCTRL {
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
pub struct DISABLER {
    bits: bool,
}
impl DISABLER {
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
pub struct ERRCHKR {
    bits: bool,
}
impl ERRCHKR {
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
pub struct INTMAPR {
    bits: bool,
}
impl INTMAPR {
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
pub struct HYSTPRS0R {
    bits: bool,
}
impl HYSTPRS0R {
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
pub struct HYSTPRS1R {
    bits: bool,
}
impl HYSTPRS1R {
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
pub struct HYSTPRS2R {
    bits: bool,
}
impl HYSTPRS2R {
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
pub struct HYSTIRQR {
    bits: bool,
}
impl HYSTIRQR {
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
pub struct PRSCNTR {
    bits: bool,
}
impl PRSCNTR {
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
pub struct INPUTR {
    bits: bool,
}
impl INPUTR {
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
#[doc = "Possible values of the field `PRSSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL0R {
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
impl PRSSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSEL0R::PRSCH0 => 0,
            PRSSEL0R::PRSCH1 => 1,
            PRSSEL0R::PRSCH2 => 2,
            PRSSEL0R::PRSCH3 => 3,
            PRSSEL0R::PRSCH4 => 4,
            PRSSEL0R::PRSCH5 => 5,
            PRSSEL0R::PRSCH6 => 6,
            PRSSEL0R::PRSCH7 => 7,
            PRSSEL0R::PRSCH8 => 8,
            PRSSEL0R::PRSCH9 => 9,
            PRSSEL0R::PRSCH10 => 10,
            PRSSEL0R::PRSCH11 => 11,
            PRSSEL0R::PRSCH12 => 12,
            PRSSEL0R::PRSCH13 => 13,
            PRSSEL0R::PRSCH14 => 14,
            PRSSEL0R::PRSCH15 => 15,
            PRSSEL0R::PRSCH16 => 16,
            PRSSEL0R::PRSCH17 => 17,
            PRSSEL0R::PRSCH18 => 18,
            PRSSEL0R::PRSCH19 => 19,
            PRSSEL0R::PRSCH20 => 20,
            PRSSEL0R::PRSCH21 => 21,
            PRSSEL0R::PRSCH22 => 22,
            PRSSEL0R::PRSCH23 => 23,
            PRSSEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSEL0R {
        match value {
            0 => PRSSEL0R::PRSCH0,
            1 => PRSSEL0R::PRSCH1,
            2 => PRSSEL0R::PRSCH2,
            3 => PRSSEL0R::PRSCH3,
            4 => PRSSEL0R::PRSCH4,
            5 => PRSSEL0R::PRSCH5,
            6 => PRSSEL0R::PRSCH6,
            7 => PRSSEL0R::PRSCH7,
            8 => PRSSEL0R::PRSCH8,
            9 => PRSSEL0R::PRSCH9,
            10 => PRSSEL0R::PRSCH10,
            11 => PRSSEL0R::PRSCH11,
            12 => PRSSEL0R::PRSCH12,
            13 => PRSSEL0R::PRSCH13,
            14 => PRSSEL0R::PRSCH14,
            15 => PRSSEL0R::PRSCH15,
            16 => PRSSEL0R::PRSCH16,
            17 => PRSSEL0R::PRSCH17,
            18 => PRSSEL0R::PRSCH18,
            19 => PRSSEL0R::PRSCH19,
            20 => PRSSEL0R::PRSCH20,
            21 => PRSSEL0R::PRSCH21,
            22 => PRSSEL0R::PRSCH22,
            23 => PRSSEL0R::PRSCH23,
            i => PRSSEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL0R::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL0R::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL0R::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL0R::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL0R::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL0R::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL0R::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL0R::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL0R::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL0R::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL0R::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL0R::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL0R::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL0R::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL0R::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL0R::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL0R::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL0R::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL0R::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL0R::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL0R::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL0R::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL0R::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL0R::PRSCH23
    }
}
#[doc = "Possible values of the field `PRSSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL1R {
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
impl PRSSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSEL1R::PRSCH0 => 0,
            PRSSEL1R::PRSCH1 => 1,
            PRSSEL1R::PRSCH2 => 2,
            PRSSEL1R::PRSCH3 => 3,
            PRSSEL1R::PRSCH4 => 4,
            PRSSEL1R::PRSCH5 => 5,
            PRSSEL1R::PRSCH6 => 6,
            PRSSEL1R::PRSCH7 => 7,
            PRSSEL1R::PRSCH8 => 8,
            PRSSEL1R::PRSCH9 => 9,
            PRSSEL1R::PRSCH10 => 10,
            PRSSEL1R::PRSCH11 => 11,
            PRSSEL1R::PRSCH12 => 12,
            PRSSEL1R::PRSCH13 => 13,
            PRSSEL1R::PRSCH14 => 14,
            PRSSEL1R::PRSCH15 => 15,
            PRSSEL1R::PRSCH16 => 16,
            PRSSEL1R::PRSCH17 => 17,
            PRSSEL1R::PRSCH18 => 18,
            PRSSEL1R::PRSCH19 => 19,
            PRSSEL1R::PRSCH20 => 20,
            PRSSEL1R::PRSCH21 => 21,
            PRSSEL1R::PRSCH22 => 22,
            PRSSEL1R::PRSCH23 => 23,
            PRSSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSEL1R {
        match value {
            0 => PRSSEL1R::PRSCH0,
            1 => PRSSEL1R::PRSCH1,
            2 => PRSSEL1R::PRSCH2,
            3 => PRSSEL1R::PRSCH3,
            4 => PRSSEL1R::PRSCH4,
            5 => PRSSEL1R::PRSCH5,
            6 => PRSSEL1R::PRSCH6,
            7 => PRSSEL1R::PRSCH7,
            8 => PRSSEL1R::PRSCH8,
            9 => PRSSEL1R::PRSCH9,
            10 => PRSSEL1R::PRSCH10,
            11 => PRSSEL1R::PRSCH11,
            12 => PRSSEL1R::PRSCH12,
            13 => PRSSEL1R::PRSCH13,
            14 => PRSSEL1R::PRSCH14,
            15 => PRSSEL1R::PRSCH15,
            16 => PRSSEL1R::PRSCH16,
            17 => PRSSEL1R::PRSCH17,
            18 => PRSSEL1R::PRSCH18,
            19 => PRSSEL1R::PRSCH19,
            20 => PRSSEL1R::PRSCH20,
            21 => PRSSEL1R::PRSCH21,
            22 => PRSSEL1R::PRSCH22,
            23 => PRSSEL1R::PRSCH23,
            i => PRSSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL1R::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL1R::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL1R::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL1R::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL1R::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL1R::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL1R::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL1R::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL1R::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL1R::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL1R::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL1R::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL1R::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL1R::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL1R::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL1R::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL1R::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL1R::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL1R::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL1R::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL1R::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL1R::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL1R::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL1R::PRSCH23
    }
}
#[doc = "Possible values of the field `PRSSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL2R {
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
impl PRSSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSEL2R::PRSCH0 => 0,
            PRSSEL2R::PRSCH1 => 1,
            PRSSEL2R::PRSCH2 => 2,
            PRSSEL2R::PRSCH3 => 3,
            PRSSEL2R::PRSCH4 => 4,
            PRSSEL2R::PRSCH5 => 5,
            PRSSEL2R::PRSCH6 => 6,
            PRSSEL2R::PRSCH7 => 7,
            PRSSEL2R::PRSCH8 => 8,
            PRSSEL2R::PRSCH9 => 9,
            PRSSEL2R::PRSCH10 => 10,
            PRSSEL2R::PRSCH11 => 11,
            PRSSEL2R::PRSCH12 => 12,
            PRSSEL2R::PRSCH13 => 13,
            PRSSEL2R::PRSCH14 => 14,
            PRSSEL2R::PRSCH15 => 15,
            PRSSEL2R::PRSCH16 => 16,
            PRSSEL2R::PRSCH17 => 17,
            PRSSEL2R::PRSCH18 => 18,
            PRSSEL2R::PRSCH19 => 19,
            PRSSEL2R::PRSCH20 => 20,
            PRSSEL2R::PRSCH21 => 21,
            PRSSEL2R::PRSCH22 => 22,
            PRSSEL2R::PRSCH23 => 23,
            PRSSEL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSEL2R {
        match value {
            0 => PRSSEL2R::PRSCH0,
            1 => PRSSEL2R::PRSCH1,
            2 => PRSSEL2R::PRSCH2,
            3 => PRSSEL2R::PRSCH3,
            4 => PRSSEL2R::PRSCH4,
            5 => PRSSEL2R::PRSCH5,
            6 => PRSSEL2R::PRSCH6,
            7 => PRSSEL2R::PRSCH7,
            8 => PRSSEL2R::PRSCH8,
            9 => PRSSEL2R::PRSCH9,
            10 => PRSSEL2R::PRSCH10,
            11 => PRSSEL2R::PRSCH11,
            12 => PRSSEL2R::PRSCH12,
            13 => PRSSEL2R::PRSCH13,
            14 => PRSSEL2R::PRSCH14,
            15 => PRSSEL2R::PRSCH15,
            16 => PRSSEL2R::PRSCH16,
            17 => PRSSEL2R::PRSCH17,
            18 => PRSSEL2R::PRSCH18,
            19 => PRSSEL2R::PRSCH19,
            20 => PRSSEL2R::PRSCH20,
            21 => PRSSEL2R::PRSCH21,
            22 => PRSSEL2R::PRSCH22,
            23 => PRSSEL2R::PRSCH23,
            i => PRSSEL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL2R::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL2R::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL2R::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL2R::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL2R::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL2R::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL2R::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL2R::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL2R::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL2R::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL2R::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL2R::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL2R::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL2R::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL2R::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL2R::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL2R::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL2R::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL2R::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL2R::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL2R::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL2R::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL2R::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL2R::PRSCH23
    }
}
#[doc = "Possible values of the field `PRSSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL3R {
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
impl PRSSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSEL3R::PRSCH0 => 0,
            PRSSEL3R::PRSCH1 => 1,
            PRSSEL3R::PRSCH2 => 2,
            PRSSEL3R::PRSCH3 => 3,
            PRSSEL3R::PRSCH4 => 4,
            PRSSEL3R::PRSCH5 => 5,
            PRSSEL3R::PRSCH6 => 6,
            PRSSEL3R::PRSCH7 => 7,
            PRSSEL3R::PRSCH8 => 8,
            PRSSEL3R::PRSCH9 => 9,
            PRSSEL3R::PRSCH10 => 10,
            PRSSEL3R::PRSCH11 => 11,
            PRSSEL3R::PRSCH12 => 12,
            PRSSEL3R::PRSCH13 => 13,
            PRSSEL3R::PRSCH14 => 14,
            PRSSEL3R::PRSCH15 => 15,
            PRSSEL3R::PRSCH16 => 16,
            PRSSEL3R::PRSCH17 => 17,
            PRSSEL3R::PRSCH18 => 18,
            PRSSEL3R::PRSCH19 => 19,
            PRSSEL3R::PRSCH20 => 20,
            PRSSEL3R::PRSCH21 => 21,
            PRSSEL3R::PRSCH22 => 22,
            PRSSEL3R::PRSCH23 => 23,
            PRSSEL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSEL3R {
        match value {
            0 => PRSSEL3R::PRSCH0,
            1 => PRSSEL3R::PRSCH1,
            2 => PRSSEL3R::PRSCH2,
            3 => PRSSEL3R::PRSCH3,
            4 => PRSSEL3R::PRSCH4,
            5 => PRSSEL3R::PRSCH5,
            6 => PRSSEL3R::PRSCH6,
            7 => PRSSEL3R::PRSCH7,
            8 => PRSSEL3R::PRSCH8,
            9 => PRSSEL3R::PRSCH9,
            10 => PRSSEL3R::PRSCH10,
            11 => PRSSEL3R::PRSCH11,
            12 => PRSSEL3R::PRSCH12,
            13 => PRSSEL3R::PRSCH13,
            14 => PRSSEL3R::PRSCH14,
            15 => PRSSEL3R::PRSCH15,
            16 => PRSSEL3R::PRSCH16,
            17 => PRSSEL3R::PRSCH17,
            18 => PRSSEL3R::PRSCH18,
            19 => PRSSEL3R::PRSCH19,
            20 => PRSSEL3R::PRSCH20,
            21 => PRSSEL3R::PRSCH21,
            22 => PRSSEL3R::PRSCH22,
            23 => PRSSEL3R::PRSCH23,
            i => PRSSEL3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL3R::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL3R::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL3R::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL3R::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL3R::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL3R::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL3R::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL3R::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL3R::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL3R::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL3R::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL3R::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL3R::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL3R::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL3R::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL3R::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL3R::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL3R::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL3R::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL3R::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL3R::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL3R::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL3R::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL3R::PRSCH23
    }
}
#[doc = r" Proxy"]
pub struct _DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLEW<'a> {
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
pub struct _ERRCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRCHKW<'a> {
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
pub struct _INTMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _INTMAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _HYSTPRS0W<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTPRS0W<'a> {
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
#[doc = r" Proxy"]
pub struct _HYSTPRS1W<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTPRS1W<'a> {
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
pub struct _HYSTPRS2W<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTPRS2W<'a> {
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
pub struct _HYSTIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTIRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRSCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCNTW<'a> {
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
#[doc = r" Proxy"]
pub struct _INPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL0`"]
pub enum PRSSEL0W {
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
impl PRSSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSEL0W::PRSCH0 => 0,
            PRSSEL0W::PRSCH1 => 1,
            PRSSEL0W::PRSCH2 => 2,
            PRSSEL0W::PRSCH3 => 3,
            PRSSEL0W::PRSCH4 => 4,
            PRSSEL0W::PRSCH5 => 5,
            PRSSEL0W::PRSCH6 => 6,
            PRSSEL0W::PRSCH7 => 7,
            PRSSEL0W::PRSCH8 => 8,
            PRSSEL0W::PRSCH9 => 9,
            PRSSEL0W::PRSCH10 => 10,
            PRSSEL0W::PRSCH11 => 11,
            PRSSEL0W::PRSCH12 => 12,
            PRSSEL0W::PRSCH13 => 13,
            PRSSEL0W::PRSCH14 => 14,
            PRSSEL0W::PRSCH15 => 15,
            PRSSEL0W::PRSCH16 => 16,
            PRSSEL0W::PRSCH17 => 17,
            PRSSEL0W::PRSCH18 => 18,
            PRSSEL0W::PRSCH19 => 19,
            PRSSEL0W::PRSCH20 => 20,
            PRSSEL0W::PRSCH21 => 21,
            PRSSEL0W::PRSCH22 => 22,
            PRSSEL0W::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL0W::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL1`"]
pub enum PRSSEL1W {
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
impl PRSSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSEL1W::PRSCH0 => 0,
            PRSSEL1W::PRSCH1 => 1,
            PRSSEL1W::PRSCH2 => 2,
            PRSSEL1W::PRSCH3 => 3,
            PRSSEL1W::PRSCH4 => 4,
            PRSSEL1W::PRSCH5 => 5,
            PRSSEL1W::PRSCH6 => 6,
            PRSSEL1W::PRSCH7 => 7,
            PRSSEL1W::PRSCH8 => 8,
            PRSSEL1W::PRSCH9 => 9,
            PRSSEL1W::PRSCH10 => 10,
            PRSSEL1W::PRSCH11 => 11,
            PRSSEL1W::PRSCH12 => 12,
            PRSSEL1W::PRSCH13 => 13,
            PRSSEL1W::PRSCH14 => 14,
            PRSSEL1W::PRSCH15 => 15,
            PRSSEL1W::PRSCH16 => 16,
            PRSSEL1W::PRSCH17 => 17,
            PRSSEL1W::PRSCH18 => 18,
            PRSSEL1W::PRSCH19 => 19,
            PRSSEL1W::PRSCH20 => 20,
            PRSSEL1W::PRSCH21 => 21,
            PRSSEL1W::PRSCH22 => 22,
            PRSSEL1W::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL1W::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL2`"]
pub enum PRSSEL2W {
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
impl PRSSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSEL2W::PRSCH0 => 0,
            PRSSEL2W::PRSCH1 => 1,
            PRSSEL2W::PRSCH2 => 2,
            PRSSEL2W::PRSCH3 => 3,
            PRSSEL2W::PRSCH4 => 4,
            PRSSEL2W::PRSCH5 => 5,
            PRSSEL2W::PRSCH6 => 6,
            PRSSEL2W::PRSCH7 => 7,
            PRSSEL2W::PRSCH8 => 8,
            PRSSEL2W::PRSCH9 => 9,
            PRSSEL2W::PRSCH10 => 10,
            PRSSEL2W::PRSCH11 => 11,
            PRSSEL2W::PRSCH12 => 12,
            PRSSEL2W::PRSCH13 => 13,
            PRSSEL2W::PRSCH14 => 14,
            PRSSEL2W::PRSCH15 => 15,
            PRSSEL2W::PRSCH16 => 16,
            PRSSEL2W::PRSCH17 => 17,
            PRSSEL2W::PRSCH18 => 18,
            PRSSEL2W::PRSCH19 => 19,
            PRSSEL2W::PRSCH20 => 20,
            PRSSEL2W::PRSCH21 => 21,
            PRSSEL2W::PRSCH22 => 22,
            PRSSEL2W::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSEL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL2W::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL3`"]
pub enum PRSSEL3W {
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
impl PRSSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSEL3W::PRSCH0 => 0,
            PRSSEL3W::PRSCH1 => 1,
            PRSSEL3W::PRSCH2 => 2,
            PRSSEL3W::PRSCH3 => 3,
            PRSSEL3W::PRSCH4 => 4,
            PRSSEL3W::PRSCH5 => 5,
            PRSSEL3W::PRSCH6 => 6,
            PRSSEL3W::PRSCH7 => 7,
            PRSSEL3W::PRSCH8 => 8,
            PRSSEL3W::PRSCH9 => 9,
            PRSSEL3W::PRSCH10 => 10,
            PRSSEL3W::PRSCH11 => 11,
            PRSSEL3W::PRSCH12 => 12,
            PRSSEL3W::PRSCH13 => 13,
            PRSSEL3W::PRSCH14 => 14,
            PRSSEL3W::PRSCH15 => 15,
            PRSSEL3W::PRSCH16 => 16,
            PRSSEL3W::PRSCH17 => 17,
            PRSSEL3W::PRSCH18 => 18,
            PRSSEL3W::PRSCH19 => 19,
            PRSSEL3W::PRSCH20 => 20,
            PRSSEL3W::PRSCH21 => 21,
            PRSSEL3W::PRSCH22 => 22,
            PRSSEL3W::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSEL3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL3W::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline]
    pub fn disable(&self) -> DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLER { bits }
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline]
    pub fn errchk(&self) -> ERRCHKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRCHKR { bits }
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline]
    pub fn intmap(&self) -> INTMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMAPR { bits }
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline]
    pub fn hystprs0(&self) -> HYSTPRS0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTPRS0R { bits }
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline]
    pub fn hystprs1(&self) -> HYSTPRS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTPRS1R { bits }
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline]
    pub fn hystprs2(&self) -> HYSTPRS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTPRS2R { bits }
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline]
    pub fn hystirq(&self) -> HYSTIRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTIRQR { bits }
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline]
    pub fn prscnt(&self) -> PRSCNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSCNTR { bits }
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline]
    pub fn input(&self) -> INPUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INPUTR { bits }
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline]
    pub fn prssel0(&self) -> PRSSEL0R {
        PRSSEL0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline]
    pub fn prssel1(&self) -> PRSSEL1R {
        PRSSEL1R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline]
    pub fn prssel2(&self) -> PRSSEL2R {
        PRSSEL2R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline]
    pub fn prssel3(&self) -> PRSSEL3R {
        PRSSEL3R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline]
    pub fn disable(&mut self) -> _DISABLEW {
        _DISABLEW { w: self }
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline]
    pub fn errchk(&mut self) -> _ERRCHKW {
        _ERRCHKW { w: self }
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline]
    pub fn intmap(&mut self) -> _INTMAPW {
        _INTMAPW { w: self }
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline]
    pub fn hystprs0(&mut self) -> _HYSTPRS0W {
        _HYSTPRS0W { w: self }
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline]
    pub fn hystprs1(&mut self) -> _HYSTPRS1W {
        _HYSTPRS1W { w: self }
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline]
    pub fn hystprs2(&mut self) -> _HYSTPRS2W {
        _HYSTPRS2W { w: self }
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline]
    pub fn hystirq(&mut self) -> _HYSTIRQW {
        _HYSTIRQW { w: self }
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline]
    pub fn prscnt(&mut self) -> _PRSCNTW {
        _PRSCNTW { w: self }
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline]
    pub fn input(&mut self) -> _INPUTW {
        _INPUTW { w: self }
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline]
    pub fn prssel0(&mut self) -> _PRSSEL0W {
        _PRSSEL0W { w: self }
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline]
    pub fn prssel1(&mut self) -> _PRSSEL1W {
        _PRSSEL1W { w: self }
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline]
    pub fn prssel2(&mut self) -> _PRSSEL2W {
        _PRSSEL2W { w: self }
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline]
    pub fn prssel3(&mut self) -> _PRSSEL3W {
        _PRSSEL3W { w: self }
    }
}
