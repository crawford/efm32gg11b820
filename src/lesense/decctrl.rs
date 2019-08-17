#[doc = "Reader of register DECCTRL"]
pub type R = crate::R<u32, super::DECCTRL>;
#[doc = "Writer for register DECCTRL"]
pub type W = crate::W<u32, super::DECCTRL>;
#[doc = "Register DECCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DECCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DISABLE`"]
pub type DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE`"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ERRCHK`"]
pub type ERRCHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRCHK`"]
pub struct ERRCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCHK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INTMAP`"]
pub type INTMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMAP`"]
pub struct INTMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `HYSTPRS0`"]
pub type HYSTPRS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTPRS0`"]
pub struct HYSTPRS0_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTPRS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HYSTPRS1`"]
pub type HYSTPRS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTPRS1`"]
pub struct HYSTPRS1_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTPRS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `HYSTPRS2`"]
pub type HYSTPRS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTPRS2`"]
pub struct HYSTPRS2_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTPRS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HYSTIRQ`"]
pub type HYSTIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTIRQ`"]
pub struct HYSTIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTIRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRSCNT`"]
pub type PRSCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSCNT`"]
pub struct PRSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `INPUT`"]
pub type INPUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INPUT`"]
pub struct INPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `PRSSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL0_A {
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
impl crate::ToBits<u8> for PRSSEL0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRSSEL0_A::PRSCH0 => 0,
            PRSSEL0_A::PRSCH1 => 1,
            PRSSEL0_A::PRSCH2 => 2,
            PRSSEL0_A::PRSCH3 => 3,
            PRSSEL0_A::PRSCH4 => 4,
            PRSSEL0_A::PRSCH5 => 5,
            PRSSEL0_A::PRSCH6 => 6,
            PRSSEL0_A::PRSCH7 => 7,
            PRSSEL0_A::PRSCH8 => 8,
            PRSSEL0_A::PRSCH9 => 9,
            PRSSEL0_A::PRSCH10 => 10,
            PRSSEL0_A::PRSCH11 => 11,
            PRSSEL0_A::PRSCH12 => 12,
            PRSSEL0_A::PRSCH13 => 13,
            PRSSEL0_A::PRSCH14 => 14,
            PRSSEL0_A::PRSCH15 => 15,
            PRSSEL0_A::PRSCH16 => 16,
            PRSSEL0_A::PRSCH17 => 17,
            PRSSEL0_A::PRSCH18 => 18,
            PRSSEL0_A::PRSCH19 => 19,
            PRSSEL0_A::PRSCH20 => 20,
            PRSSEL0_A::PRSCH21 => 21,
            PRSSEL0_A::PRSCH22 => 22,
            PRSSEL0_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSSEL0`"]
