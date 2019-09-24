#[doc = "Reader of register INPUTSEL"]
pub type R = crate::R<u32, super::INPUTSEL>;
#[doc = "Writer for register INPUTSEL"]
pub type W = crate::W<u32, super::INPUTSEL>;
#[doc = "Register INPUTSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POSSEL`"]
pub type POSSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSSEL`"]
pub struct POSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POSSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NEGSEL`"]
pub type NEGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEGSEL`"]
pub struct NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "VA Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VASEL_A {
    #[doc = "0: ACMPVDD"]
    VDD,
    #[doc = "1: APORT2Y Channel 0"]
    APORT2YCH0,
    #[doc = "3: APORT2Y Channel 2"]
    APORT2YCH2,
    #[doc = "5: APORT2Y Channel 4"]
    APORT2YCH4,
    #[doc = "7: APORT2Y Channel 6"]
    APORT2YCH6,
    #[doc = "9: APORT2Y Channel 8"]
    APORT2YCH8,
    #[doc = "11: APORT2Y Channel 10"]
    APORT2YCH10,
    #[doc = "13: APORT2Y Channel 12"]
    APORT2YCH12,
    #[doc = "15: APORT2Y Channel 14"]
    APORT2YCH14,
    #[doc = "17: APORT2Y Channel 16"]
    APORT2YCH16,
    #[doc = "19: APORT2Y Channel 18"]
    APORT2YCH18,
    #[doc = "21: APORT2Y Channel 20"]
    APORT2YCH20,
    #[doc = "23: APORT2Y Channel 22"]
    APORT2YCH22,
    #[doc = "25: APORT2Y Channel 24"]
    APORT2YCH24,
    #[doc = "27: APORT2Y Channel 26"]
    APORT2YCH26,
    #[doc = "29: APORT2Y Channel 28"]
    APORT2YCH28,
    #[doc = "31: APORT2Y Channel 30"]
    APORT2YCH30,
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
impl From<VASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VASEL_A) -> Self {
        match variant {
            VASEL_A::VDD => 0,
            VASEL_A::APORT2YCH0 => 1,
            VASEL_A::APORT2YCH2 => 3,
            VASEL_A::APORT2YCH4 => 5,
            VASEL_A::APORT2YCH6 => 7,
            VASEL_A::APORT2YCH8 => 9,
            VASEL_A::APORT2YCH10 => 11,
            VASEL_A::APORT2YCH12 => 13,
            VASEL_A::APORT2YCH14 => 15,
            VASEL_A::APORT2YCH16 => 17,
            VASEL_A::APORT2YCH18 => 19,
            VASEL_A::APORT2YCH20 => 21,
            VASEL_A::APORT2YCH22 => 23,
            VASEL_A::APORT2YCH24 => 25,
            VASEL_A::APORT2YCH26 => 27,
            VASEL_A::APORT2YCH28 => 29,
            VASEL_A::APORT2YCH30 => 31,
            VASEL_A::APORT1XCH0 => 32,
            VASEL_A::APORT1YCH1 => 33,
            VASEL_A::APORT1XCH2 => 34,
            VASEL_A::APORT1YCH3 => 35,
            VASEL_A::APORT1XCH4 => 36,
            VASEL_A::APORT1YCH5 => 37,
            VASEL_A::APORT1XCH6 => 38,
            VASEL_A::APORT1YCH7 => 39,
            VASEL_A::APORT1XCH8 => 40,
            VASEL_A::APORT1YCH9 => 41,
            VASEL_A::APORT1XCH10 => 42,
            VASEL_A::APORT1YCH11 => 43,
            VASEL_A::APORT1XCH12 => 44,
            VASEL_A::APORT1YCH13 => 45,
            VASEL_A::APORT1XCH14 => 46,
            VASEL_A::APORT1YCH15 => 47,
            VASEL_A::APORT1XCH16 => 48,
            VASEL_A::APORT1YCH17 => 49,
            VASEL_A::APORT1XCH18 => 50,
            VASEL_A::APORT1YCH19 => 51,
            VASEL_A::APORT1XCH20 => 52,
            VASEL_A::APORT1YCH21 => 53,
            VASEL_A::APORT1XCH22 => 54,
            VASEL_A::APORT1YCH23 => 55,
            VASEL_A::APORT1XCH24 => 56,
            VASEL_A::APORT1YCH25 => 57,
            VASEL_A::APORT1XCH26 => 58,
            VASEL_A::APORT1YCH27 => 59,
            VASEL_A::APORT1XCH28 => 60,
            VASEL_A::APORT1YCH29 => 61,
            VASEL_A::APORT1XCH30 => 62,
            VASEL_A::APORT1YCH31 => 63,
        }
    }
}
#[doc = "Reader of field `VASEL`"]
pub type VASEL_R = crate::R<u8, VASEL_A>;
impl VASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VASEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VASEL_A::VDD),
            1 => Val(VASEL_A::APORT2YCH0),
            3 => Val(VASEL_A::APORT2YCH2),
            5 => Val(VASEL_A::APORT2YCH4),
            7 => Val(VASEL_A::APORT2YCH6),
            9 => Val(VASEL_A::APORT2YCH8),
            11 => Val(VASEL_A::APORT2YCH10),
            13 => Val(VASEL_A::APORT2YCH12),
            15 => Val(VASEL_A::APORT2YCH14),
            17 => Val(VASEL_A::APORT2YCH16),
            19 => Val(VASEL_A::APORT2YCH18),
            21 => Val(VASEL_A::APORT2YCH20),
            23 => Val(VASEL_A::APORT2YCH22),
            25 => Val(VASEL_A::APORT2YCH24),
            27 => Val(VASEL_A::APORT2YCH26),
            29 => Val(VASEL_A::APORT2YCH28),
            31 => Val(VASEL_A::APORT2YCH30),
            32 => Val(VASEL_A::APORT1XCH0),
            33 => Val(VASEL_A::APORT1YCH1),
            34 => Val(VASEL_A::APORT1XCH2),
            35 => Val(VASEL_A::APORT1YCH3),
            36 => Val(VASEL_A::APORT1XCH4),
            37 => Val(VASEL_A::APORT1YCH5),
            38 => Val(VASEL_A::APORT1XCH6),
            39 => Val(VASEL_A::APORT1YCH7),
            40 => Val(VASEL_A::APORT1XCH8),
            41 => Val(VASEL_A::APORT1YCH9),
            42 => Val(VASEL_A::APORT1XCH10),
            43 => Val(VASEL_A::APORT1YCH11),
            44 => Val(VASEL_A::APORT1XCH12),
            45 => Val(VASEL_A::APORT1YCH13),
            46 => Val(VASEL_A::APORT1XCH14),
            47 => Val(VASEL_A::APORT1YCH15),
            48 => Val(VASEL_A::APORT1XCH16),
            49 => Val(VASEL_A::APORT1YCH17),
            50 => Val(VASEL_A::APORT1XCH18),
            51 => Val(VASEL_A::APORT1YCH19),
            52 => Val(VASEL_A::APORT1XCH20),
            53 => Val(VASEL_A::APORT1YCH21),
            54 => Val(VASEL_A::APORT1XCH22),
            55 => Val(VASEL_A::APORT1YCH23),
            56 => Val(VASEL_A::APORT1XCH24),
            57 => Val(VASEL_A::APORT1YCH25),
            58 => Val(VASEL_A::APORT1XCH26),
            59 => Val(VASEL_A::APORT1YCH27),
            60 => Val(VASEL_A::APORT1XCH28),
            61 => Val(VASEL_A::APORT1YCH29),
            62 => Val(VASEL_A::APORT1XCH30),
            63 => Val(VASEL_A::APORT1YCH31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == VASEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `APORT2YCH0`"]
    #[inline(always)]
    pub fn is_aport2ych0(&self) -> bool {
        *self == VASEL_A::APORT2YCH0
    }
    #[doc = "Checks if the value of the field is `APORT2YCH2`"]
    #[inline(always)]
    pub fn is_aport2ych2(&self) -> bool {
        *self == VASEL_A::APORT2YCH2
    }
    #[doc = "Checks if the value of the field is `APORT2YCH4`"]
    #[inline(always)]
    pub fn is_aport2ych4(&self) -> bool {
        *self == VASEL_A::APORT2YCH4
    }
    #[doc = "Checks if the value of the field is `APORT2YCH6`"]
    #[inline(always)]
    pub fn is_aport2ych6(&self) -> bool {
        *self == VASEL_A::APORT2YCH6
    }
    #[doc = "Checks if the value of the field is `APORT2YCH8`"]
    #[inline(always)]
    pub fn is_aport2ych8(&self) -> bool {
        *self == VASEL_A::APORT2YCH8
    }
    #[doc = "Checks if the value of the field is `APORT2YCH10`"]
    #[inline(always)]
    pub fn is_aport2ych10(&self) -> bool {
        *self == VASEL_A::APORT2YCH10
    }
    #[doc = "Checks if the value of the field is `APORT2YCH12`"]
    #[inline(always)]
    pub fn is_aport2ych12(&self) -> bool {
        *self == VASEL_A::APORT2YCH12
    }
    #[doc = "Checks if the value of the field is `APORT2YCH14`"]
    #[inline(always)]
    pub fn is_aport2ych14(&self) -> bool {
        *self == VASEL_A::APORT2YCH14
    }
    #[doc = "Checks if the value of the field is `APORT2YCH16`"]
    #[inline(always)]
    pub fn is_aport2ych16(&self) -> bool {
        *self == VASEL_A::APORT2YCH16
    }
    #[doc = "Checks if the value of the field is `APORT2YCH18`"]
    #[inline(always)]
    pub fn is_aport2ych18(&self) -> bool {
        *self == VASEL_A::APORT2YCH18
    }
    #[doc = "Checks if the value of the field is `APORT2YCH20`"]
    #[inline(always)]
    pub fn is_aport2ych20(&self) -> bool {
        *self == VASEL_A::APORT2YCH20
    }
    #[doc = "Checks if the value of the field is `APORT2YCH22`"]
    #[inline(always)]
    pub fn is_aport2ych22(&self) -> bool {
        *self == VASEL_A::APORT2YCH22
    }
    #[doc = "Checks if the value of the field is `APORT2YCH24`"]
    #[inline(always)]
    pub fn is_aport2ych24(&self) -> bool {
        *self == VASEL_A::APORT2YCH24
    }
    #[doc = "Checks if the value of the field is `APORT2YCH26`"]
    #[inline(always)]
    pub fn is_aport2ych26(&self) -> bool {
        *self == VASEL_A::APORT2YCH26
    }
    #[doc = "Checks if the value of the field is `APORT2YCH28`"]
    #[inline(always)]
    pub fn is_aport2ych28(&self) -> bool {
        *self == VASEL_A::APORT2YCH28
    }
    #[doc = "Checks if the value of the field is `APORT2YCH30`"]
    #[inline(always)]
    pub fn is_aport2ych30(&self) -> bool {
        *self == VASEL_A::APORT2YCH30
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == VASEL_A::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == VASEL_A::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == VASEL_A::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == VASEL_A::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == VASEL_A::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == VASEL_A::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == VASEL_A::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == VASEL_A::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == VASEL_A::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == VASEL_A::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == VASEL_A::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == VASEL_A::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == VASEL_A::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == VASEL_A::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == VASEL_A::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == VASEL_A::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == VASEL_A::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == VASEL_A::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == VASEL_A::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == VASEL_A::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == VASEL_A::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == VASEL_A::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == VASEL_A::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == VASEL_A::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == VASEL_A::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == VASEL_A::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == VASEL_A::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == VASEL_A::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == VASEL_A::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == VASEL_A::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == VASEL_A::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == VASEL_A::APORT1YCH31
    }
}
#[doc = "Write proxy for field `VASEL`"]
pub struct VASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ACMPVDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(VASEL_A::VDD)
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline(always)]
    pub fn aport2ych0(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH0)
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline(always)]
    pub fn aport2ych2(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH2)
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline(always)]
    pub fn aport2ych4(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH4)
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline(always)]
    pub fn aport2ych6(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH6)
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline(always)]
    pub fn aport2ych8(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH8)
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline(always)]
    pub fn aport2ych10(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH10)
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline(always)]
    pub fn aport2ych12(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH12)
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline(always)]
    pub fn aport2ych14(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH14)
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline(always)]
    pub fn aport2ych16(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH16)
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline(always)]
    pub fn aport2ych18(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH18)
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline(always)]
    pub fn aport2ych20(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH20)
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline(always)]
    pub fn aport2ych22(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH22)
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline(always)]
    pub fn aport2ych24(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH24)
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline(always)]
    pub fn aport2ych26(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH26)
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline(always)]
    pub fn aport2ych28(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH28)
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline(always)]
    pub fn aport2ych30(self) -> &'a mut W {
        self.variant(VASEL_A::APORT2YCH30)
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(VASEL_A::APORT1YCH31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VBSEL`"]
pub type VBSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBSEL`"]
pub struct VBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `VLPSEL`"]
pub type VLPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLPSEL`"]
pub struct VLPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VLPSEL_W<'a> {
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
#[doc = "Reader of field `CSRESEN`"]
pub type CSRESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSRESEN`"]
pub struct CSRESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESEN_W<'a> {
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
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0"]
    RES0,
    #[doc = "1: Internal capacitive sense resistor value 1"]
    RES1,
    #[doc = "2: Internal capacitive sense resistor value 2"]
    RES2,
    #[doc = "3: Internal capacitive sense resistor value 3"]
    RES3,
    #[doc = "4: Internal capacitive sense resistor value 4"]
    RES4,
    #[doc = "5: Internal capacitive sense resistor value 5"]
    RES5,
    #[doc = "6: Internal capacitive sense resistor value 6"]
    RES6,
    #[doc = "7: Internal capacitive sense resistor value 7"]
    RES7,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        match variant {
            CSRESSEL_A::RES0 => 0,
            CSRESSEL_A::RES1 => 1,
            CSRESSEL_A::RES2 => 2,
            CSRESSEL_A::RES3 => 3,
            CSRESSEL_A::RES4 => 4,
            CSRESSEL_A::RES5 => 5,
            CSRESSEL_A::RES6 => 6,
            CSRESSEL_A::RES7 => 7,
        }
    }
}
#[doc = "Reader of field `CSRESSEL`"]
pub type CSRESSEL_R = crate::R<u8, CSRESSEL_A>;
impl CSRESSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRESSEL_A {
        match self.bits {
            0 => CSRESSEL_A::RES0,
            1 => CSRESSEL_A::RES1,
            2 => CSRESSEL_A::RES2,
            3 => CSRESSEL_A::RES3,
            4 => CSRESSEL_A::RES4,
            5 => CSRESSEL_A::RES5,
            6 => CSRESSEL_A::RES6,
            7 => CSRESSEL_A::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL_A::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == CSRESSEL_A::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == CSRESSEL_A::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == CSRESSEL_A::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == CSRESSEL_A::RES7
    }
}
#[doc = "Write proxy for field `CSRESSEL`"]
pub struct CSRESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRESSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES6)
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    pub fn vasel(&self) -> VASEL_R {
        VASEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    pub fn vbsel(&self) -> VBSEL_R {
        VBSEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    pub fn vlpsel(&self) -> VLPSEL_R {
        VLPSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CSRESEN_R {
        CSRESEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    pub fn vasel(&mut self) -> VASEL_W {
        VASEL_W { w: self }
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    pub fn vbsel(&mut self) -> VBSEL_W {
        VBSEL_W { w: self }
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    pub fn vlpsel(&mut self) -> VLPSEL_W {
        VLPSEL_W { w: self }
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&mut self) -> CSRESEN_W {
        CSRESEN_W { w: self }
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CSRESSEL_W {
        CSRESSEL_W { w: self }
    }
}
