#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTFC {
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
#[doc = "Possible values of the field `DTPRS0FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS0FSELR {
    #[doc = "PRS Channel 0 selected as fault source 0"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 2"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 3"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 4"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 5"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 6"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 7"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as fault source 8"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as fault source 9"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as fault source 10"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as fault source 11"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as fault source 12"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as fault source 13"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as fault source 14"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as fault source 15"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as fault source 16"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as fault source 17"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as fault source 18"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as fault source 19"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as fault source 20"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as fault source 21"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as fault source 22"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as fault source 23"]
    PRSCH23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTPRS0FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRS0FSELR::PRSCH0 => 0,
            DTPRS0FSELR::PRSCH1 => 1,
            DTPRS0FSELR::PRSCH2 => 2,
            DTPRS0FSELR::PRSCH3 => 3,
            DTPRS0FSELR::PRSCH4 => 4,
            DTPRS0FSELR::PRSCH5 => 5,
            DTPRS0FSELR::PRSCH6 => 6,
            DTPRS0FSELR::PRSCH7 => 7,
            DTPRS0FSELR::PRSCH8 => 8,
            DTPRS0FSELR::PRSCH9 => 9,
            DTPRS0FSELR::PRSCH10 => 10,
            DTPRS0FSELR::PRSCH11 => 11,
            DTPRS0FSELR::PRSCH12 => 12,
            DTPRS0FSELR::PRSCH13 => 13,
            DTPRS0FSELR::PRSCH14 => 14,
            DTPRS0FSELR::PRSCH15 => 15,
            DTPRS0FSELR::PRSCH16 => 16,
            DTPRS0FSELR::PRSCH17 => 17,
            DTPRS0FSELR::PRSCH18 => 18,
            DTPRS0FSELR::PRSCH19 => 19,
            DTPRS0FSELR::PRSCH20 => 20,
            DTPRS0FSELR::PRSCH21 => 21,
            DTPRS0FSELR::PRSCH22 => 22,
            DTPRS0FSELR::PRSCH23 => 23,
            DTPRS0FSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRS0FSELR {
        match value {
            0 => DTPRS0FSELR::PRSCH0,
            1 => DTPRS0FSELR::PRSCH1,
            2 => DTPRS0FSELR::PRSCH2,
            3 => DTPRS0FSELR::PRSCH3,
            4 => DTPRS0FSELR::PRSCH4,
            5 => DTPRS0FSELR::PRSCH5,
            6 => DTPRS0FSELR::PRSCH6,
            7 => DTPRS0FSELR::PRSCH7,
            8 => DTPRS0FSELR::PRSCH8,
            9 => DTPRS0FSELR::PRSCH9,
            10 => DTPRS0FSELR::PRSCH10,
            11 => DTPRS0FSELR::PRSCH11,
            12 => DTPRS0FSELR::PRSCH12,
            13 => DTPRS0FSELR::PRSCH13,
            14 => DTPRS0FSELR::PRSCH14,
            15 => DTPRS0FSELR::PRSCH15,
            16 => DTPRS0FSELR::PRSCH16,
            17 => DTPRS0FSELR::PRSCH17,
            18 => DTPRS0FSELR::PRSCH18,
            19 => DTPRS0FSELR::PRSCH19,
            20 => DTPRS0FSELR::PRSCH20,
            21 => DTPRS0FSELR::PRSCH21,
            22 => DTPRS0FSELR::PRSCH22,
            23 => DTPRS0FSELR::PRSCH23,
            i => DTPRS0FSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH23
    }
}
#[doc = "Possible values of the field `DTPRS1FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS1FSELR {
    #[doc = "PRS Channel 0 selected as fault source 1"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 1"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 1"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 1"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 1"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 1"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 1"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as fault source 1"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as fault source 1"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as fault source 1"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as fault source 1"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as fault source 1"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as fault source 1"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as fault source 1"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as fault source 1"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as fault source 1"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as fault source 1"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as fault source 1"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as fault source 1"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as fault source 1"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as fault source 1"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as fault source 1"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as fault source 1"]
    PRSCH23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTPRS1FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRS1FSELR::PRSCH0 => 0,
            DTPRS1FSELR::PRSCH1 => 1,
            DTPRS1FSELR::PRSCH2 => 2,
            DTPRS1FSELR::PRSCH3 => 3,
            DTPRS1FSELR::PRSCH4 => 4,
            DTPRS1FSELR::PRSCH5 => 5,
            DTPRS1FSELR::PRSCH6 => 6,
            DTPRS1FSELR::PRSCH7 => 7,
            DTPRS1FSELR::PRSCH8 => 8,
            DTPRS1FSELR::PRSCH9 => 9,
            DTPRS1FSELR::PRSCH10 => 10,
            DTPRS1FSELR::PRSCH11 => 11,
            DTPRS1FSELR::PRSCH12 => 12,
            DTPRS1FSELR::PRSCH13 => 13,
            DTPRS1FSELR::PRSCH14 => 14,
            DTPRS1FSELR::PRSCH15 => 15,
            DTPRS1FSELR::PRSCH16 => 16,
            DTPRS1FSELR::PRSCH17 => 17,
            DTPRS1FSELR::PRSCH18 => 18,
            DTPRS1FSELR::PRSCH19 => 19,
            DTPRS1FSELR::PRSCH20 => 20,
            DTPRS1FSELR::PRSCH21 => 21,
            DTPRS1FSELR::PRSCH22 => 22,
            DTPRS1FSELR::PRSCH23 => 23,
            DTPRS1FSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRS1FSELR {
        match value {
            0 => DTPRS1FSELR::PRSCH0,
            1 => DTPRS1FSELR::PRSCH1,
            2 => DTPRS1FSELR::PRSCH2,
            3 => DTPRS1FSELR::PRSCH3,
            4 => DTPRS1FSELR::PRSCH4,
            5 => DTPRS1FSELR::PRSCH5,
            6 => DTPRS1FSELR::PRSCH6,
            7 => DTPRS1FSELR::PRSCH7,
            8 => DTPRS1FSELR::PRSCH8,
            9 => DTPRS1FSELR::PRSCH9,
            10 => DTPRS1FSELR::PRSCH10,
            11 => DTPRS1FSELR::PRSCH11,
            12 => DTPRS1FSELR::PRSCH12,
            13 => DTPRS1FSELR::PRSCH13,
            14 => DTPRS1FSELR::PRSCH14,
            15 => DTPRS1FSELR::PRSCH15,
            16 => DTPRS1FSELR::PRSCH16,
            17 => DTPRS1FSELR::PRSCH17,
            18 => DTPRS1FSELR::PRSCH18,
            19 => DTPRS1FSELR::PRSCH19,
            20 => DTPRS1FSELR::PRSCH20,
            21 => DTPRS1FSELR::PRSCH21,
            22 => DTPRS1FSELR::PRSCH22,
            23 => DTPRS1FSELR::PRSCH23,
            i => DTPRS1FSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH23
    }
}
#[doc = "Possible values of the field `DTFA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFAR {
    #[doc = "No action on fault"]
    NONE,
    #[doc = "Set outputs inactive"]
    INACTIVE,
    #[doc = "Clear outputs"]
    CLEAR,
    #[doc = "Tristate outputs"]
    TRISTATE,
}
impl DTFAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTFAR::NONE => 0,
            DTFAR::INACTIVE => 1,
            DTFAR::CLEAR => 2,
            DTFAR::TRISTATE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTFAR {
        match value {
            0 => DTFAR::NONE,
            1 => DTFAR::INACTIVE,
            2 => DTFAR::CLEAR,
            3 => DTFAR::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DTFAR::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == DTFAR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == DTFAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline]
    pub fn is_tristate(&self) -> bool {
        *self == DTFAR::TRISTATE
    }
}
#[doc = r" Value of the field"]
pub struct DTPRS0FENR {
    bits: bool,
}
impl DTPRS0FENR {
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
pub struct DTPRS1FENR {
    bits: bool,
}
impl DTPRS1FENR {
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
pub struct DTDBGFENR {
    bits: bool,
}
impl DTDBGFENR {
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
pub struct DTLOCKUPFENR {
    bits: bool,
}
impl DTLOCKUPFENR {
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
#[doc = "Values that can be written to the field `DTPRS0FSEL`"]
pub enum DTPRS0FSELW {
    #[doc = "PRS Channel 0 selected as fault source 0"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 2"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 3"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 4"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 5"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 6"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 7"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as fault source 8"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as fault source 9"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as fault source 10"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as fault source 11"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as fault source 12"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as fault source 13"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as fault source 14"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as fault source 15"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as fault source 16"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as fault source 17"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as fault source 18"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as fault source 19"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as fault source 20"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as fault source 21"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as fault source 22"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as fault source 23"]
    PRSCH23,
}
impl DTPRS0FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRS0FSELW::PRSCH0 => 0,
            DTPRS0FSELW::PRSCH1 => 1,
            DTPRS0FSELW::PRSCH2 => 2,
            DTPRS0FSELW::PRSCH3 => 3,
            DTPRS0FSELW::PRSCH4 => 4,
            DTPRS0FSELW::PRSCH5 => 5,
            DTPRS0FSELW::PRSCH6 => 6,
            DTPRS0FSELW::PRSCH7 => 7,
            DTPRS0FSELW::PRSCH8 => 8,
            DTPRS0FSELW::PRSCH9 => 9,
            DTPRS0FSELW::PRSCH10 => 10,
            DTPRS0FSELW::PRSCH11 => 11,
            DTPRS0FSELW::PRSCH12 => 12,
            DTPRS0FSELW::PRSCH13 => 13,
            DTPRS0FSELW::PRSCH14 => 14,
            DTPRS0FSELW::PRSCH15 => 15,
            DTPRS0FSELW::PRSCH16 => 16,
            DTPRS0FSELW::PRSCH17 => 17,
            DTPRS0FSELW::PRSCH18 => 18,
            DTPRS0FSELW::PRSCH19 => 19,
            DTPRS0FSELW::PRSCH20 => 20,
            DTPRS0FSELW::PRSCH21 => 21,
            DTPRS0FSELW::PRSCH22 => 22,
            DTPRS0FSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS0FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS0FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRS0FSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 2"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 3"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 4"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 5"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 6"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 7"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 8"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 9"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 10"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 11"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 12"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 13"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 14"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 15"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 16"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 17"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 18"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 19"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 20"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 21"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 22"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 23"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH23)
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
#[doc = "Values that can be written to the field `DTPRS1FSEL`"]
pub enum DTPRS1FSELW {
    #[doc = "PRS Channel 0 selected as fault source 1"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 1"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 1"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 1"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 1"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 1"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 1"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as fault source 1"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as fault source 1"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as fault source 1"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as fault source 1"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as fault source 1"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as fault source 1"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as fault source 1"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as fault source 1"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as fault source 1"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as fault source 1"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as fault source 1"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as fault source 1"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as fault source 1"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as fault source 1"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as fault source 1"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as fault source 1"]
    PRSCH23,
}
impl DTPRS1FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRS1FSELW::PRSCH0 => 0,
            DTPRS1FSELW::PRSCH1 => 1,
            DTPRS1FSELW::PRSCH2 => 2,
            DTPRS1FSELW::PRSCH3 => 3,
            DTPRS1FSELW::PRSCH4 => 4,
            DTPRS1FSELW::PRSCH5 => 5,
            DTPRS1FSELW::PRSCH6 => 6,
            DTPRS1FSELW::PRSCH7 => 7,
            DTPRS1FSELW::PRSCH8 => 8,
            DTPRS1FSELW::PRSCH9 => 9,
            DTPRS1FSELW::PRSCH10 => 10,
            DTPRS1FSELW::PRSCH11 => 11,
            DTPRS1FSELW::PRSCH12 => 12,
            DTPRS1FSELW::PRSCH13 => 13,
            DTPRS1FSELW::PRSCH14 => 14,
            DTPRS1FSELW::PRSCH15 => 15,
            DTPRS1FSELW::PRSCH16 => 16,
            DTPRS1FSELW::PRSCH17 => 17,
            DTPRS1FSELW::PRSCH18 => 18,
            DTPRS1FSELW::PRSCH19 => 19,
            DTPRS1FSELW::PRSCH20 => 20,
            DTPRS1FSELW::PRSCH21 => 21,
            DTPRS1FSELW::PRSCH22 => 22,
            DTPRS1FSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS1FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS1FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRS1FSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 1"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 1"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 1"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 1"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 1"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 1"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 1"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 1"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 1"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 1"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 1"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 1"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 1"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 1"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 1"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 1"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH23)
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
#[doc = "Values that can be written to the field `DTFA`"]
pub enum DTFAW {
    #[doc = "No action on fault"]
    NONE,
    #[doc = "Set outputs inactive"]
    INACTIVE,
    #[doc = "Clear outputs"]
    CLEAR,
    #[doc = "Tristate outputs"]
    TRISTATE,
}
impl DTFAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTFAW::NONE => 0,
            DTFAW::INACTIVE => 1,
            DTFAW::CLEAR => 2,
            DTFAW::TRISTATE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTFAW<'a> {
    w: &'a mut W,
}
impl<'a> _DTFAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTFAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on fault"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFAW::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFAW::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFAW::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFAW::TRISTATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS0FENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS0FENW<'a> {
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
pub struct _DTPRS1FENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS1FENW<'a> {
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
pub struct _DTDBGFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDBGFENW<'a> {
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
pub struct _DTLOCKUPFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTLOCKUPFENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline]
    pub fn dtprs0fsel(&self) -> DTPRS0FSELR {
        DTPRS0FSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline]
    pub fn dtprs1fsel(&self) -> DTPRS1FSELR {
        DTPRS1FSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline]
    pub fn dtfa(&self) -> DTFAR {
        DTFAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline]
    pub fn dtprs0fen(&self) -> DTPRS0FENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTPRS0FENR { bits }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline]
    pub fn dtprs1fen(&self) -> DTPRS1FENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTPRS1FENR { bits }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline]
    pub fn dtdbgfen(&self) -> DTDBGFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTDBGFENR { bits }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline]
    pub fn dtlockupfen(&self) -> DTLOCKUPFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTLOCKUPFENR { bits }
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
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline]
    pub fn dtprs0fsel(&mut self) -> _DTPRS0FSELW {
        _DTPRS0FSELW { w: self }
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline]
    pub fn dtprs1fsel(&mut self) -> _DTPRS1FSELW {
        _DTPRS1FSELW { w: self }
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline]
    pub fn dtfa(&mut self) -> _DTFAW {
        _DTFAW { w: self }
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline]
    pub fn dtprs0fen(&mut self) -> _DTPRS0FENW {
        _DTPRS0FENW { w: self }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline]
    pub fn dtprs1fen(&mut self) -> _DTPRS1FENW {
        _DTPRS1FENW { w: self }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline]
    pub fn dtdbgfen(&mut self) -> _DTDBGFENW {
        _DTDBGFENW { w: self }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline]
    pub fn dtlockupfen(&mut self) -> _DTLOCKUPFENW {
        _DTLOCKUPFENW { w: self }
    }
}