pub type PRSSEL0_R = crate::R<u8, PRSSEL0_A>;
impl PRSSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL0_A::PRSCH0),
            1 => Val(PRSSEL0_A::PRSCH1),
            2 => Val(PRSSEL0_A::PRSCH2),
            3 => Val(PRSSEL0_A::PRSCH3),
            4 => Val(PRSSEL0_A::PRSCH4),
            5 => Val(PRSSEL0_A::PRSCH5),
            6 => Val(PRSSEL0_A::PRSCH6),
            7 => Val(PRSSEL0_A::PRSCH7),
            8 => Val(PRSSEL0_A::PRSCH8),
            9 => Val(PRSSEL0_A::PRSCH9),
            10 => Val(PRSSEL0_A::PRSCH10),
            11 => Val(PRSSEL0_A::PRSCH11),
            12 => Val(PRSSEL0_A::PRSCH12),
            13 => Val(PRSSEL0_A::PRSCH13),
            14 => Val(PRSSEL0_A::PRSCH14),
            15 => Val(PRSSEL0_A::PRSCH15),
            16 => Val(PRSSEL0_A::PRSCH16),
            17 => Val(PRSSEL0_A::PRSCH17),
            18 => Val(PRSSEL0_A::PRSCH18),
            19 => Val(PRSSEL0_A::PRSCH19),
            20 => Val(PRSSEL0_A::PRSCH20),
            21 => Val(PRSSEL0_A::PRSCH21),
            22 => Val(PRSSEL0_A::PRSCH22),
            23 => Val(PRSSEL0_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL0_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL0_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL0_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL0_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL0_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL0_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL0_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL0_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL0_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL0_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL0_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL0_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL0_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL0_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL0_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL0_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL0_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL0_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL0_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL0_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL0_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL0_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL0_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL0_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSSEL0`"]
pub struct PRSSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL0_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `PRSSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL1_A {
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
impl crate::ToBits<u8> for PRSSEL1_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRSSEL1_A::PRSCH0 => 0,
            PRSSEL1_A::PRSCH1 => 1,
            PRSSEL1_A::PRSCH2 => 2,
            PRSSEL1_A::PRSCH3 => 3,
            PRSSEL1_A::PRSCH4 => 4,
            PRSSEL1_A::PRSCH5 => 5,
            PRSSEL1_A::PRSCH6 => 6,
            PRSSEL1_A::PRSCH7 => 7,
            PRSSEL1_A::PRSCH8 => 8,
            PRSSEL1_A::PRSCH9 => 9,
            PRSSEL1_A::PRSCH10 => 10,
            PRSSEL1_A::PRSCH11 => 11,
            PRSSEL1_A::PRSCH12 => 12,
            PRSSEL1_A::PRSCH13 => 13,
            PRSSEL1_A::PRSCH14 => 14,
            PRSSEL1_A::PRSCH15 => 15,
            PRSSEL1_A::PRSCH16 => 16,
            PRSSEL1_A::PRSCH17 => 17,
            PRSSEL1_A::PRSCH18 => 18,
            PRSSEL1_A::PRSCH19 => 19,
            PRSSEL1_A::PRSCH20 => 20,
            PRSSEL1_A::PRSCH21 => 21,
            PRSSEL1_A::PRSCH22 => 22,
            PRSSEL1_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSSEL1`"]
pub type PRSSEL1_R = crate::R<u8, PRSSEL1_A>;
impl PRSSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL1_A::PRSCH0),
            1 => Val(PRSSEL1_A::PRSCH1),
            2 => Val(PRSSEL1_A::PRSCH2),
            3 => Val(PRSSEL1_A::PRSCH3),
            4 => Val(PRSSEL1_A::PRSCH4),
            5 => Val(PRSSEL1_A::PRSCH5),
            6 => Val(PRSSEL1_A::PRSCH6),
            7 => Val(PRSSEL1_A::PRSCH7),
            8 => Val(PRSSEL1_A::PRSCH8),
            9 => Val(PRSSEL1_A::PRSCH9),
            10 => Val(PRSSEL1_A::PRSCH10),
            11 => Val(PRSSEL1_A::PRSCH11),
            12 => Val(PRSSEL1_A::PRSCH12),
            13 => Val(PRSSEL1_A::PRSCH13),
            14 => Val(PRSSEL1_A::PRSCH14),
            15 => Val(PRSSEL1_A::PRSCH15),
            16 => Val(PRSSEL1_A::PRSCH16),
            17 => Val(PRSSEL1_A::PRSCH17),
            18 => Val(PRSSEL1_A::PRSCH18),
            19 => Val(PRSSEL1_A::PRSCH19),
            20 => Val(PRSSEL1_A::PRSCH20),
            21 => Val(PRSSEL1_A::PRSCH21),
            22 => Val(PRSSEL1_A::PRSCH22),
            23 => Val(PRSSEL1_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL1_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL1_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL1_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL1_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL1_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL1_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL1_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL1_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL1_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL1_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL1_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL1_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL1_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL1_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL1_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL1_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL1_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL1_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL1_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL1_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL1_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL1_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL1_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL1_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSSEL1`"]
pub struct PRSSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL1_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `PRSSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL2_A {
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
impl crate::ToBits<u8> for PRSSEL2_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRSSEL2_A::PRSCH0 => 0,
            PRSSEL2_A::PRSCH1 => 1,
            PRSSEL2_A::PRSCH2 => 2,
            PRSSEL2_A::PRSCH3 => 3,
            PRSSEL2_A::PRSCH4 => 4,
            PRSSEL2_A::PRSCH5 => 5,
            PRSSEL2_A::PRSCH6 => 6,
            PRSSEL2_A::PRSCH7 => 7,
            PRSSEL2_A::PRSCH8 => 8,
            PRSSEL2_A::PRSCH9 => 9,
            PRSSEL2_A::PRSCH10 => 10,
            PRSSEL2_A::PRSCH11 => 11,
            PRSSEL2_A::PRSCH12 => 12,
            PRSSEL2_A::PRSCH13 => 13,
            PRSSEL2_A::PRSCH14 => 14,
            PRSSEL2_A::PRSCH15 => 15,
            PRSSEL2_A::PRSCH16 => 16,
            PRSSEL2_A::PRSCH17 => 17,
            PRSSEL2_A::PRSCH18 => 18,
            PRSSEL2_A::PRSCH19 => 19,
            PRSSEL2_A::PRSCH20 => 20,
            PRSSEL2_A::PRSCH21 => 21,
            PRSSEL2_A::PRSCH22 => 22,
            PRSSEL2_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSSEL2`"]
pub type PRSSEL2_R = crate::R<u8, PRSSEL2_A>;
impl PRSSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL2_A::PRSCH0),
            1 => Val(PRSSEL2_A::PRSCH1),
            2 => Val(PRSSEL2_A::PRSCH2),
            3 => Val(PRSSEL2_A::PRSCH3),
            4 => Val(PRSSEL2_A::PRSCH4),
            5 => Val(PRSSEL2_A::PRSCH5),
            6 => Val(PRSSEL2_A::PRSCH6),
            7 => Val(PRSSEL2_A::PRSCH7),
            8 => Val(PRSSEL2_A::PRSCH8),
            9 => Val(PRSSEL2_A::PRSCH9),
            10 => Val(PRSSEL2_A::PRSCH10),
            11 => Val(PRSSEL2_A::PRSCH11),
            12 => Val(PRSSEL2_A::PRSCH12),
            13 => Val(PRSSEL2_A::PRSCH13),
            14 => Val(PRSSEL2_A::PRSCH14),
            15 => Val(PRSSEL2_A::PRSCH15),
            16 => Val(PRSSEL2_A::PRSCH16),
            17 => Val(PRSSEL2_A::PRSCH17),
            18 => Val(PRSSEL2_A::PRSCH18),
            19 => Val(PRSSEL2_A::PRSCH19),
            20 => Val(PRSSEL2_A::PRSCH20),
            21 => Val(PRSSEL2_A::PRSCH21),
            22 => Val(PRSSEL2_A::PRSCH22),
            23 => Val(PRSSEL2_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL2_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL2_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL2_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL2_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL2_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL2_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL2_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL2_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL2_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL2_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL2_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL2_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL2_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL2_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL2_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL2_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL2_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL2_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL2_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL2_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL2_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL2_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL2_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL2_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSSEL2`"]
pub struct PRSSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL2_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `PRSSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL3_A {
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
impl crate::ToBits<u8> for PRSSEL3_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRSSEL3_A::PRSCH0 => 0,
            PRSSEL3_A::PRSCH1 => 1,
            PRSSEL3_A::PRSCH2 => 2,
            PRSSEL3_A::PRSCH3 => 3,
            PRSSEL3_A::PRSCH4 => 4,
            PRSSEL3_A::PRSCH5 => 5,
            PRSSEL3_A::PRSCH6 => 6,
            PRSSEL3_A::PRSCH7 => 7,
            PRSSEL3_A::PRSCH8 => 8,
            PRSSEL3_A::PRSCH9 => 9,
            PRSSEL3_A::PRSCH10 => 10,
            PRSSEL3_A::PRSCH11 => 11,
            PRSSEL3_A::PRSCH12 => 12,
            PRSSEL3_A::PRSCH13 => 13,
            PRSSEL3_A::PRSCH14 => 14,
            PRSSEL3_A::PRSCH15 => 15,
            PRSSEL3_A::PRSCH16 => 16,
            PRSSEL3_A::PRSCH17 => 17,
            PRSSEL3_A::PRSCH18 => 18,
            PRSSEL3_A::PRSCH19 => 19,
            PRSSEL3_A::PRSCH20 => 20,
            PRSSEL3_A::PRSCH21 => 21,
            PRSSEL3_A::PRSCH22 => 22,
            PRSSEL3_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSSEL3`"]
pub type PRSSEL3_R = crate::R<u8, PRSSEL3_A>;
impl PRSSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSSEL3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSSEL3_A::PRSCH0),
            1 => Val(PRSSEL3_A::PRSCH1),
            2 => Val(PRSSEL3_A::PRSCH2),
            3 => Val(PRSSEL3_A::PRSCH3),
            4 => Val(PRSSEL3_A::PRSCH4),
            5 => Val(PRSSEL3_A::PRSCH5),
            6 => Val(PRSSEL3_A::PRSCH6),
            7 => Val(PRSSEL3_A::PRSCH7),
            8 => Val(PRSSEL3_A::PRSCH8),
            9 => Val(PRSSEL3_A::PRSCH9),
            10 => Val(PRSSEL3_A::PRSCH10),
            11 => Val(PRSSEL3_A::PRSCH11),
            12 => Val(PRSSEL3_A::PRSCH12),
            13 => Val(PRSSEL3_A::PRSCH13),
            14 => Val(PRSSEL3_A::PRSCH14),
            15 => Val(PRSSEL3_A::PRSCH15),
            16 => Val(PRSSEL3_A::PRSCH16),
            17 => Val(PRSSEL3_A::PRSCH17),
            18 => Val(PRSSEL3_A::PRSCH18),
            19 => Val(PRSSEL3_A::PRSCH19),
            20 => Val(PRSSEL3_A::PRSCH20),
            21 => Val(PRSSEL3_A::PRSCH21),
            22 => Val(PRSSEL3_A::PRSCH22),
            23 => Val(PRSSEL3_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL3_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL3_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL3_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL3_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL3_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL3_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL3_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL3_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL3_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL3_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL3_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL3_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL3_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL3_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL3_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL3_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL3_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL3_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL3_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL3_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL3_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL3_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL3_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL3_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSSEL3`"]
pub struct PRSSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSSEL3_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&self) -> ERRCHK_R {
        ERRCHK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&self) -> INTMAP_R {
        INTMAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> HYSTPRS0_R {
        HYSTPRS0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> HYSTPRS1_R {
        HYSTPRS1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> HYSTPRS2_R {
        HYSTPRS2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&self) -> HYSTIRQ_R {
        HYSTIRQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&self) -> PRSCNT_R {
        PRSCNT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&self) -> PRSSEL0_R {
        PRSSEL0_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&self) -> PRSSEL1_R {
        PRSSEL1_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&self) -> PRSSEL2_R {
        PRSSEL2_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&self) -> PRSSEL3_R {
        PRSSEL3_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&mut self) -> ERRCHK_W {
        ERRCHK_W { w: self }
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&mut self) -> INTMAP_W {
        INTMAP_W { w: self }
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&mut self) -> HYSTPRS0_W {
        HYSTPRS0_W { w: self }
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&mut self) -> HYSTPRS1_W {
        HYSTPRS1_W { w: self }
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&mut self) -> HYSTPRS2_W {
        HYSTPRS2_W { w: self }
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&mut self) -> HYSTIRQ_W {
        HYSTIRQ_W { w: self }
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&mut self) -> PRSCNT_W {
        PRSCNT_W { w: self }
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W {
        INPUT_W { w: self }
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&mut self) -> PRSSEL0_W {
        PRSSEL0_W { w: self }
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&mut self) -> PRSSEL1_W {
        PRSSEL1_W { w: self }
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&mut self) -> PRSSEL2_W {
        PRSSEL2_W { w: self }
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&mut self) -> PRSSEL3_W {
        PRSSEL3_W { w: self }
    }
}
