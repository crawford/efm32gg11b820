#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH1CTRL {
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
pub struct CONVMODER {
    bits: bool,
}
impl CONVMODER {
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
#[doc = "Possible values of the field `TRIGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGMODER {
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    SW,
    #[doc = "Channel 1 is triggered by PRS input"]
    PRS,
    #[doc = "Channel 1 is triggered by Refresh timer"]
    REFRESH,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    SWPRS,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    SWREFRESH,
    #[doc = "Channel 1 is triggered by LESENSE"]
    LESENSE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGMODER::SW => 0,
            TRIGMODER::PRS => 1,
            TRIGMODER::REFRESH => 2,
            TRIGMODER::SWPRS => 3,
            TRIGMODER::SWREFRESH => 4,
            TRIGMODER::LESENSE => 5,
            TRIGMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGMODER {
        match value {
            0 => TRIGMODER::SW,
            1 => TRIGMODER::PRS,
            2 => TRIGMODER::REFRESH,
            3 => TRIGMODER::SWPRS,
            4 => TRIGMODER::SWREFRESH,
            5 => TRIGMODER::LESENSE,
            i => TRIGMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline]
    pub fn is_sw(&self) -> bool {
        *self == TRIGMODER::SW
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == TRIGMODER::PRS
    }
    #[doc = "Checks if the value of the field is `REFRESH`"]
    #[inline]
    pub fn is_refresh(&self) -> bool {
        *self == TRIGMODER::REFRESH
    }
    #[doc = "Checks if the value of the field is `SWPRS`"]
    #[inline]
    pub fn is_swprs(&self) -> bool {
        *self == TRIGMODER::SWPRS
    }
    #[doc = "Checks if the value of the field is `SWREFRESH`"]
    #[inline]
    pub fn is_swrefresh(&self) -> bool {
        *self == TRIGMODER::SWREFRESH
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline]
    pub fn is_lesense(&self) -> bool {
        *self == TRIGMODER::LESENSE
    }
}
#[doc = r" Value of the field"]
pub struct PRSASYNCR {
    bits: bool,
}
impl PRSASYNCR {
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
    #[doc = "PRS ch 0 triggers a conversion."]
    PRSCH0,
    #[doc = "PRS ch 1 triggers a conversion."]
    PRSCH1,
    #[doc = "PRS ch 2 triggers a conversion."]
    PRSCH2,
    #[doc = "PRS ch 3 triggers a conversion."]
    PRSCH3,
    #[doc = "PRS ch 4 triggers a conversion."]
    PRSCH4,
    #[doc = "PRS ch 5 triggers a conversion."]
    PRSCH5,
    #[doc = "PRS ch 6 triggers a conversion."]
    PRSCH6,
    #[doc = "PRS ch 7 triggers a conversion."]
    PRSCH7,
    #[doc = "PRS ch 8 triggers a conversion."]
    PRSCH8,
    #[doc = "PRS ch 9 triggers a conversion."]
    PRSCH9,
    #[doc = "PRS ch 10 triggers a conversion."]
    PRSCH10,
    #[doc = "PRS ch 11 triggers a conversion."]
    PRSCH11,
    #[doc = "PRS ch 12 triggers a conversion."]
    PRSCH12,
    #[doc = "PRS ch 13 triggers a conversion."]
    PRSCH13,
    #[doc = "PRS ch 14 triggers a conversion."]
    PRSCH14,
    #[doc = "PRS ch 15 triggers a conversion."]
    PRSCH15,
    #[doc = "PRS ch 16 triggers a conversion."]
    PRSCH16,
    #[doc = "PRS ch 17 triggers a conversion."]
    PRSCH17,
    #[doc = "PRS ch 18 triggers a conversion."]
    PRSCH18,
    #[doc = "PRS ch 19 triggers a conversion."]
    PRSCH19,
    #[doc = "PRS ch 20 triggers a conversion."]
    PRSCH20,
    #[doc = "PRS ch 21 triggers a conversion."]
    PRSCH21,
    #[doc = "PRS ch 22 triggers a conversion."]
    PRSCH22,
    #[doc = "PRS ch 23 triggers a conversion."]
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
#[doc = r" Proxy"]
pub struct _CONVMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVMODEW<'a> {
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
#[doc = "Values that can be written to the field `TRIGMODE`"]
pub enum TRIGMODEW {
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    SW,
    #[doc = "Channel 1 is triggered by PRS input"]
    PRS,
    #[doc = "Channel 1 is triggered by Refresh timer"]
    REFRESH,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    SWPRS,
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    SWREFRESH,
    #[doc = "Channel 1 is triggered by LESENSE"]
    LESENSE,
}
impl TRIGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGMODEW::SW => 0,
            TRIGMODEW::PRS => 1,
            TRIGMODEW::REFRESH => 2,
            TRIGMODEW::SWPRS => 3,
            TRIGMODEW::SWREFRESH => 4,
            TRIGMODEW::LESENSE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGMODEW::SW)
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(TRIGMODEW::PRS)
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline]
    pub fn refresh(self) -> &'a mut W {
        self.variant(TRIGMODEW::REFRESH)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline]
    pub fn swprs(self) -> &'a mut W {
        self.variant(TRIGMODEW::SWPRS)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline]
    pub fn swrefresh(self) -> &'a mut W {
        self.variant(TRIGMODEW::SWREFRESH)
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline]
    pub fn lesense(self) -> &'a mut W {
        self.variant(TRIGMODEW::LESENSE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRSASYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSASYNCW<'a> {
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
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS ch 0 triggers a conversion."]
    PRSCH0,
    #[doc = "PRS ch 1 triggers a conversion."]
    PRSCH1,
    #[doc = "PRS ch 2 triggers a conversion."]
    PRSCH2,
    #[doc = "PRS ch 3 triggers a conversion."]
    PRSCH3,
    #[doc = "PRS ch 4 triggers a conversion."]
    PRSCH4,
    #[doc = "PRS ch 5 triggers a conversion."]
    PRSCH5,
    #[doc = "PRS ch 6 triggers a conversion."]
    PRSCH6,
    #[doc = "PRS ch 7 triggers a conversion."]
    PRSCH7,
    #[doc = "PRS ch 8 triggers a conversion."]
    PRSCH8,
    #[doc = "PRS ch 9 triggers a conversion."]
    PRSCH9,
    #[doc = "PRS ch 10 triggers a conversion."]
    PRSCH10,
    #[doc = "PRS ch 11 triggers a conversion."]
    PRSCH11,
    #[doc = "PRS ch 12 triggers a conversion."]
    PRSCH12,
    #[doc = "PRS ch 13 triggers a conversion."]
    PRSCH13,
    #[doc = "PRS ch 14 triggers a conversion."]
    PRSCH14,
    #[doc = "PRS ch 15 triggers a conversion."]
    PRSCH15,
    #[doc = "PRS ch 16 triggers a conversion."]
    PRSCH16,
    #[doc = "PRS ch 17 triggers a conversion."]
    PRSCH17,
    #[doc = "PRS ch 18 triggers a conversion."]
    PRSCH18,
    #[doc = "PRS ch 19 triggers a conversion."]
    PRSCH19,
    #[doc = "PRS ch 20 triggers a conversion."]
    PRSCH20,
    #[doc = "PRS ch 21 triggers a conversion."]
    PRSCH21,
    #[doc = "PRS ch 22 triggers a conversion."]
    PRSCH22,
    #[doc = "PRS ch 23 triggers a conversion."]
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
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers a conversion."]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers a conversion."]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers a conversion."]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers a conversion."]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers a conversion."]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers a conversion."]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers a conversion."]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers a conversion."]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers a conversion."]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers a conversion."]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers a conversion."]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers a conversion."]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline]
    pub fn convmode(&self) -> CONVMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONVMODER { bits }
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline]
    pub fn trigmode(&self) -> TRIGMODER {
        TRIGMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline]
    pub fn prsasync(&self) -> PRSASYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSASYNCR { bits }
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline]
    pub fn convmode(&mut self) -> _CONVMODEW {
        _CONVMODEW { w: self }
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline]
    pub fn trigmode(&mut self) -> _TRIGMODEW {
        _TRIGMODEW { w: self }
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline]
    pub fn prsasync(&mut self) -> _PRSASYNCW {
        _PRSASYNCW { w: self }
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
}