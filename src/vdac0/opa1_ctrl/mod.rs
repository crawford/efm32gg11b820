#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPA1_CTRL {
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
#[doc = "Possible values of the field `DRIVESTRENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIVESTRENGTHR {
    #[doc = "Lower accuracy with Low drive strength."]
    _0,
    #[doc = "Low accuracy with Low drive strength."]
    _1,
    #[doc = "High accuracy with High drive strength."]
    _2,
    #[doc = "Higher accuracy with High drive strength."]
    _3,
}
impl DRIVESTRENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRIVESTRENGTHR::_0 => 0,
            DRIVESTRENGTHR::_1 => 1,
            DRIVESTRENGTHR::_2 => 2,
            DRIVESTRENGTHR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRIVESTRENGTHR {
        match value {
            0 => DRIVESTRENGTHR::_0,
            1 => DRIVESTRENGTHR::_1,
            2 => DRIVESTRENGTHR::_2,
            3 => DRIVESTRENGTHR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DRIVESTRENGTHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DRIVESTRENGTHR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == DRIVESTRENGTHR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == DRIVESTRENGTHR::_3
    }
}
#[doc = r" Value of the field"]
pub struct INCBWR {
    bits: bool,
}
impl INCBWR {
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
pub struct HCMDISR {
    bits: bool,
}
impl HCMDISR {
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
pub struct OUTSCALER {
    bits: bool,
}
impl OUTSCALER {
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
pub struct PRSENR {
    bits: bool,
}
impl PRSENR {
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
pub struct PRSMODER {
    bits: bool,
}
impl PRSMODER {
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
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS ch 0 triggers OPA."]
    PRSCH0,
    #[doc = "PRS ch 1 triggers OPA."]
    PRSCH1,
    #[doc = "PRS ch 2 triggers OPA."]
    PRSCH2,
    #[doc = "PRS ch 3 triggers OPA."]
    PRSCH3,
    #[doc = "PRS ch 4 triggers OPA."]
    PRSCH4,
    #[doc = "PRS ch 5 triggers OPA."]
    PRSCH5,
    #[doc = "PRS ch 6 triggers OPA."]
    PRSCH6,
    #[doc = "PRS ch 7 triggers OPA."]
    PRSCH7,
    #[doc = "PRS ch 8 triggers OPA."]
    PRSCH8,
    #[doc = "PRS ch 9 triggers OPA."]
    PRSCH9,
    #[doc = "PRS ch 10 triggers OPA."]
    PRSCH10,
    #[doc = "PRS ch 11 triggers OPA."]
    PRSCH11,
    #[doc = "PRS ch 12 triggers OPA."]
    PRSCH12,
    #[doc = "PRS ch 13 triggers OPA."]
    PRSCH13,
    #[doc = "PRS ch 14 triggers OPA."]
    PRSCH14,
    #[doc = "PRS ch 15 triggers OPA."]
    PRSCH15,
    #[doc = "PRS ch 16 triggers OPA."]
    PRSCH16,
    #[doc = "PRS ch 17 triggers OPA."]
    PRSCH17,
    #[doc = "PRS ch 18 triggers OPA."]
    PRSCH18,
    #[doc = "PRS ch 19 triggers OPA."]
    PRSCH19,
    #[doc = "PRS ch 20 triggers OPA."]
    PRSCH20,
    #[doc = "PRS ch 21 triggers OPA."]
    PRSCH21,
    #[doc = "PRS ch 22 triggers OPA."]
    PRSCH22,
    #[doc = "PRS ch 23 triggers OPA."]
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
#[doc = r" Value of the field"]
pub struct PRSOUTMODER {
    bits: bool,
}
impl PRSOUTMODER {
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
pub struct APORTXMASTERDISR {
    bits: bool,
}
impl APORTXMASTERDISR {
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
pub struct APORTYMASTERDISR {
    bits: bool,
}
impl APORTYMASTERDISR {
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
#[doc = "Values that can be written to the field `DRIVESTRENGTH`"]
pub enum DRIVESTRENGTHW {
    #[doc = "Lower accuracy with Low drive strength."]
    _0,
    #[doc = "Low accuracy with Low drive strength."]
    _1,
    #[doc = "High accuracy with High drive strength."]
    _2,
    #[doc = "Higher accuracy with High drive strength."]
    _3,
}
impl DRIVESTRENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRIVESTRENGTHW::_0 => 0,
            DRIVESTRENGTHW::_1 => 1,
            DRIVESTRENGTHW::_2 => 2,
            DRIVESTRENGTHW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRIVESTRENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIVESTRENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRIVESTRENGTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRIVESTRENGTHW::_0)
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRIVESTRENGTHW::_1)
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(DRIVESTRENGTHW::_2)
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(DRIVESTRENGTHW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INCBWW<'a> {
    w: &'a mut W,
}
impl<'a> _INCBWW<'a> {
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
pub struct _HCMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _HCMDISW<'a> {
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
pub struct _OUTSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTSCALEW<'a> {
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
pub struct _PRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSENW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSMODEW<'a> {
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
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS ch 0 triggers OPA."]
    PRSCH0,
    #[doc = "PRS ch 1 triggers OPA."]
    PRSCH1,
    #[doc = "PRS ch 2 triggers OPA."]
    PRSCH2,
    #[doc = "PRS ch 3 triggers OPA."]
    PRSCH3,
    #[doc = "PRS ch 4 triggers OPA."]
    PRSCH4,
    #[doc = "PRS ch 5 triggers OPA."]
    PRSCH5,
    #[doc = "PRS ch 6 triggers OPA."]
    PRSCH6,
    #[doc = "PRS ch 7 triggers OPA."]
    PRSCH7,
    #[doc = "PRS ch 8 triggers OPA."]
    PRSCH8,
    #[doc = "PRS ch 9 triggers OPA."]
    PRSCH9,
    #[doc = "PRS ch 10 triggers OPA."]
    PRSCH10,
    #[doc = "PRS ch 11 triggers OPA."]
    PRSCH11,
    #[doc = "PRS ch 12 triggers OPA."]
    PRSCH12,
    #[doc = "PRS ch 13 triggers OPA."]
    PRSCH13,
    #[doc = "PRS ch 14 triggers OPA."]
    PRSCH14,
    #[doc = "PRS ch 15 triggers OPA."]
    PRSCH15,
    #[doc = "PRS ch 16 triggers OPA."]
    PRSCH16,
    #[doc = "PRS ch 17 triggers OPA."]
    PRSCH17,
    #[doc = "PRS ch 18 triggers OPA."]
    PRSCH18,
    #[doc = "PRS ch 19 triggers OPA."]
    PRSCH19,
    #[doc = "PRS ch 20 triggers OPA."]
    PRSCH20,
    #[doc = "PRS ch 21 triggers OPA."]
    PRSCH21,
    #[doc = "PRS ch 22 triggers OPA."]
    PRSCH22,
    #[doc = "PRS ch 23 triggers OPA."]
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
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers OPA."]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers OPA."]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers OPA."]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers OPA."]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers OPA."]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers OPA."]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers OPA."]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers OPA."]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH23)
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
#[doc = r" Proxy"]
pub struct _PRSOUTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSOUTMODEW<'a> {
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
pub struct _APORTXMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTXMASTERDISW<'a> {
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
pub struct _APORTYMASTERDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTYMASTERDISW<'a> {
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
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline]
    pub fn drivestrength(&self) -> DRIVESTRENGTHR {
        DRIVESTRENGTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline]
    pub fn incbw(&self) -> INCBWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INCBWR { bits }
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline]
    pub fn hcmdis(&self) -> HCMDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HCMDISR { bits }
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline]
    pub fn outscale(&self) -> OUTSCALER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTSCALER { bits }
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&self) -> PRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSENR { bits }
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline]
    pub fn prsmode(&self) -> PRSMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSMODER { bits }
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline]
    pub fn prsoutmode(&self) -> PRSOUTMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSOUTMODER { bits }
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTXMASTERDISR { bits }
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportymasterdis(&self) -> APORTYMASTERDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTYMASTERDISR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 14 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline]
    pub fn drivestrength(&mut self) -> _DRIVESTRENGTHW {
        _DRIVESTRENGTHW { w: self }
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline]
    pub fn incbw(&mut self) -> _INCBWW {
        _INCBWW { w: self }
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline]
    pub fn hcmdis(&mut self) -> _HCMDISW {
        _HCMDISW { w: self }
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline]
    pub fn outscale(&mut self) -> _OUTSCALEW {
        _OUTSCALEW { w: self }
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&mut self) -> _PRSENW {
        _PRSENW { w: self }
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline]
    pub fn prsmode(&mut self) -> _PRSMODEW {
        _PRSMODEW { w: self }
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline]
    pub fn prsoutmode(&mut self) -> _PRSOUTMODEW {
        _PRSOUTMODEW { w: self }
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportxmasterdis(&mut self) -> _APORTXMASTERDISW {
        _APORTXMASTERDISW { w: self }
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline]
    pub fn aportymasterdis(&mut self) -> _APORTYMASTERDISW {
        _APORTYMASTERDISW { w: self }
    }
}
