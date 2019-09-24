#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `CURSINK`"]
pub type CURSINK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURSINK`"]
pub struct CURSINK_W<'a> {
    w: &'a mut W,
}
impl<'a> CURSINK_W<'a> {
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
#[doc = "Reader of field `MINOUTTRANS`"]
pub type MINOUTTRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINOUTTRANS`"]
pub struct MINOUTTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOUTTRANS_W<'a> {
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
#[doc = "Reader of field `APORTOUTEN`"]
pub type APORTOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTOUTEN`"]
pub struct APORTOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTOUTEN_W<'a> {
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
#[doc = "APORT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APORTOUTSEL_A {
    #[doc = "32: APORT1X Channel 0"]
    APORT1XCH0,
    #[doc = "33: APORT1Y Channel 1"]
    APORT1YCH1,
    #[doc = "34: APORT1X Channel 2"]
    APORT1XCH2,
    #[doc = "35: APORT1Y Channel 3"]
    APORT1YCH3,
    #[doc = "36: APORT1X Channel 4"]
    APORT1XCH4,
    #[doc = "37: APORT1Y Channel 5"]
    APORT1YCH5,
    #[doc = "38: APORT1X Channel 6"]
    APORT1XCH6,
    #[doc = "39: APORT1Y Channel 7"]
    APORT1YCH7,
    #[doc = "40: APORT1X Channel 8"]
    APORT1XCH8,
    #[doc = "41: APORT1Y Channel 9"]
    APORT1YCH9,
    #[doc = "42: APORT1X Channel 10"]
    APORT1XCH10,
    #[doc = "43: APORT1Y Channel 11"]
    APORT1YCH11,
    #[doc = "44: APORT1X Channel 12"]
    APORT1XCH12,
    #[doc = "45: APORT1Y Channel 13"]
    APORT1YCH13,
    #[doc = "46: APORT1X Channel 14"]
    APORT1XCH14,
    #[doc = "47: APORT1Y Channel 15"]
    APORT1YCH15,
    #[doc = "48: APORT1X Channel 16"]
    APORT1XCH16,
    #[doc = "49: APORT1Y Channel 17"]
    APORT1YCH17,
    #[doc = "50: APORT1X Channel 18"]
    APORT1XCH18,
    #[doc = "51: APORT1Y Channel 19"]
    APORT1YCH19,
    #[doc = "52: APORT1X Channel 20"]
    APORT1XCH20,
    #[doc = "53: APORT1Y Channel 21"]
    APORT1YCH21,
    #[doc = "54: APORT1X Channel 22"]
    APORT1XCH22,
    #[doc = "55: APORT1Y Channel 23"]
    APORT1YCH23,
    #[doc = "56: APORT1X Channel 24"]
    APORT1XCH24,
    #[doc = "57: APORT1Y Channel 25"]
    APORT1YCH25,
    #[doc = "58: APORT1X Channel 26"]
    APORT1XCH26,
    #[doc = "59: APORT1Y Channel 27"]
    APORT1YCH27,
    #[doc = "60: APORT1X Channel 28"]
    APORT1XCH28,
    #[doc = "61: APORT1Y Channel 29"]
    APORT1YCH29,
    #[doc = "62: APORT1X Channel 30"]
    APORT1XCH30,
    #[doc = "63: APORT1Y Channel 31"]
    APORT1YCH31,
}
impl From<APORTOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: APORTOUTSEL_A) -> Self {
        match variant {
            APORTOUTSEL_A::APORT1XCH0 => 32,
            APORTOUTSEL_A::APORT1YCH1 => 33,
            APORTOUTSEL_A::APORT1XCH2 => 34,
            APORTOUTSEL_A::APORT1YCH3 => 35,
            APORTOUTSEL_A::APORT1XCH4 => 36,
            APORTOUTSEL_A::APORT1YCH5 => 37,
            APORTOUTSEL_A::APORT1XCH6 => 38,
            APORTOUTSEL_A::APORT1YCH7 => 39,
            APORTOUTSEL_A::APORT1XCH8 => 40,
            APORTOUTSEL_A::APORT1YCH9 => 41,
            APORTOUTSEL_A::APORT1XCH10 => 42,
            APORTOUTSEL_A::APORT1YCH11 => 43,
            APORTOUTSEL_A::APORT1XCH12 => 44,
            APORTOUTSEL_A::APORT1YCH13 => 45,
            APORTOUTSEL_A::APORT1XCH14 => 46,
            APORTOUTSEL_A::APORT1YCH15 => 47,
            APORTOUTSEL_A::APORT1XCH16 => 48,
            APORTOUTSEL_A::APORT1YCH17 => 49,
            APORTOUTSEL_A::APORT1XCH18 => 50,
            APORTOUTSEL_A::APORT1YCH19 => 51,
            APORTOUTSEL_A::APORT1XCH20 => 52,
            APORTOUTSEL_A::APORT1YCH21 => 53,
            APORTOUTSEL_A::APORT1XCH22 => 54,
            APORTOUTSEL_A::APORT1YCH23 => 55,
            APORTOUTSEL_A::APORT1XCH24 => 56,
            APORTOUTSEL_A::APORT1YCH25 => 57,
            APORTOUTSEL_A::APORT1XCH26 => 58,
            APORTOUTSEL_A::APORT1YCH27 => 59,
            APORTOUTSEL_A::APORT1XCH28 => 60,
            APORTOUTSEL_A::APORT1YCH29 => 61,
            APORTOUTSEL_A::APORT1XCH30 => 62,
            APORTOUTSEL_A::APORT1YCH31 => 63,
        }
    }
}
#[doc = "Reader of field `APORTOUTSEL`"]
pub type APORTOUTSEL_R = crate::R<u8, APORTOUTSEL_A>;
impl APORTOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APORTOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            32 => Val(APORTOUTSEL_A::APORT1XCH0),
            33 => Val(APORTOUTSEL_A::APORT1YCH1),
            34 => Val(APORTOUTSEL_A::APORT1XCH2),
            35 => Val(APORTOUTSEL_A::APORT1YCH3),
            36 => Val(APORTOUTSEL_A::APORT1XCH4),
            37 => Val(APORTOUTSEL_A::APORT1YCH5),
            38 => Val(APORTOUTSEL_A::APORT1XCH6),
            39 => Val(APORTOUTSEL_A::APORT1YCH7),
            40 => Val(APORTOUTSEL_A::APORT1XCH8),
            41 => Val(APORTOUTSEL_A::APORT1YCH9),
            42 => Val(APORTOUTSEL_A::APORT1XCH10),
            43 => Val(APORTOUTSEL_A::APORT1YCH11),
            44 => Val(APORTOUTSEL_A::APORT1XCH12),
            45 => Val(APORTOUTSEL_A::APORT1YCH13),
            46 => Val(APORTOUTSEL_A::APORT1XCH14),
            47 => Val(APORTOUTSEL_A::APORT1YCH15),
            48 => Val(APORTOUTSEL_A::APORT1XCH16),
            49 => Val(APORTOUTSEL_A::APORT1YCH17),
            50 => Val(APORTOUTSEL_A::APORT1XCH18),
            51 => Val(APORTOUTSEL_A::APORT1YCH19),
            52 => Val(APORTOUTSEL_A::APORT1XCH20),
            53 => Val(APORTOUTSEL_A::APORT1YCH21),
            54 => Val(APORTOUTSEL_A::APORT1XCH22),
            55 => Val(APORTOUTSEL_A::APORT1YCH23),
            56 => Val(APORTOUTSEL_A::APORT1XCH24),
            57 => Val(APORTOUTSEL_A::APORT1YCH25),
            58 => Val(APORTOUTSEL_A::APORT1XCH26),
            59 => Val(APORTOUTSEL_A::APORT1YCH27),
            60 => Val(APORTOUTSEL_A::APORT1XCH28),
            61 => Val(APORTOUTSEL_A::APORT1YCH29),
            62 => Val(APORTOUTSEL_A::APORT1XCH30),
            63 => Val(APORTOUTSEL_A::APORT1YCH31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH31
    }
}
#[doc = "Write proxy for field `APORTOUTSEL`"]
pub struct APORTOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APORTOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `PWRSEL`"]
pub type PWRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRSEL`"]
pub struct PWRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSEL_W<'a> {
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
#[doc = "Reader of field `EM2DELAY`"]
pub type EM2DELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2DELAY`"]
pub struct EM2DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2DELAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `APORTMASTERDIS`"]
pub type APORTMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTMASTERDIS`"]
pub struct APORTMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `APORTOUTENPRS`"]
pub type APORTOUTENPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTOUTENPRS`"]
pub struct APORTOUTENPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTOUTENPRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAINOUTEN`"]
pub type MAINOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAINOUTEN`"]
pub struct MAINOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINOUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MAINOUTENPRS`"]
pub type MAINOUTENPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAINOUTENPRS`"]
pub struct MAINOUTENPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINOUTENPRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "IDAC Output Enable PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11,
    #[doc = "12: PRS Channel 12 selected."]
    PRSCH12,
    #[doc = "13: PRS Channel 13 selected."]
    PRSCH13,
    #[doc = "14: PRS Channel 14 selected."]
    PRSCH14,
    #[doc = "15: PRS Channel 15 selected."]
    PRSCH15,
    #[doc = "16: PRS Channel 16 selected."]
    PRSCH16,
    #[doc = "17: PRS Channel 17 selected."]
    PRSCH17,
    #[doc = "18: PRS Channel 18 selected."]
    PRSCH18,
    #[doc = "19: PRS Channel 19 selected."]
    PRSCH19,
    #[doc = "20: PRS Channel 20 selected."]
    PRSCH20,
    #[doc = "21: PRS Channel 21 selected."]
    PRSCH21,
    #[doc = "22: PRS Channel 22 selected."]
    PRSCH22,
    #[doc = "23: PRS Channel 23 selected."]
    PRSCH23,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        match variant {
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CURSINK_R {
        CURSINK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MINOUTTRANS_R {
        MINOUTTRANS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> APORTOUTEN_R {
        APORTOUTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> APORTOUTSEL_R {
        APORTOUTSEL_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    pub fn em2delay(&self) -> EM2DELAY_R {
        EM2DELAY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportmasterdis(&self) -> APORTMASTERDIS_R {
        APORTMASTERDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    pub fn aportoutenprs(&self) -> APORTOUTENPRS_R {
        APORTOUTENPRS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Enable"]
    #[inline(always)]
    pub fn mainouten(&self) -> MAINOUTEN_R {
        MAINOUTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PRS Controlled Main Pad Output Enable"]
    #[inline(always)]
    pub fn mainoutenprs(&self) -> MAINOUTENPRS_R {
        MAINOUTENPRS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:24 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&mut self) -> CURSINK_W {
        CURSINK_W { w: self }
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&mut self) -> MINOUTTRANS_W {
        MINOUTTRANS_W { w: self }
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    pub fn aportouten(&mut self) -> APORTOUTEN_W {
        APORTOUTEN_W { w: self }
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    pub fn aportoutsel(&mut self) -> APORTOUTSEL_W {
        APORTOUTSEL_W { w: self }
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PWRSEL_W {
        PWRSEL_W { w: self }
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    pub fn em2delay(&mut self) -> EM2DELAY_W {
        EM2DELAY_W { w: self }
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportmasterdis(&mut self) -> APORTMASTERDIS_W {
        APORTMASTERDIS_W { w: self }
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    pub fn aportoutenprs(&mut self) -> APORTOUTENPRS_W {
        APORTOUTENPRS_W { w: self }
    }
    #[doc = "Bit 18 - Output Enable"]
    #[inline(always)]
    pub fn mainouten(&mut self) -> MAINOUTEN_W {
        MAINOUTEN_W { w: self }
    }
    #[doc = "Bit 19 - PRS Controlled Main Pad Output Enable"]
    #[inline(always)]
    pub fn mainoutenprs(&mut self) -> MAINOUTENPRS_W {
        MAINOUTENPRS_W { w: self }
    }
    #[doc = "Bits 20:24 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
}
