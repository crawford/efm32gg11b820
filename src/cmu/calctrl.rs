#[doc = "Reader of register CALCTRL"]
pub type R = crate::R<u32, super::CALCTRL>;
#[doc = "Writer for register CALCTRL"]
pub type W = crate::W<u32, super::CALCTRL>;
#[doc = "Register CALCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CALCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPSEL_A {
    #[doc = "0: Select HFXO as up-counter"]
    HFXO,
    #[doc = "1: Select LFXO as up-counter"]
    LFXO,
    #[doc = "2: Select HFRCO as up-counter"]
    HFRCO,
    #[doc = "3: Select LFRCO as up-counter"]
    LFRCO,
    #[doc = "4: Select AUXHFRCO as up-counter"]
    AUXHFRCO,
    #[doc = "5: Select PRS input selected by PRSUPSEL as up-counter"]
    PRS,
    #[doc = "7: Select USHFRCO as up-counter"]
    USHFRCO,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        match variant {
            UPSEL_A::HFXO => 0,
            UPSEL_A::LFXO => 1,
            UPSEL_A::HFRCO => 2,
            UPSEL_A::LFRCO => 3,
            UPSEL_A::AUXHFRCO => 4,
            UPSEL_A::PRS => 5,
            UPSEL_A::USHFRCO => 7,
        }
    }
}
#[doc = "Reader of field `UPSEL`"]
pub type UPSEL_R = crate::R<u8, UPSEL_A>;
impl UPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UPSEL_A::HFXO),
            1 => Val(UPSEL_A::LFXO),
            2 => Val(UPSEL_A::HFRCO),
            3 => Val(UPSEL_A::LFRCO),
            4 => Val(UPSEL_A::AUXHFRCO),
            5 => Val(UPSEL_A::PRS),
            7 => Val(UPSEL_A::USHFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == UPSEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == UPSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `UPSEL`"]
pub struct UPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFXO as up-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "Select LFXO as up-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "Select HFRCO as up-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(UPSEL_A::PRS)
    }
    #[doc = "Select USHFRCO as up-counter"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNSEL_A {
    #[doc = "0: Select HFCLK for down-counter"]
    HFCLK,
    #[doc = "1: Select HFXO for down-counter"]
    HFXO,
    #[doc = "2: Select LFXO for down-counter"]
    LFXO,
    #[doc = "3: Select HFRCO for down-counter"]
    HFRCO,
    #[doc = "4: Select LFRCO for down-counter"]
    LFRCO,
    #[doc = "5: Select AUXHFRCO for down-counter"]
    AUXHFRCO,
    #[doc = "6: Select PRS input selected by PRSDOWNSEL as down-counter"]
    PRS,
    #[doc = "8: Select USHFRCO for down-counter"]
    USHFRCO,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        match variant {
            DOWNSEL_A::HFCLK => 0,
            DOWNSEL_A::HFXO => 1,
            DOWNSEL_A::LFXO => 2,
            DOWNSEL_A::HFRCO => 3,
            DOWNSEL_A::LFRCO => 4,
            DOWNSEL_A::AUXHFRCO => 5,
            DOWNSEL_A::PRS => 6,
            DOWNSEL_A::USHFRCO => 8,
        }
    }
}
#[doc = "Reader of field `DOWNSEL`"]
pub type DOWNSEL_R = crate::R<u8, DOWNSEL_A>;
impl DOWNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DOWNSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DOWNSEL_A::HFCLK),
            1 => Val(DOWNSEL_A::HFXO),
            2 => Val(DOWNSEL_A::LFXO),
            3 => Val(DOWNSEL_A::HFRCO),
            4 => Val(DOWNSEL_A::LFRCO),
            5 => Val(DOWNSEL_A::AUXHFRCO),
            6 => Val(DOWNSEL_A::PRS),
            8 => Val(DOWNSEL_A::USHFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSEL_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == DOWNSEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == DOWNSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `DOWNSEL`"]
pub struct DOWNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFCLK for down-counter"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFCLK)
    }
    #[doc = "Select HFXO for down-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "Select LFXO for down-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "Select HFRCO for down-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(DOWNSEL_A::PRS)
    }
    #[doc = "Select USHFRCO for down-counter"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "PRS Select for PRS Input When Selected in UPSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSUPSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23,
}
impl From<PRSUPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSUPSEL_A) -> Self {
        match variant {
            PRSUPSEL_A::PRSCH0 => 0,
            PRSUPSEL_A::PRSCH1 => 1,
            PRSUPSEL_A::PRSCH2 => 2,
            PRSUPSEL_A::PRSCH3 => 3,
            PRSUPSEL_A::PRSCH4 => 4,
            PRSUPSEL_A::PRSCH5 => 5,
            PRSUPSEL_A::PRSCH6 => 6,
            PRSUPSEL_A::PRSCH7 => 7,
            PRSUPSEL_A::PRSCH8 => 8,
            PRSUPSEL_A::PRSCH9 => 9,
            PRSUPSEL_A::PRSCH10 => 10,
            PRSUPSEL_A::PRSCH11 => 11,
            PRSUPSEL_A::PRSCH12 => 12,
            PRSUPSEL_A::PRSCH13 => 13,
            PRSUPSEL_A::PRSCH14 => 14,
            PRSUPSEL_A::PRSCH15 => 15,
            PRSUPSEL_A::PRSCH16 => 16,
            PRSUPSEL_A::PRSCH17 => 17,
            PRSUPSEL_A::PRSCH18 => 18,
            PRSUPSEL_A::PRSCH19 => 19,
            PRSUPSEL_A::PRSCH20 => 20,
            PRSUPSEL_A::PRSCH21 => 21,
            PRSUPSEL_A::PRSCH22 => 22,
            PRSUPSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSUPSEL`"]
pub type PRSUPSEL_R = crate::R<u8, PRSUPSEL_A>;
impl PRSUPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSUPSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSUPSEL_A::PRSCH0),
            1 => Val(PRSUPSEL_A::PRSCH1),
            2 => Val(PRSUPSEL_A::PRSCH2),
            3 => Val(PRSUPSEL_A::PRSCH3),
            4 => Val(PRSUPSEL_A::PRSCH4),
            5 => Val(PRSUPSEL_A::PRSCH5),
            6 => Val(PRSUPSEL_A::PRSCH6),
            7 => Val(PRSUPSEL_A::PRSCH7),
            8 => Val(PRSUPSEL_A::PRSCH8),
            9 => Val(PRSUPSEL_A::PRSCH9),
            10 => Val(PRSUPSEL_A::PRSCH10),
            11 => Val(PRSUPSEL_A::PRSCH11),
            12 => Val(PRSUPSEL_A::PRSCH12),
            13 => Val(PRSUPSEL_A::PRSCH13),
            14 => Val(PRSUPSEL_A::PRSCH14),
            15 => Val(PRSUPSEL_A::PRSCH15),
            16 => Val(PRSUPSEL_A::PRSCH16),
            17 => Val(PRSUPSEL_A::PRSCH17),
            18 => Val(PRSUPSEL_A::PRSCH18),
            19 => Val(PRSUPSEL_A::PRSCH19),
            20 => Val(PRSUPSEL_A::PRSCH20),
            21 => Val(PRSUPSEL_A::PRSCH21),
            22 => Val(PRSUPSEL_A::PRSCH22),
            23 => Val(PRSUPSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSUPSEL`"]
pub struct PRSUPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSUPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSUPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "PRS Select for PRS Input When Selected in DOWNSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSDOWNSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23,
}
impl From<PRSDOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSDOWNSEL_A) -> Self {
        match variant {
            PRSDOWNSEL_A::PRSCH0 => 0,
            PRSDOWNSEL_A::PRSCH1 => 1,
            PRSDOWNSEL_A::PRSCH2 => 2,
            PRSDOWNSEL_A::PRSCH3 => 3,
            PRSDOWNSEL_A::PRSCH4 => 4,
            PRSDOWNSEL_A::PRSCH5 => 5,
            PRSDOWNSEL_A::PRSCH6 => 6,
            PRSDOWNSEL_A::PRSCH7 => 7,
            PRSDOWNSEL_A::PRSCH8 => 8,
            PRSDOWNSEL_A::PRSCH9 => 9,
            PRSDOWNSEL_A::PRSCH10 => 10,
            PRSDOWNSEL_A::PRSCH11 => 11,
            PRSDOWNSEL_A::PRSCH12 => 12,
            PRSDOWNSEL_A::PRSCH13 => 13,
            PRSDOWNSEL_A::PRSCH14 => 14,
            PRSDOWNSEL_A::PRSCH15 => 15,
            PRSDOWNSEL_A::PRSCH16 => 16,
            PRSDOWNSEL_A::PRSCH17 => 17,
            PRSDOWNSEL_A::PRSCH18 => 18,
            PRSDOWNSEL_A::PRSCH19 => 19,
            PRSDOWNSEL_A::PRSCH20 => 20,
            PRSDOWNSEL_A::PRSCH21 => 21,
            PRSDOWNSEL_A::PRSCH22 => 22,
            PRSDOWNSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `PRSDOWNSEL`"]
pub type PRSDOWNSEL_R = crate::R<u8, PRSDOWNSEL_A>;
impl PRSDOWNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRSDOWNSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRSDOWNSEL_A::PRSCH0),
            1 => Val(PRSDOWNSEL_A::PRSCH1),
            2 => Val(PRSDOWNSEL_A::PRSCH2),
            3 => Val(PRSDOWNSEL_A::PRSCH3),
            4 => Val(PRSDOWNSEL_A::PRSCH4),
            5 => Val(PRSDOWNSEL_A::PRSCH5),
            6 => Val(PRSDOWNSEL_A::PRSCH6),
            7 => Val(PRSDOWNSEL_A::PRSCH7),
            8 => Val(PRSDOWNSEL_A::PRSCH8),
            9 => Val(PRSDOWNSEL_A::PRSCH9),
            10 => Val(PRSDOWNSEL_A::PRSCH10),
            11 => Val(PRSDOWNSEL_A::PRSCH11),
            12 => Val(PRSDOWNSEL_A::PRSCH12),
            13 => Val(PRSDOWNSEL_A::PRSCH13),
            14 => Val(PRSDOWNSEL_A::PRSCH14),
            15 => Val(PRSDOWNSEL_A::PRSCH15),
            16 => Val(PRSDOWNSEL_A::PRSCH16),
            17 => Val(PRSDOWNSEL_A::PRSCH17),
            18 => Val(PRSDOWNSEL_A::PRSCH18),
            19 => Val(PRSDOWNSEL_A::PRSCH19),
            20 => Val(PRSDOWNSEL_A::PRSCH20),
            21 => Val(PRSDOWNSEL_A::PRSCH21),
            22 => Val(PRSDOWNSEL_A::PRSCH22),
            23 => Val(PRSDOWNSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `PRSDOWNSEL`"]
pub struct PRSDOWNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSDOWNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSDOWNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    pub fn prsupsel(&self) -> PRSUPSEL_R {
        PRSUPSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    pub fn prsdownsel(&self) -> PRSDOWNSEL_R {
        PRSDOWNSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UPSEL_W {
        UPSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DOWNSEL_W {
        DOWNSEL_W { w: self }
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 16:20 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    pub fn prsupsel(&mut self) -> PRSUPSEL_W {
        PRSUPSEL_W { w: self }
    }
    #[doc = "Bits 24:28 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    pub fn prsdownsel(&mut self) -> PRSDOWNSEL_W {
        PRSDOWNSEL_W { w: self }
    }
}
