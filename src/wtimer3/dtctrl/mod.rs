#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTCTRL {
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
pub struct DTENR {
    bits: bool,
}
impl DTENR {
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
pub struct DTDASR {
    bits: bool,
}
impl DTDASR {
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
pub struct DTIPOLR {
    bits: bool,
}
impl DTIPOLR {
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
pub struct DTCINVR {
    bits: bool,
}
impl DTCINVR {
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
#[doc = "Possible values of the field `DTPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRSSELR {
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
impl DTPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRSSELR::PRSCH0 => 0,
            DTPRSSELR::PRSCH1 => 1,
            DTPRSSELR::PRSCH2 => 2,
            DTPRSSELR::PRSCH3 => 3,
            DTPRSSELR::PRSCH4 => 4,
            DTPRSSELR::PRSCH5 => 5,
            DTPRSSELR::PRSCH6 => 6,
            DTPRSSELR::PRSCH7 => 7,
            DTPRSSELR::PRSCH8 => 8,
            DTPRSSELR::PRSCH9 => 9,
            DTPRSSELR::PRSCH10 => 10,
            DTPRSSELR::PRSCH11 => 11,
            DTPRSSELR::PRSCH12 => 12,
            DTPRSSELR::PRSCH13 => 13,
            DTPRSSELR::PRSCH14 => 14,
            DTPRSSELR::PRSCH15 => 15,
            DTPRSSELR::PRSCH16 => 16,
            DTPRSSELR::PRSCH17 => 17,
            DTPRSSELR::PRSCH18 => 18,
            DTPRSSELR::PRSCH19 => 19,
            DTPRSSELR::PRSCH20 => 20,
            DTPRSSELR::PRSCH21 => 21,
            DTPRSSELR::PRSCH22 => 22,
            DTPRSSELR::PRSCH23 => 23,
            DTPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRSSELR {
        match value {
            0 => DTPRSSELR::PRSCH0,
            1 => DTPRSSELR::PRSCH1,
            2 => DTPRSSELR::PRSCH2,
            3 => DTPRSSELR::PRSCH3,
            4 => DTPRSSELR::PRSCH4,
            5 => DTPRSSELR::PRSCH5,
            6 => DTPRSSELR::PRSCH6,
            7 => DTPRSSELR::PRSCH7,
            8 => DTPRSSELR::PRSCH8,
            9 => DTPRSSELR::PRSCH9,
            10 => DTPRSSELR::PRSCH10,
            11 => DTPRSSELR::PRSCH11,
            12 => DTPRSSELR::PRSCH12,
            13 => DTPRSSELR::PRSCH13,
            14 => DTPRSSELR::PRSCH14,
            15 => DTPRSSELR::PRSCH15,
            16 => DTPRSSELR::PRSCH16,
            17 => DTPRSSELR::PRSCH17,
            18 => DTPRSSELR::PRSCH18,
            19 => DTPRSSELR::PRSCH19,
            20 => DTPRSSELR::PRSCH20,
            21 => DTPRSSELR::PRSCH21,
            22 => DTPRSSELR::PRSCH22,
            23 => DTPRSSELR::PRSCH23,
            i => DTPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRSSELR::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRSSELR::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRSSELR::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRSSELR::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRSSELR::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRSSELR::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRSSELR::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRSSELR::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRSSELR::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRSSELR::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRSSELR::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRSSELR::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRSSELR::PRSCH23
    }
}
#[doc = r" Value of the field"]
pub struct DTARR {
    bits: bool,
}
impl DTARR {
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
pub struct DTFATSR {
    bits: bool,
}
impl DTFATSR {
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
pub struct DTPRSENR {
    bits: bool,
}
impl DTPRSENR {
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
pub struct _DTENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTENW<'a> {
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
pub struct _DTDASW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDASW<'a> {
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
pub struct _DTIPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DTIPOLW<'a> {
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
pub struct _DTCINVW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCINVW<'a> {
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
#[doc = "Values that can be written to the field `DTPRSSEL`"]
pub enum DTPRSSELW {
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
impl DTPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRSSELW::PRSCH0 => 0,
            DTPRSSELW::PRSCH1 => 1,
            DTPRSSELW::PRSCH2 => 2,
            DTPRSSELW::PRSCH3 => 3,
            DTPRSSELW::PRSCH4 => 4,
            DTPRSSELW::PRSCH5 => 5,
            DTPRSSELW::PRSCH6 => 6,
            DTPRSSELW::PRSCH7 => 7,
            DTPRSSELW::PRSCH8 => 8,
            DTPRSSELW::PRSCH9 => 9,
            DTPRSSELW::PRSCH10 => 10,
            DTPRSSELW::PRSCH11 => 11,
            DTPRSSELW::PRSCH12 => 12,
            DTPRSSELW::PRSCH13 => 13,
            DTPRSSELW::PRSCH14 => 14,
            DTPRSSELW::PRSCH15 => 15,
            DTPRSSELW::PRSCH16 => 16,
            DTPRSSELW::PRSCH17 => 17,
            DTPRSSELW::PRSCH18 => 18,
            DTPRSSELW::PRSCH19 => 19,
            DTPRSSELW::PRSCH20 => 20,
            DTPRSSELW::PRSCH21 => 21,
            DTPRSSELW::PRSCH22 => 22,
            DTPRSSELW::PRSCH23 => 23,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRSSELW::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTARW<'a> {
    w: &'a mut W,
}
impl<'a> _DTARW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTFATSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTFATSW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTPRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRSENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DTI Enable"]
    #[inline]
    pub fn dten(&self) -> DTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTENR { bits }
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline]
    pub fn dtdas(&self) -> DTDASR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTDASR { bits }
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline]
    pub fn dtipol(&self) -> DTIPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTIPOLR { bits }
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline]
    pub fn dtcinv(&self) -> DTCINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTCINVR { bits }
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline]
    pub fn dtprssel(&self) -> DTPRSSELR {
        DTPRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline]
    pub fn dtar(&self) -> DTARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTARR { bits }
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline]
    pub fn dtfats(&self) -> DTFATSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTFATSR { bits }
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline]
    pub fn dtprsen(&self) -> DTPRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTPRSENR { bits }
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
    #[doc = "Bit 0 - DTI Enable"]
    #[inline]
    pub fn dten(&mut self) -> _DTENW {
        _DTENW { w: self }
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline]
    pub fn dtdas(&mut self) -> _DTDASW {
        _DTDASW { w: self }
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline]
    pub fn dtipol(&mut self) -> _DTIPOLW {
        _DTIPOLW { w: self }
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline]
    pub fn dtcinv(&mut self) -> _DTCINVW {
        _DTCINVW { w: self }
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline]
    pub fn dtprssel(&mut self) -> _DTPRSSELW {
        _DTPRSSELW { w: self }
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline]
    pub fn dtar(&mut self) -> _DTARW {
        _DTARW { w: self }
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline]
    pub fn dtfats(&mut self) -> _DTFATSW {
        _DTFATSW { w: self }
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline]
    pub fn dtprsen(&mut self) -> _DTPRSENW {
        _DTPRSENW { w: self }
    }
}
