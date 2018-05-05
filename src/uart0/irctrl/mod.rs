#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRCTRL {
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
#[doc = r" Value of the field"]
pub struct IRENR {
    bits: bool,
}
impl IRENR {
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
#[doc = "Possible values of the field `IRPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRPWR {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE,
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO,
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE,
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR,
}
impl IRPWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IRPWR::ONE => 0,
            IRPWR::TWO => 1,
            IRPWR::THREE => 2,
            IRPWR::FOUR => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IRPWR {
        match value {
            0 => IRPWR::ONE,
            1 => IRPWR::TWO,
            2 => IRPWR::THREE,
            3 => IRPWR::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == IRPWR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == IRPWR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == IRPWR::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == IRPWR::FOUR
    }
}
#[doc = r" Value of the field"]
pub struct IRFILTR {
    bits: bool,
}
impl IRFILTR {
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
pub struct IRPRSENR {
    bits: bool,
}
impl IRPRSENR {
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
#[doc = "Possible values of the field `IRPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRPRSSELR {
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
impl IRPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IRPRSSELR::PRSCH0 => 0,
            IRPRSSELR::PRSCH1 => 1,
            IRPRSSELR::PRSCH2 => 2,
            IRPRSSELR::PRSCH3 => 3,
            IRPRSSELR::PRSCH4 => 4,
            IRPRSSELR::PRSCH5 => 5,
            IRPRSSELR::PRSCH6 => 6,
            IRPRSSELR::PRSCH7 => 7,
            IRPRSSELR::PRSCH8 => 8,
            IRPRSSELR::PRSCH9 => 9,
            IRPRSSELR::PRSCH10 => 10,
            IRPRSSELR::PRSCH11 => 11,
            IRPRSSELR::PRSCH12 => 12,
            IRPRSSELR::PRSCH13 => 13,
            IRPRSSELR::PRSCH14 => 14,
            IRPRSSELR::PRSCH15 => 15,
            IRPRSSELR::PRSCH16 => 16,
            IRPRSSELR::PRSCH17 => 17,
            IRPRSSELR::PRSCH18 => 18,
            IRPRSSELR::PRSCH19 => 19,
            IRPRSSELR::PRSCH20 => 20,
            IRPRSSELR::PRSCH21 => 21,
            IRPRSSELR::PRSCH22 => 22,
            IRPRSSELR::PRSCH23 => 23,
            IRPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IRPRSSELR {
        match value {
            0 => IRPRSSELR::PRSCH0,
            1 => IRPRSSELR::PRSCH1,
            2 => IRPRSSELR::PRSCH2,
            3 => IRPRSSELR::PRSCH3,
            4 => IRPRSSELR::PRSCH4,
            5 => IRPRSSELR::PRSCH5,
            6 => IRPRSSELR::PRSCH6,
            7 => IRPRSSELR::PRSCH7,
            8 => IRPRSSELR::PRSCH8,
            9 => IRPRSSELR::PRSCH9,
            10 => IRPRSSELR::PRSCH10,
            11 => IRPRSSELR::PRSCH11,
            12 => IRPRSSELR::PRSCH12,
            13 => IRPRSSELR::PRSCH13,
            14 => IRPRSSELR::PRSCH14,
            15 => IRPRSSELR::PRSCH15,
            16 => IRPRSSELR::PRSCH16,
            17 => IRPRSSELR::PRSCH17,
            18 => IRPRSSELR::PRSCH18,
            19 => IRPRSSELR::PRSCH19,
            20 => IRPRSSELR::PRSCH20,
            21 => IRPRSSELR::PRSCH21,
            22 => IRPRSSELR::PRSCH22,
            23 => IRPRSSELR::PRSCH23,
            i => IRPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == IRPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == IRPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == IRPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == IRPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == IRPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == IRPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == IRPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == IRPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == IRPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == IRPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == IRPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == IRPRSSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == IRPRSSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == IRPRSSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == IRPRSSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == IRPRSSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == IRPRSSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == IRPRSSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == IRPRSSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == IRPRSSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == IRPRSSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == IRPRSSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == IRPRSSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == IRPRSSELR::PRSCH23
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
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
#[doc = "Values that can be written to the field `IRPW`"]
pub enum IRPWW {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE,
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO,
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE,
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR,
}
impl IRPWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IRPWW::ONE => 0,
            IRPWW::TWO => 1,
            IRPWW::THREE => 2,
            IRPWW::FOUR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRPWW<'a> {
    w: &'a mut W,
}
impl<'a> _IRPWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRPWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(IRPWW::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(IRPWW::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(IRPWW::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(IRPWW::FOUR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _IRFILTW<'a> {
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
pub struct _IRPRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRPRSENW<'a> {
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
#[doc = "Values that can be written to the field `IRPRSSEL`"]
pub enum IRPRSSELW {
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
impl IRPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IRPRSSELW::PRSCH0 => 0,
            IRPRSSELW::PRSCH1 => 1,
            IRPRSSELW::PRSCH2 => 2,
            IRPRSSELW::PRSCH3 => 3,
            IRPRSSELW::PRSCH4 => 4,
            IRPRSSELW::PRSCH5 => 5,
            IRPRSSELW::PRSCH6 => 6,
            IRPRSSELW::PRSCH7 => 7,
            IRPRSSELW::PRSCH8 => 8,
            IRPRSSELW::PRSCH9 => 9,
            IRPRSSELW::PRSCH10 => 10,
            IRPRSSELW::PRSCH11 => 11,
            IRPRSSELW::PRSCH12 => 12,
            IRPRSSELW::PRSCH13 => 13,
            IRPRSSELW::PRSCH14 => 14,
            IRPRSSELW::PRSCH15 => 15,
            IRPRSSELW::PRSCH16 => 16,
            IRPRSSELW::PRSCH17 => 17,
            IRPRSSELW::PRSCH18 => 18,
            IRPRSSELW::PRSCH19 => 19,
            IRPRSSELW::PRSCH20 => 20,
            IRPRSSELW::PRSCH21 => 21,
            IRPRSSELW::PRSCH22 => 22,
            IRPRSSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _IRPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(IRPRSSELW::PRSCH23)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline]
    pub fn iren(&self) -> IRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRENR { bits }
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline]
    pub fn irpw(&self) -> IRPWR {
        IRPWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline]
    pub fn irfilt(&self) -> IRFILTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRFILTR { bits }
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline]
    pub fn irprsen(&self) -> IRPRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRPRSENR { bits }
    }
    #[doc = "Bits 8:12 - IrDA PRS Channel Select"]
    #[inline]
    pub fn irprssel(&self) -> IRPRSSELR {
        IRPRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline]
    pub fn irpw(&mut self) -> _IRPWW {
        _IRPWW { w: self }
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline]
    pub fn irfilt(&mut self) -> _IRFILTW {
        _IRFILTW { w: self }
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline]
    pub fn irprsen(&mut self) -> _IRPRSENW {
        _IRPRSENW { w: self }
    }
    #[doc = "Bits 8:12 - IrDA PRS Channel Select"]
    #[inline]
    pub fn irprssel(&mut self) -> _IRPRSSELW {
        _IRPRSSELW { w: self }
    }
}
