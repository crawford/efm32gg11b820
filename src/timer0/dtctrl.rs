#[doc = "Reader of register DTCTRL"]
pub type R = crate::R<u32, super::DTCTRL>;
#[doc = "Writer for register DTCTRL"]
pub type W = crate::W<u32, super::DTCTRL>;
#[doc = "Register DTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTEN`"]
pub type DTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTEN`"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
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
#[doc = "Reader of field `DTDAS`"]
pub type DTDAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTDAS`"]
pub struct DTDAS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDAS_W<'a> {
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
#[doc = "Reader of field `DTIPOL`"]
pub type DTIPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIPOL`"]
pub struct DTIPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIPOL_W<'a> {
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
#[doc = "Reader of field `DTCINV`"]
pub type DTCINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCINV`"]
pub struct DTCINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCINV_W<'a> {
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
#[doc = "Possible values of the field `DTPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRSSEL_A {
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
impl crate::ToBits<u8> for DTPRSSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTPRSSEL_A::PRSCH0 => 0,
            DTPRSSEL_A::PRSCH1 => 1,
            DTPRSSEL_A::PRSCH2 => 2,
            DTPRSSEL_A::PRSCH3 => 3,
            DTPRSSEL_A::PRSCH4 => 4,
            DTPRSSEL_A::PRSCH5 => 5,
            DTPRSSEL_A::PRSCH6 => 6,
            DTPRSSEL_A::PRSCH7 => 7,
            DTPRSSEL_A::PRSCH8 => 8,
            DTPRSSEL_A::PRSCH9 => 9,
            DTPRSSEL_A::PRSCH10 => 10,
            DTPRSSEL_A::PRSCH11 => 11,
            DTPRSSEL_A::PRSCH12 => 12,
            DTPRSSEL_A::PRSCH13 => 13,
            DTPRSSEL_A::PRSCH14 => 14,
            DTPRSSEL_A::PRSCH15 => 15,
            DTPRSSEL_A::PRSCH16 => 16,
            DTPRSSEL_A::PRSCH17 => 17,
            DTPRSSEL_A::PRSCH18 => 18,
            DTPRSSEL_A::PRSCH19 => 19,
            DTPRSSEL_A::PRSCH20 => 20,
            DTPRSSEL_A::PRSCH21 => 21,
            DTPRSSEL_A::PRSCH22 => 22,
            DTPRSSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `DTPRSSEL`"]
pub type DTPRSSEL_R = crate::R<u8, DTPRSSEL_A>;
impl DTPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPRSSEL_A::PRSCH0),
            1 => Val(DTPRSSEL_A::PRSCH1),
            2 => Val(DTPRSSEL_A::PRSCH2),
            3 => Val(DTPRSSEL_A::PRSCH3),
            4 => Val(DTPRSSEL_A::PRSCH4),
            5 => Val(DTPRSSEL_A::PRSCH5),
            6 => Val(DTPRSSEL_A::PRSCH6),
            7 => Val(DTPRSSEL_A::PRSCH7),
            8 => Val(DTPRSSEL_A::PRSCH8),
            9 => Val(DTPRSSEL_A::PRSCH9),
            10 => Val(DTPRSSEL_A::PRSCH10),
            11 => Val(DTPRSSEL_A::PRSCH11),
            12 => Val(DTPRSSEL_A::PRSCH12),
            13 => Val(DTPRSSEL_A::PRSCH13),
            14 => Val(DTPRSSEL_A::PRSCH14),
            15 => Val(DTPRSSEL_A::PRSCH15),
            16 => Val(DTPRSSEL_A::PRSCH16),
            17 => Val(DTPRSSEL_A::PRSCH17),
            18 => Val(DTPRSSEL_A::PRSCH18),
            19 => Val(DTPRSSEL_A::PRSCH19),
            20 => Val(DTPRSSEL_A::PRSCH20),
            21 => Val(DTPRSSEL_A::PRSCH21),
            22 => Val(DTPRSSEL_A::PRSCH22),
            23 => Val(DTPRSSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `DTPRSSEL`"]
pub struct DTPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRSSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DTAR`"]
pub type DTAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTAR`"]
pub struct DTAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DTFATS`"]
pub type DTFATS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTFATS`"]
pub struct DTFATS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFATS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DTPRSEN`"]
pub type DTPRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRSEN`"]
pub struct DTPRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DTPRSSEL_R {
        DTPRSSEL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DTAR_R {
        DTAR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DTFATS_R {
        DTFATS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DTDAS_W {
        DTDAS_W { w: self }
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DTIPOL_W {
        DTIPOL_W { w: self }
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DTCINV_W {
        DTCINV_W { w: self }
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DTPRSSEL_W {
        DTPRSSEL_W { w: self }
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&mut self) -> DTAR_W {
        DTAR_W { w: self }
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&mut self) -> DTFATS_W {
        DTFATS_W { w: self }
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DTPRSEN_W {
        DTPRSEN_W { w: self }
    }
}
