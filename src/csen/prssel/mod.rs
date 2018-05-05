#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRSSEL {
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
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS Channel 0 selected as the start trigger"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as the start trigger"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as the start trigger"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as the start trigger"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as the start trigger"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as the start trigger"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as the start trigger"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as the start trigger"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as the start trigger"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as the start trigger"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as the start trigger"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as the start trigger"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as the start trigger"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as the start trigger"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as the start trigger"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as the start trigger"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as the start trigger"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as the start trigger"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as the start trigger"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as the start trigger"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as the start trigger"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as the start trigger"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as the start trigger"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as the start trigger"]
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
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS Channel 0 selected as the start trigger"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as the start trigger"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as the start trigger"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as the start trigger"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as the start trigger"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as the start trigger"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as the start trigger"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as the start trigger"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected as the start trigger"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected as the start trigger"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected as the start trigger"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected as the start trigger"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected as the start trigger"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected as the start trigger"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected as the start trigger"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected as the start trigger"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected as the start trigger"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected as the start trigger"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected as the start trigger"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected as the start trigger"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected as the start trigger"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected as the start trigger"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected as the start trigger"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected as the start trigger"]
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
    #[doc = "PRS Channel 0 selected as the start trigger"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as the start trigger"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as the start trigger"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as the start trigger"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as the start trigger"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as the start trigger"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as the start trigger"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as the start trigger"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as the start trigger"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as the start trigger"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as the start trigger"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as the start trigger"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as the start trigger"]
    #[inline]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as the start trigger"]
    #[inline]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as the start trigger"]
    #[inline]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as the start trigger"]
    #[inline]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as the start trigger"]
    #[inline]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as the start trigger"]
    #[inline]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as the start trigger"]
    #[inline]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as the start trigger"]
    #[inline]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as the start trigger"]
    #[inline]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as the start trigger"]
    #[inline]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as the start trigger"]
    #[inline]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as the start trigger"]
    #[inline]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH23)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
}
