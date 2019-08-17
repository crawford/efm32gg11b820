#[doc = "Reader of register IRCTRL"]
pub type R = crate::R<u32, super::IRCTRL>;
#[doc = "Writer for register IRCTRL"]
pub type W = crate::W<u32, super::IRCTRL>;
#[doc = "Register IRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::IRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IREN`"]
pub type IREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IREN`"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
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
#[doc = "Possible values of the field `IRPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRPW_A {
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE,
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO,
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE,
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR,
}
impl crate::ToBits<u8> for IRPW_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IRPW_A::ONE => 0,
            IRPW_A::TWO => 1,
            IRPW_A::THREE => 2,
            IRPW_A::FOUR => 3,
        }
    }
}
#[doc = "Reader of field `IRPW`"]
pub type IRPW_R = crate::R<u8, IRPW_A>;
impl IRPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPW_A {
        match self.bits {
            0 => IRPW_A::ONE,
            1 => IRPW_A::TWO,
            2 => IRPW_A::THREE,
            3 => IRPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == IRPW_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == IRPW_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == IRPW_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == IRPW_A::FOUR
    }
}
#[doc = "Write proxy for field `IRPW`"]
pub struct IRPW_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRPW_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(IRPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(IRPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(IRPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(IRPW_A::FOUR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `IRFILT`"]
pub type IRFILT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRFILT`"]
pub struct IRFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRFILT_W<'a> {
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
#[doc = "Reader of field `IRPRSEN`"]
pub type IRPRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRPRSEN`"]
pub struct IRPRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPRSEN_W<'a> {
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
#[doc = "Possible values of the field `IRPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRPRSSEL_A {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = "PRS Channel 12 selected"]
    PRSCH12,
    #[doc = "PRS Channel 13 selected"]
    PRSCH13,
    #[doc = "PRS Channel 14 selected"]
    PRSCH14,
    #[doc = "PRS Channel 15 selected"]
    PRSCH15,
    #[doc = "PRS Channel 16 selected"]
    PRSCH16,
    #[doc = "PRS Channel 17 selected"]
    PRSCH17,
    #[doc = "PRS Channel 18 selected"]
    PRSCH18,
    #[doc = "PRS Channel 19 selected"]
    PRSCH19,
    #[doc = "PRS Channel 20 selected"]
    PRSCH20,
    #[doc = "PRS Channel 21 selected"]
    PRSCH21,
    #[doc = "PRS Channel 22 selected"]
    PRSCH22,
    #[doc = "PRS Channel 23 selected"]
    PRSCH23,
}
impl crate::ToBits<u8> for IRPRSSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IRPRSSEL_A::PRSCH0 => 0,
            IRPRSSEL_A::PRSCH1 => 1,
            IRPRSSEL_A::PRSCH2 => 2,
            IRPRSSEL_A::PRSCH3 => 3,
            IRPRSSEL_A::PRSCH4 => 4,
            IRPRSSEL_A::PRSCH5 => 5,
            IRPRSSEL_A::PRSCH6 => 6,
            IRPRSSEL_A::PRSCH7 => 7,
            IRPRSSEL_A::PRSCH8 => 8,
            IRPRSSEL_A::PRSCH9 => 9,
            IRPRSSEL_A::PRSCH10 => 10,
            IRPRSSEL_A::PRSCH11 => 11,
            IRPRSSEL_A::PRSCH12 => 12,
            IRPRSSEL_A::PRSCH13 => 13,
            IRPRSSEL_A::PRSCH14 => 14,
            IRPRSSEL_A::PRSCH15 => 15,
            IRPRSSEL_A::PRSCH16 => 16,
            IRPRSSEL_A::PRSCH17 => 17,
            IRPRSSEL_A::PRSCH18 => 18,
            IRPRSSEL_A::PRSCH19 => 19,
            IRPRSSEL_A::PRSCH20 => 20,
            IRPRSSEL_A::PRSCH21 => 21,
            IRPRSSEL_A::PRSCH22 => 22,
            IRPRSSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `IRPRSSEL`"]
pub type IRPRSSEL_R = crate::R<u8, IRPRSSEL_A>;
impl IRPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IRPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IRPRSSEL_A::PRSCH0),
            1 => Val(IRPRSSEL_A::PRSCH1),
            2 => Val(IRPRSSEL_A::PRSCH2),
            3 => Val(IRPRSSEL_A::PRSCH3),
            4 => Val(IRPRSSEL_A::PRSCH4),
            5 => Val(IRPRSSEL_A::PRSCH5),
            6 => Val(IRPRSSEL_A::PRSCH6),
            7 => Val(IRPRSSEL_A::PRSCH7),
            8 => Val(IRPRSSEL_A::PRSCH8),
            9 => Val(IRPRSSEL_A::PRSCH9),
            10 => Val(IRPRSSEL_A::PRSCH10),
            11 => Val(IRPRSSEL_A::PRSCH11),
            12 => Val(IRPRSSEL_A::PRSCH12),
            13 => Val(IRPRSSEL_A::PRSCH13),
            14 => Val(IRPRSSEL_A::PRSCH14),
            15 => Val(IRPRSSEL_A::PRSCH15),
            16 => Val(IRPRSSEL_A::PRSCH16),
            17 => Val(IRPRSSEL_A::PRSCH17),
            18 => Val(IRPRSSEL_A::PRSCH18),
            19 => Val(IRPRSSEL_A::PRSCH19),
            20 => Val(IRPRSSEL_A::PRSCH20),
            21 => Val(IRPRSSEL_A::PRSCH21),
            22 => Val(IRPRSSEL_A::PRSCH22),
            23 => Val(IRPRSSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == IRPRSSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `IRPRSSEL`"]
pub struct IRPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRPRSSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IRPW_R {
        IRPW_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IRFILT_R {
        IRFILT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&self) -> IRPRSEN_R {
        IRPRSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&self) -> IRPRSSEL_R {
        IRPRSSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&mut self) -> IRPW_W {
        IRPW_W { w: self }
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&mut self) -> IRFILT_W {
        IRFILT_W { w: self }
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&mut self) -> IRPRSEN_W {
        IRPRSEN_W { w: self }
    }
    #[doc = "Bits 8:12 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&mut self) -> IRPRSSEL_W {
        IRPRSSEL_W { w: self }
    }
}
