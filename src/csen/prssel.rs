#[doc = "Reader of register PRSSEL"]
pub type R = crate::R<u32, super::PRSSEL>;
#[doc = "Writer for register PRSSEL"]
pub type W = crate::W<u32, super::PRSSEL>;
#[doc = "Register PRSSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRSSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL_A {
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
impl crate::ToBits<u8> for PRSSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRSSEL_A::PRSCH0 => 0,
            PRSSEL_A::PRSCH1 => 1,
            PRSSEL_A::PRSCH2 => 2,
            PRSSEL_A::PRSCH3 => 3,
            PRSSEL_A::PRSCH4 => 4,
            PRSSEL_A::PRSCH5 => 5,
            PRSSEL_A::PRSCH6 => 6,
            PRSSEL_A::PRSCH7 => 7,
            PRSSEL_A::PRSCH8 => 8,
            PRSSEL_A::PRSCH9 => 9,
            PRSSEL_A::PRSCH10 => 10,
            PRSSEL_A::PRSCH11 => 11,
            PRSSEL_A::PRSCH12 => 12,
            PRSSEL_A::PRSCH13 => 13,
            PRSSEL_A::PRSCH14 => 14,
            PRSSEL_A::PRSCH15 => 15,
            PRSSEL_A::PRSCH16 => 16,
            PRSSEL_A::PRSCH17 => 17,
            PRSSEL_A::PRSCH18 => 18,
            PRSSEL_A::PRSCH19 => 19,
            PRSSEL_A::PRSCH20 => 20,
            PRSSEL_A::PRSCH21 => 21,
            PRSSEL_A::PRSCH22 => 22,
            PRSSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSSEL`"]
pub type PRSSEL_R = crate::R<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL_A::PRSCH0),
            1 => Val(PRSSEL_A::PRSCH1),
            2 => Val(PRSSEL_A::PRSCH2),
            3 => Val(PRSSEL_A::PRSCH3),
            4 => Val(PRSSEL_A::PRSCH4),
            5 => Val(PRSSEL_A::PRSCH5),
            6 => Val(PRSSEL_A::PRSCH6),
            7 => Val(PRSSEL_A::PRSCH7),
            8 => Val(PRSSEL_A::PRSCH8),
            9 => Val(PRSSEL_A::PRSCH9),
            10 => Val(PRSSEL_A::PRSCH10),
            11 => Val(PRSSEL_A::PRSCH11),
            12 => Val(PRSSEL_A::PRSCH12),
            13 => Val(PRSSEL_A::PRSCH13),
            14 => Val(PRSSEL_A::PRSCH14),
            15 => Val(PRSSEL_A::PRSCH15),
            16 => Val(PRSSEL_A::PRSCH16),
            17 => Val(PRSSEL_A::PRSCH17),
            18 => Val(PRSSEL_A::PRSCH18),
            19 => Val(PRSSEL_A::PRSCH19),
            20 => Val(PRSSEL_A::PRSCH20),
            21 => Val(PRSSEL_A::PRSCH21),
            22 => Val(PRSSEL_A::PRSCH22),
            23 => Val(PRSSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSSEL`"]
pub struct PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
}
