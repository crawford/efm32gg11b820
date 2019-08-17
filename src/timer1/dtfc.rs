#[doc = "Reader of register DTFC"]
pub type R = crate::R<u32, super::DTFC>;
#[doc = "Writer for register DTFC"]
pub type W = crate::W<u32, super::DTFC>;
#[doc = "Register DTFC `reset()`'s with value 0"]
impl crate::ResetValue for super::DTFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DTPRS0FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS0FSEL_A {
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
impl crate::ToBits<u8> for DTPRS0FSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTPRS0FSEL_A::PRSCH0 => 0,
            DTPRS0FSEL_A::PRSCH1 => 1,
            DTPRS0FSEL_A::PRSCH2 => 2,
            DTPRS0FSEL_A::PRSCH3 => 3,
            DTPRS0FSEL_A::PRSCH4 => 4,
            DTPRS0FSEL_A::PRSCH5 => 5,
            DTPRS0FSEL_A::PRSCH6 => 6,
            DTPRS0FSEL_A::PRSCH7 => 7,
            DTPRS0FSEL_A::PRSCH8 => 8,
            DTPRS0FSEL_A::PRSCH9 => 9,
            DTPRS0FSEL_A::PRSCH10 => 10,
            DTPRS0FSEL_A::PRSCH11 => 11,
            DTPRS0FSEL_A::PRSCH12 => 12,
            DTPRS0FSEL_A::PRSCH13 => 13,
            DTPRS0FSEL_A::PRSCH14 => 14,
            DTPRS0FSEL_A::PRSCH15 => 15,
            DTPRS0FSEL_A::PRSCH16 => 16,
            DTPRS0FSEL_A::PRSCH17 => 17,
            DTPRS0FSEL_A::PRSCH18 => 18,
            DTPRS0FSEL_A::PRSCH19 => 19,
            DTPRS0FSEL_A::PRSCH20 => 20,
            DTPRS0FSEL_A::PRSCH21 => 21,
            DTPRS0FSEL_A::PRSCH22 => 22,
            DTPRS0FSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `DTPRS0FSEL`"]
pub type DTPRS0FSEL_R = crate::R<u8, DTPRS0FSEL_A>;
impl DTPRS0FSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPRS0FSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPRS0FSEL_A::PRSCH0),
            1 => Val(DTPRS0FSEL_A::PRSCH1),
            2 => Val(DTPRS0FSEL_A::PRSCH2),
            3 => Val(DTPRS0FSEL_A::PRSCH3),
            4 => Val(DTPRS0FSEL_A::PRSCH4),
            5 => Val(DTPRS0FSEL_A::PRSCH5),
            6 => Val(DTPRS0FSEL_A::PRSCH6),
            7 => Val(DTPRS0FSEL_A::PRSCH7),
            8 => Val(DTPRS0FSEL_A::PRSCH8),
            9 => Val(DTPRS0FSEL_A::PRSCH9),
            10 => Val(DTPRS0FSEL_A::PRSCH10),
            11 => Val(DTPRS0FSEL_A::PRSCH11),
            12 => Val(DTPRS0FSEL_A::PRSCH12),
            13 => Val(DTPRS0FSEL_A::PRSCH13),
            14 => Val(DTPRS0FSEL_A::PRSCH14),
            15 => Val(DTPRS0FSEL_A::PRSCH15),
            16 => Val(DTPRS0FSEL_A::PRSCH16),
            17 => Val(DTPRS0FSEL_A::PRSCH17),
            18 => Val(DTPRS0FSEL_A::PRSCH18),
            19 => Val(DTPRS0FSEL_A::PRSCH19),
            20 => Val(DTPRS0FSEL_A::PRSCH20),
            21 => Val(DTPRS0FSEL_A::PRSCH21),
            22 => Val(DTPRS0FSEL_A::PRSCH22),
            23 => Val(DTPRS0FSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `DTPRS0FSEL`"]
pub struct DTPRS0FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRS0FSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 2"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 3"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 4"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 5"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 6"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 7"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 8"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 9"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 10"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 11"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 12"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 13"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 14"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 15"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 16"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 17"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 18"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 19"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 20"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 21"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 22"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 23"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `DTPRS1FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS1FSEL_A {
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
impl crate::ToBits<u8> for DTPRS1FSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTPRS1FSEL_A::PRSCH0 => 0,
            DTPRS1FSEL_A::PRSCH1 => 1,
            DTPRS1FSEL_A::PRSCH2 => 2,
            DTPRS1FSEL_A::PRSCH3 => 3,
            DTPRS1FSEL_A::PRSCH4 => 4,
            DTPRS1FSEL_A::PRSCH5 => 5,
            DTPRS1FSEL_A::PRSCH6 => 6,
            DTPRS1FSEL_A::PRSCH7 => 7,
            DTPRS1FSEL_A::PRSCH8 => 8,
            DTPRS1FSEL_A::PRSCH9 => 9,
            DTPRS1FSEL_A::PRSCH10 => 10,
            DTPRS1FSEL_A::PRSCH11 => 11,
            DTPRS1FSEL_A::PRSCH12 => 12,
            DTPRS1FSEL_A::PRSCH13 => 13,
            DTPRS1FSEL_A::PRSCH14 => 14,
            DTPRS1FSEL_A::PRSCH15 => 15,
            DTPRS1FSEL_A::PRSCH16 => 16,
            DTPRS1FSEL_A::PRSCH17 => 17,
            DTPRS1FSEL_A::PRSCH18 => 18,
            DTPRS1FSEL_A::PRSCH19 => 19,
            DTPRS1FSEL_A::PRSCH20 => 20,
            DTPRS1FSEL_A::PRSCH21 => 21,
            DTPRS1FSEL_A::PRSCH22 => 22,
            DTPRS1FSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `DTPRS1FSEL`"]
pub type DTPRS1FSEL_R = crate::R<u8, DTPRS1FSEL_A>;
impl DTPRS1FSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPRS1FSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPRS1FSEL_A::PRSCH0),
            1 => Val(DTPRS1FSEL_A::PRSCH1),
            2 => Val(DTPRS1FSEL_A::PRSCH2),
            3 => Val(DTPRS1FSEL_A::PRSCH3),
            4 => Val(DTPRS1FSEL_A::PRSCH4),
            5 => Val(DTPRS1FSEL_A::PRSCH5),
            6 => Val(DTPRS1FSEL_A::PRSCH6),
            7 => Val(DTPRS1FSEL_A::PRSCH7),
            8 => Val(DTPRS1FSEL_A::PRSCH8),
            9 => Val(DTPRS1FSEL_A::PRSCH9),
            10 => Val(DTPRS1FSEL_A::PRSCH10),
            11 => Val(DTPRS1FSEL_A::PRSCH11),
            12 => Val(DTPRS1FSEL_A::PRSCH12),
            13 => Val(DTPRS1FSEL_A::PRSCH13),
            14 => Val(DTPRS1FSEL_A::PRSCH14),
            15 => Val(DTPRS1FSEL_A::PRSCH15),
            16 => Val(DTPRS1FSEL_A::PRSCH16),
            17 => Val(DTPRS1FSEL_A::PRSCH17),
            18 => Val(DTPRS1FSEL_A::PRSCH18),
            19 => Val(DTPRS1FSEL_A::PRSCH19),
            20 => Val(DTPRS1FSEL_A::PRSCH20),
            21 => Val(DTPRS1FSEL_A::PRSCH21),
            22 => Val(DTPRS1FSEL_A::PRSCH22),
            23 => Val(DTPRS1FSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `DTPRS1FSEL`"]
pub struct DTPRS1FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRS1FSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DTFA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFA_A {
    #[doc = "No action on fault"]
    NONE,
    #[doc = "Set outputs inactive"]
    INACTIVE,
    #[doc = "Clear outputs"]
    CLEAR,
    #[doc = "Tristate outputs"]
    TRISTATE,
}
impl crate::ToBits<u8> for DTFA_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTFA_A::NONE => 0,
            DTFA_A::INACTIVE => 1,
            DTFA_A::CLEAR => 2,
            DTFA_A::TRISTATE => 3,
        }
    }
}
#[doc = "Reader of field `DTFA`"]
pub type DTFA_R = crate::R<u8, DTFA_A>;
impl DTFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFA_A {
        match self.bits {
            0 => DTFA_A::NONE,
            1 => DTFA_A::INACTIVE,
            2 => DTFA_A::CLEAR,
            3 => DTFA_A::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA_A::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA_A::TRISTATE
    }
}
#[doc = "Write proxy for field `DTFA`"]
pub struct DTFA_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFA_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFA_A::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFA_A::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFA_A::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFA_A::TRISTATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DTPRS0FEN`"]
pub type DTPRS0FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRS0FEN`"]
pub struct DTPRS0FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DTPRS1FEN`"]
pub type DTPRS1FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRS1FEN`"]
pub struct DTPRS1FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DTDBGFEN`"]
pub type DTDBGFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTDBGFEN`"]
pub struct DTDBGFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDBGFEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DTLOCKUPFEN`"]
pub type DTLOCKUPFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTLOCKUPFEN`"]
pub struct DTLOCKUPFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLOCKUPFEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> DTPRS0FSEL_R {
        DTPRS0FSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> DTPRS1FSEL_R {
        DTPRS1FSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DTFA_R {
        DTFA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> DTPRS0FEN_R {
        DTPRS0FEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> DTPRS1FEN_R {
        DTPRS1FEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DTDBGFEN_R {
        DTDBGFEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DTLOCKUPFEN_R {
        DTLOCKUPFEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&mut self) -> DTPRS0FSEL_W {
        DTPRS0FSEL_W { w: self }
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&mut self) -> DTPRS1FSEL_W {
        DTPRS1FSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&mut self) -> DTFA_W {
        DTFA_W { w: self }
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&mut self) -> DTPRS0FEN_W {
        DTPRS0FEN_W { w: self }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&mut self) -> DTPRS1FEN_W {
        DTPRS1FEN_W { w: self }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&mut self) -> DTDBGFEN_W {
        DTDBGFEN_W { w: self }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&mut self) -> DTLOCKUPFEN_W {
        DTLOCKUPFEN_W { w: self }
    }
}
