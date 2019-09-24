#[doc = "Reader of register TRIGCTRL"]
pub type R = crate::R<u32, super::TRIGCTRL>;
#[doc = "Writer for register TRIGCTRL"]
pub type W = crate::W<u32, super::TRIGCTRL>;
#[doc = "Register TRIGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXTEN`"]
pub type RXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTEN`"]
pub struct RXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTEN_W<'a> {
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
#[doc = "Reader of field `TXTEN`"]
pub type TXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTEN`"]
pub struct TXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTEN_W<'a> {
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
#[doc = "Reader of field `AUTOTXTEN`"]
pub type AUTOTXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTXTEN`"]
pub struct AUTOTXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTXTEN_W<'a> {
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
#[doc = "Reader of field `TXARX0EN`"]
pub type TXARX0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXARX0EN`"]
pub struct TXARX0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXARX0EN_W<'a> {
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
#[doc = "Reader of field `TXARX1EN`"]
pub type TXARX1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXARX1EN`"]
pub struct TXARX1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXARX1EN_W<'a> {
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
#[doc = "Reader of field `TXARX2EN`"]
pub type TXARX2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXARX2EN`"]
pub struct TXARX2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXARX2EN_W<'a> {
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
#[doc = "Reader of field `RXATX0EN`"]
pub type RXATX0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXATX0EN`"]
pub struct RXATX0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXATX0EN_W<'a> {
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
#[doc = "Reader of field `RXATX1EN`"]
pub type RXATX1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXATX1EN`"]
pub struct RXATX1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXATX1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RXATX2EN`"]
pub type RXATX2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXATX2EN`"]
pub struct RXATX2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXATX2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Trigger PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL_A {
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
impl From<TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL_A) -> Self {
        match variant {
            TSEL_A::PRSCH0 => 0,
            TSEL_A::PRSCH1 => 1,
            TSEL_A::PRSCH2 => 2,
            TSEL_A::PRSCH3 => 3,
            TSEL_A::PRSCH4 => 4,
            TSEL_A::PRSCH5 => 5,
            TSEL_A::PRSCH6 => 6,
            TSEL_A::PRSCH7 => 7,
            TSEL_A::PRSCH8 => 8,
            TSEL_A::PRSCH9 => 9,
            TSEL_A::PRSCH10 => 10,
            TSEL_A::PRSCH11 => 11,
            TSEL_A::PRSCH12 => 12,
            TSEL_A::PRSCH13 => 13,
            TSEL_A::PRSCH14 => 14,
            TSEL_A::PRSCH15 => 15,
            TSEL_A::PRSCH16 => 16,
            TSEL_A::PRSCH17 => 17,
            TSEL_A::PRSCH18 => 18,
            TSEL_A::PRSCH19 => 19,
            TSEL_A::PRSCH20 => 20,
            TSEL_A::PRSCH21 => 21,
            TSEL_A::PRSCH22 => 22,
            TSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `TSEL`"]
pub type TSEL_R = crate::R<u8, TSEL_A>;
impl TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL_A::PRSCH0),
            1 => Val(TSEL_A::PRSCH1),
            2 => Val(TSEL_A::PRSCH2),
            3 => Val(TSEL_A::PRSCH3),
            4 => Val(TSEL_A::PRSCH4),
            5 => Val(TSEL_A::PRSCH5),
            6 => Val(TSEL_A::PRSCH6),
            7 => Val(TSEL_A::PRSCH7),
            8 => Val(TSEL_A::PRSCH8),
            9 => Val(TSEL_A::PRSCH9),
            10 => Val(TSEL_A::PRSCH10),
            11 => Val(TSEL_A::PRSCH11),
            12 => Val(TSEL_A::PRSCH12),
            13 => Val(TSEL_A::PRSCH13),
            14 => Val(TSEL_A::PRSCH14),
            15 => Val(TSEL_A::PRSCH15),
            16 => Val(TSEL_A::PRSCH16),
            17 => Val(TSEL_A::PRSCH17),
            18 => Val(TSEL_A::PRSCH18),
            19 => Val(TSEL_A::PRSCH19),
            20 => Val(TSEL_A::PRSCH20),
            21 => Val(TSEL_A::PRSCH21),
            22 => Val(TSEL_A::PRSCH22),
            23 => Val(TSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == TSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == TSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == TSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == TSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == TSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == TSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == TSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == TSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == TSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == TSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == TSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == TSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `TSEL`"]
pub struct TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    pub fn txarx0en(&self) -> TXARX0EN_R {
        TXARX0EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    pub fn txarx1en(&self) -> TXARX1EN_R {
        TXARX1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    pub fn txarx2en(&self) -> TXARX2EN_R {
        TXARX2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    pub fn rxatx0en(&self) -> RXATX0EN_R {
        RXATX0EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    pub fn rxatx1en(&self) -> RXATX1EN_R {
        RXATX1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    pub fn rxatx2en(&self) -> RXATX2EN_R {
        RXATX2EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RXTEN_W {
        RXTEN_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TXTEN_W {
        TXTEN_W { w: self }
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W {
        AUTOTXTEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    pub fn txarx0en(&mut self) -> TXARX0EN_W {
        TXARX0EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    pub fn txarx1en(&mut self) -> TXARX1EN_W {
        TXARX1EN_W { w: self }
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    pub fn txarx2en(&mut self) -> TXARX2EN_W {
        TXARX2EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    pub fn rxatx0en(&mut self) -> RXATX0EN_W {
        RXATX0EN_W { w: self }
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    pub fn rxatx1en(&mut self) -> RXATX1EN_W {
        RXATX1EN_W { w: self }
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    pub fn rxatx2en(&mut self) -> RXATX2EN_W {
        RXATX2EN_W { w: self }
    }
    #[doc = "Bits 16:20 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W { w: self }
    }
}
