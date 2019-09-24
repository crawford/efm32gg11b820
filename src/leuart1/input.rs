#[doc = "Reader of register INPUT"]
pub type R = crate::R<u32, super::INPUT>;
#[doc = "Writer for register INPUT"]
pub type W = crate::W<u32, super::INPUT>;
#[doc = "Register INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "12: PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "13: PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "14: PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "15: PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "16: PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "17: PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "18: PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "19: PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "20: PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "21: PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "22: PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "23: PRS Channel 23 selected"]
    PRSCH23,
}
impl From<RXPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPRSSEL_A) -> Self {
        match variant {
            RXPRSSEL_A::PRSCH0 => 0,
            RXPRSSEL_A::PRSCH1 => 1,
            RXPRSSEL_A::PRSCH2 => 2,
            RXPRSSEL_A::PRSCH3 => 3,
            RXPRSSEL_A::PRSCH4 => 4,
            RXPRSSEL_A::PRSCH5 => 5,
            RXPRSSEL_A::PRSCH6 => 6,
            RXPRSSEL_A::PRSCH7 => 7,
            RXPRSSEL_A::PRSCH8 => 8,
            RXPRSSEL_A::PRSCH9 => 9,
            RXPRSSEL_A::PRSCH10 => 10,
            RXPRSSEL_A::PRSCH11 => 11,
            RXPRSSEL_A::PRSCH12 => 12,
            RXPRSSEL_A::PRSCH13 => 13,
            RXPRSSEL_A::PRSCH14 => 14,
            RXPRSSEL_A::PRSCH15 => 15,
            RXPRSSEL_A::PRSCH16 => 16,
            RXPRSSEL_A::PRSCH17 => 17,
            RXPRSSEL_A::PRSCH18 => 18,
            RXPRSSEL_A::PRSCH19 => 19,
            RXPRSSEL_A::PRSCH20 => 20,
            RXPRSSEL_A::PRSCH21 => 21,
            RXPRSSEL_A::PRSCH22 => 22,
            RXPRSSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `RXPRSSEL`"]
pub type RXPRSSEL_R = crate::R<u8, RXPRSSEL_A>;
impl RXPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPRSSEL_A::PRSCH0),
            1 => Val(RXPRSSEL_A::PRSCH1),
            2 => Val(RXPRSSEL_A::PRSCH2),
            3 => Val(RXPRSSEL_A::PRSCH3),
            4 => Val(RXPRSSEL_A::PRSCH4),
            5 => Val(RXPRSSEL_A::PRSCH5),
            6 => Val(RXPRSSEL_A::PRSCH6),
            7 => Val(RXPRSSEL_A::PRSCH7),
            8 => Val(RXPRSSEL_A::PRSCH8),
            9 => Val(RXPRSSEL_A::PRSCH9),
            10 => Val(RXPRSSEL_A::PRSCH10),
            11 => Val(RXPRSSEL_A::PRSCH11),
            12 => Val(RXPRSSEL_A::PRSCH12),
            13 => Val(RXPRSSEL_A::PRSCH13),
            14 => Val(RXPRSSEL_A::PRSCH14),
            15 => Val(RXPRSSEL_A::PRSCH15),
            16 => Val(RXPRSSEL_A::PRSCH16),
            17 => Val(RXPRSSEL_A::PRSCH17),
            18 => Val(RXPRSSEL_A::PRSCH18),
            19 => Val(RXPRSSEL_A::PRSCH19),
            20 => Val(RXPRSSEL_A::PRSCH20),
            21 => Val(RXPRSSEL_A::PRSCH21),
            22 => Val(RXPRSSEL_A::PRSCH22),
            23 => Val(RXPRSSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `RXPRSSEL`"]
pub struct RXPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `RXPRS`"]
pub type RXPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPRS`"]
pub struct RXPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRS_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RXPRSSEL_R {
        RXPRSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RXPRS_R {
        RXPRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RXPRSSEL_W {
        RXPRSSEL_W { w: self }
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RXPRS_W {
        RXPRS_W { w: self }
    }
}
