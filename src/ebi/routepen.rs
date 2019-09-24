#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EBIPEN`"]
pub type EBIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIPEN`"]
pub struct EBIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIPEN_W<'a> {
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
#[doc = "Reader of field `CS0PEN`"]
pub type CS0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS0PEN`"]
pub struct CS0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0PEN_W<'a> {
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
#[doc = "Reader of field `CS1PEN`"]
pub type CS1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1PEN`"]
pub struct CS1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1PEN_W<'a> {
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
#[doc = "Reader of field `CS2PEN`"]
pub type CS2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS2PEN`"]
pub struct CS2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2PEN_W<'a> {
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
#[doc = "Reader of field `CS3PEN`"]
pub type CS3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS3PEN`"]
pub struct CS3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3PEN_W<'a> {
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
#[doc = "Reader of field `ALEPEN`"]
pub type ALEPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALEPEN`"]
pub struct ALEPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEPEN_W<'a> {
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
#[doc = "Reader of field `ARDYPEN`"]
pub type ARDYPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYPEN`"]
pub struct ARDYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYPEN_W<'a> {
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
#[doc = "Reader of field `BLPEN`"]
pub type BLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLPEN`"]
pub struct BLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLPEN_W<'a> {
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
#[doc = "Reader of field `NANDPEN`"]
pub type NANDPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NANDPEN`"]
pub struct NANDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDPEN_W<'a> {
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
#[doc = "Sets the Lower Bound for EBI_A Enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALB_A {
    #[doc = "0: Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    A0,
    #[doc = "1: Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    A8,
    #[doc = "2: Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    A16,
    #[doc = "3: Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    A24,
}
impl From<ALB_A> for u8 {
    #[inline(always)]
    fn from(variant: ALB_A) -> Self {
        match variant {
            ALB_A::A0 => 0,
            ALB_A::A8 => 1,
            ALB_A::A16 => 2,
            ALB_A::A24 => 3,
        }
    }
}
#[doc = "Reader of field `ALB`"]
pub type ALB_R = crate::R<u8, ALB_A>;
impl ALB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALB_A {
        match self.bits {
            0 => ALB_A::A0,
            1 => ALB_A::A8,
            2 => ALB_A::A16,
            3 => ALB_A::A24,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == ALB_A::A0
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == ALB_A::A8
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == ALB_A::A16
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == ALB_A::A24
    }
}
#[doc = "Write proxy for field `ALB`"]
pub struct ALB_W<'a> {
    w: &'a mut W,
}
impl<'a> ALB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(ALB_A::A0)
    }
    #[doc = "Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(ALB_A::A8)
    }
    #[doc = "Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut W {
        self.variant(ALB_A::A16)
    }
    #[doc = "Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut W {
        self.variant(ALB_A::A24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "EBI_A Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APEN_A {
    #[doc = "0: All EBI_A pins are disabled."]
    A0,
    #[doc = "5: EBI_A\\[4:L\\] pins enabled."]
    A5,
    #[doc = "6: EBI_A\\[5:L\\] pins enabled."]
    A6,
    #[doc = "7: EBI_A\\[6:L\\] pins enabled."]
    A7,
    #[doc = "8: EBI_A\\[7:L\\] pins enabled."]
    A8,
    #[doc = "9: EBI_A\\[8:L\\] pins enabled."]
    A9,
    #[doc = "10: EBI_A\\[9:L\\] pins enabled."]
    A10,
    #[doc = "11: EBI_A\\[10:L\\] pins enabled."]
    A11,
    #[doc = "12: EBI_A\\[11:L\\] pins enabled."]
    A12,
    #[doc = "13: EBI_A\\[12:L\\] pins enabled."]
    A13,
    #[doc = "14: EBI_A\\[13:L\\] pins enabled."]
    A14,
    #[doc = "15: EBI_A\\[14:L\\] pins enabled."]
    A15,
    #[doc = "16: EBI_A\\[15:L\\] pins enabled."]
    A16,
    #[doc = "17: EBI_A\\[16:L\\] pins enabled."]
    A17,
    #[doc = "18: EBI_A\\[17:L\\] pins enabled."]
    A18,
    #[doc = "19: EBI_A\\[18:L\\] pins enabled."]
    A19,
    #[doc = "20: EBI_A\\[19:L\\] pins enabled."]
    A20,
    #[doc = "21: EBI_A\\[20:L\\] pins enabled."]
    A21,
    #[doc = "22: EBI_A\\[21:L\\] pins enabled."]
    A22,
    #[doc = "23: EBI_A\\[22:L\\] pins enabled."]
    A23,
    #[doc = "24: EBI_A\\[23:L\\] pins enabled."]
    A24,
    #[doc = "25: EBI_A\\[24:L\\] pins enabled."]
    A25,
    #[doc = "26: EBI_A\\[25:L\\] pins enabled."]
    A26,
    #[doc = "27: EBI_A\\[26:L\\] pins enabled."]
    A27,
    #[doc = "28: EBI_A\\[27:L\\] pins enabled."]
    A28,
}
impl From<APEN_A> for u8 {
    #[inline(always)]
    fn from(variant: APEN_A) -> Self {
        match variant {
            APEN_A::A0 => 0,
            APEN_A::A5 => 5,
            APEN_A::A6 => 6,
            APEN_A::A7 => 7,
            APEN_A::A8 => 8,
            APEN_A::A9 => 9,
            APEN_A::A10 => 10,
            APEN_A::A11 => 11,
            APEN_A::A12 => 12,
            APEN_A::A13 => 13,
            APEN_A::A14 => 14,
            APEN_A::A15 => 15,
            APEN_A::A16 => 16,
            APEN_A::A17 => 17,
            APEN_A::A18 => 18,
            APEN_A::A19 => 19,
            APEN_A::A20 => 20,
            APEN_A::A21 => 21,
            APEN_A::A22 => 22,
            APEN_A::A23 => 23,
            APEN_A::A24 => 24,
            APEN_A::A25 => 25,
            APEN_A::A26 => 26,
            APEN_A::A27 => 27,
            APEN_A::A28 => 28,
        }
    }
}
#[doc = "Reader of field `APEN`"]
pub type APEN_R = crate::R<u8, APEN_A>;
impl APEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(APEN_A::A0),
            5 => Val(APEN_A::A5),
            6 => Val(APEN_A::A6),
            7 => Val(APEN_A::A7),
            8 => Val(APEN_A::A8),
            9 => Val(APEN_A::A9),
            10 => Val(APEN_A::A10),
            11 => Val(APEN_A::A11),
            12 => Val(APEN_A::A12),
            13 => Val(APEN_A::A13),
            14 => Val(APEN_A::A14),
            15 => Val(APEN_A::A15),
            16 => Val(APEN_A::A16),
            17 => Val(APEN_A::A17),
            18 => Val(APEN_A::A18),
            19 => Val(APEN_A::A19),
            20 => Val(APEN_A::A20),
            21 => Val(APEN_A::A21),
            22 => Val(APEN_A::A22),
            23 => Val(APEN_A::A23),
            24 => Val(APEN_A::A24),
            25 => Val(APEN_A::A25),
            26 => Val(APEN_A::A26),
            27 => Val(APEN_A::A27),
            28 => Val(APEN_A::A28),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == APEN_A::A0
    }
    #[doc = "Checks if the value of the field is `A5`"]
    #[inline(always)]
    pub fn is_a5(&self) -> bool {
        *self == APEN_A::A5
    }
    #[doc = "Checks if the value of the field is `A6`"]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == APEN_A::A6
    }
    #[doc = "Checks if the value of the field is `A7`"]
    #[inline(always)]
    pub fn is_a7(&self) -> bool {
        *self == APEN_A::A7
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == APEN_A::A8
    }
    #[doc = "Checks if the value of the field is `A9`"]
    #[inline(always)]
    pub fn is_a9(&self) -> bool {
        *self == APEN_A::A9
    }
    #[doc = "Checks if the value of the field is `A10`"]
    #[inline(always)]
    pub fn is_a10(&self) -> bool {
        *self == APEN_A::A10
    }
    #[doc = "Checks if the value of the field is `A11`"]
    #[inline(always)]
    pub fn is_a11(&self) -> bool {
        *self == APEN_A::A11
    }
    #[doc = "Checks if the value of the field is `A12`"]
    #[inline(always)]
    pub fn is_a12(&self) -> bool {
        *self == APEN_A::A12
    }
    #[doc = "Checks if the value of the field is `A13`"]
    #[inline(always)]
    pub fn is_a13(&self) -> bool {
        *self == APEN_A::A13
    }
    #[doc = "Checks if the value of the field is `A14`"]
    #[inline(always)]
    pub fn is_a14(&self) -> bool {
        *self == APEN_A::A14
    }
    #[doc = "Checks if the value of the field is `A15`"]
    #[inline(always)]
    pub fn is_a15(&self) -> bool {
        *self == APEN_A::A15
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == APEN_A::A16
    }
    #[doc = "Checks if the value of the field is `A17`"]
    #[inline(always)]
    pub fn is_a17(&self) -> bool {
        *self == APEN_A::A17
    }
    #[doc = "Checks if the value of the field is `A18`"]
    #[inline(always)]
    pub fn is_a18(&self) -> bool {
        *self == APEN_A::A18
    }
    #[doc = "Checks if the value of the field is `A19`"]
    #[inline(always)]
    pub fn is_a19(&self) -> bool {
        *self == APEN_A::A19
    }
    #[doc = "Checks if the value of the field is `A20`"]
    #[inline(always)]
    pub fn is_a20(&self) -> bool {
        *self == APEN_A::A20
    }
    #[doc = "Checks if the value of the field is `A21`"]
    #[inline(always)]
    pub fn is_a21(&self) -> bool {
        *self == APEN_A::A21
    }
    #[doc = "Checks if the value of the field is `A22`"]
    #[inline(always)]
    pub fn is_a22(&self) -> bool {
        *self == APEN_A::A22
    }
    #[doc = "Checks if the value of the field is `A23`"]
    #[inline(always)]
    pub fn is_a23(&self) -> bool {
        *self == APEN_A::A23
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == APEN_A::A24
    }
    #[doc = "Checks if the value of the field is `A25`"]
    #[inline(always)]
    pub fn is_a25(&self) -> bool {
        *self == APEN_A::A25
    }
    #[doc = "Checks if the value of the field is `A26`"]
    #[inline(always)]
    pub fn is_a26(&self) -> bool {
        *self == APEN_A::A26
    }
    #[doc = "Checks if the value of the field is `A27`"]
    #[inline(always)]
    pub fn is_a27(&self) -> bool {
        *self == APEN_A::A27
    }
    #[doc = "Checks if the value of the field is `A28`"]
    #[inline(always)]
    pub fn is_a28(&self) -> bool {
        *self == APEN_A::A28
    }
}
#[doc = "Write proxy for field `APEN`"]
pub struct APEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut W {
        self.variant(APEN_A::A0)
    }
    #[doc = "EBI_A\\[4:L\\] pins enabled."]
    #[inline(always)]
    pub fn a5(self) -> &'a mut W {
        self.variant(APEN_A::A5)
    }
    #[doc = "EBI_A\\[5:L\\] pins enabled."]
    #[inline(always)]
    pub fn a6(self) -> &'a mut W {
        self.variant(APEN_A::A6)
    }
    #[doc = "EBI_A\\[6:L\\] pins enabled."]
    #[inline(always)]
    pub fn a7(self) -> &'a mut W {
        self.variant(APEN_A::A7)
    }
    #[doc = "EBI_A\\[7:L\\] pins enabled."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(APEN_A::A8)
    }
    #[doc = "EBI_A\\[8:L\\] pins enabled."]
    #[inline(always)]
    pub fn a9(self) -> &'a mut W {
        self.variant(APEN_A::A9)
    }
    #[doc = "EBI_A\\[9:L\\] pins enabled."]
    #[inline(always)]
    pub fn a10(self) -> &'a mut W {
        self.variant(APEN_A::A10)
    }
    #[doc = "EBI_A\\[10:L\\] pins enabled."]
    #[inline(always)]
    pub fn a11(self) -> &'a mut W {
        self.variant(APEN_A::A11)
    }
    #[doc = "EBI_A\\[11:L\\] pins enabled."]
    #[inline(always)]
    pub fn a12(self) -> &'a mut W {
        self.variant(APEN_A::A12)
    }
    #[doc = "EBI_A\\[12:L\\] pins enabled."]
    #[inline(always)]
    pub fn a13(self) -> &'a mut W {
        self.variant(APEN_A::A13)
    }
    #[doc = "EBI_A\\[13:L\\] pins enabled."]
    #[inline(always)]
    pub fn a14(self) -> &'a mut W {
        self.variant(APEN_A::A14)
    }
    #[doc = "EBI_A\\[14:L\\] pins enabled."]
    #[inline(always)]
    pub fn a15(self) -> &'a mut W {
        self.variant(APEN_A::A15)
    }
    #[doc = "EBI_A\\[15:L\\] pins enabled."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut W {
        self.variant(APEN_A::A16)
    }
    #[doc = "EBI_A\\[16:L\\] pins enabled."]
    #[inline(always)]
    pub fn a17(self) -> &'a mut W {
        self.variant(APEN_A::A17)
    }
    #[doc = "EBI_A\\[17:L\\] pins enabled."]
    #[inline(always)]
    pub fn a18(self) -> &'a mut W {
        self.variant(APEN_A::A18)
    }
    #[doc = "EBI_A\\[18:L\\] pins enabled."]
    #[inline(always)]
    pub fn a19(self) -> &'a mut W {
        self.variant(APEN_A::A19)
    }
    #[doc = "EBI_A\\[19:L\\] pins enabled."]
    #[inline(always)]
    pub fn a20(self) -> &'a mut W {
        self.variant(APEN_A::A20)
    }
    #[doc = "EBI_A\\[20:L\\] pins enabled."]
    #[inline(always)]
    pub fn a21(self) -> &'a mut W {
        self.variant(APEN_A::A21)
    }
    #[doc = "EBI_A\\[21:L\\] pins enabled."]
    #[inline(always)]
    pub fn a22(self) -> &'a mut W {
        self.variant(APEN_A::A22)
    }
    #[doc = "EBI_A\\[22:L\\] pins enabled."]
    #[inline(always)]
    pub fn a23(self) -> &'a mut W {
        self.variant(APEN_A::A23)
    }
    #[doc = "EBI_A\\[23:L\\] pins enabled."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut W {
        self.variant(APEN_A::A24)
    }
    #[doc = "EBI_A\\[24:L\\] pins enabled."]
    #[inline(always)]
    pub fn a25(self) -> &'a mut W {
        self.variant(APEN_A::A25)
    }
    #[doc = "EBI_A\\[25:L\\] pins enabled."]
    #[inline(always)]
    pub fn a26(self) -> &'a mut W {
        self.variant(APEN_A::A26)
    }
    #[doc = "EBI_A\\[26:L\\] pins enabled."]
    #[inline(always)]
    pub fn a27(self) -> &'a mut W {
        self.variant(APEN_A::A27)
    }
    #[doc = "EBI_A\\[27:L\\] pins enabled."]
    #[inline(always)]
    pub fn a28(self) -> &'a mut W {
        self.variant(APEN_A::A28)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `TFTPEN`"]
pub type TFTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTPEN`"]
pub struct TFTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTPEN_W<'a> {
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
#[doc = "Reader of field `DATAENPEN`"]
pub type DATAENPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAENPEN`"]
pub struct DATAENPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENPEN_W<'a> {
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
#[doc = "Reader of field `CSTFTPEN`"]
pub type CSTFTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSTFTPEN`"]
pub struct CSTFTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTFTPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&self) -> EBIPEN_R {
        EBIPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&self) -> CS2PEN_R {
        CS2PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&self) -> CS3PEN_R {
        CS3PEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&self) -> ALEPEN_R {
        ALEPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&self) -> ARDYPEN_R {
        ARDYPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\] Pin Enable"]
    #[inline(always)]
    pub fn blpen(&self) -> BLPEN_R {
        BLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&self) -> NANDPEN_R {
        NANDPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    pub fn alb(&self) -> ALB_R {
        ALB_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&self) -> APEN_R {
        APEN_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&self) -> TFTPEN_R {
        TFTPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&self) -> DATAENPEN_R {
        DATAENPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&self) -> CSTFTPEN_R {
        CSTFTPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&mut self) -> EBIPEN_W {
        EBIPEN_W { w: self }
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&mut self) -> CS0PEN_W {
        CS0PEN_W { w: self }
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&mut self) -> CS1PEN_W {
        CS1PEN_W { w: self }
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&mut self) -> CS2PEN_W {
        CS2PEN_W { w: self }
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&mut self) -> CS3PEN_W {
        CS3PEN_W { w: self }
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&mut self) -> ALEPEN_W {
        ALEPEN_W { w: self }
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&mut self) -> ARDYPEN_W {
        ARDYPEN_W { w: self }
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\] Pin Enable"]
    #[inline(always)]
    pub fn blpen(&mut self) -> BLPEN_W {
        BLPEN_W { w: self }
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&mut self) -> NANDPEN_W {
        NANDPEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    pub fn alb(&mut self) -> ALB_W {
        ALB_W { w: self }
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&mut self) -> APEN_W {
        APEN_W { w: self }
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&mut self) -> TFTPEN_W {
        TFTPEN_W { w: self }
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&mut self) -> DATAENPEN_W {
        DATAENPEN_W { w: self }
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&mut self) -> CSTFTPEN_W {
        CSTFTPEN_W { w: self }
    }
}
