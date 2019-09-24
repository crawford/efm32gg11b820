#[doc = "Reader of register SCANCTRLX"]
pub type R = crate::R<u32, super::SCANCTRLX>;
#[doc = "Writer for register SCANCTRLX"]
pub type W = crate::W<u32, super::SCANCTRLX>;
#[doc = "Register SCANCTRLX `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANCTRLX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Scan Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFSEL_A {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    VBGR,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    VREFP,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW,
}
impl From<VREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFSEL_A) -> Self {
        match variant {
            VREFSEL_A::VBGR => 0,
            VREFSEL_A::VDDXWATT => 1,
            VREFSEL_A::VREFPWATT => 2,
            VREFSEL_A::VREFP => 3,
            VREFSEL_A::VREFPNWATT => 5,
            VREFSEL_A::VREFPN => 6,
            VREFSEL_A::VBGRLOW => 7,
        }
    }
}
#[doc = "Reader of field `VREFSEL`"]
pub type VREFSEL_R = crate::R<u8, VREFSEL_A>;
impl VREFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VREFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VREFSEL_A::VBGR),
            1 => Val(VREFSEL_A::VDDXWATT),
            2 => Val(VREFSEL_A::VREFPWATT),
            3 => Val(VREFSEL_A::VREFP),
            5 => Val(VREFSEL_A::VREFPNWATT),
            6 => Val(VREFSEL_A::VREFPN),
            7 => Val(VREFSEL_A::VBGRLOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VDDXWATT`"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL_A::VDDXWATT
    }
    #[doc = "Checks if the value of the field is `VREFPWATT`"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPWATT
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL_A::VREFP
    }
    #[doc = "Checks if the value of the field is `VREFPNWATT`"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPNWATT
    }
    #[doc = "Checks if the value of the field is `VREFPN`"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL_A::VREFPN
    }
    #[doc = "Checks if the value of the field is `VBGRLOW`"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL_A::VBGRLOW
    }
}
#[doc = "Write proxy for field `VREFSEL`"]
pub struct VREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFP)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGRLOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `VREFATTFIX`"]
pub type VREFATTFIX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFATTFIX`"]
pub struct VREFATTFIX_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFATTFIX_W<'a> {
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
#[doc = "Reader of field `VREFATT`"]
pub type VREFATT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREFATT`"]
pub struct VREFATT_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFATT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `VINATT`"]
pub type VINATT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VINATT`"]
pub struct VINATT_W<'a> {
    w: &'a mut W,
}
impl<'a> VINATT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DVL`"]
pub type DVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DVL`"]
pub struct DVL_W<'a> {
    w: &'a mut W,
}
impl<'a> DVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `FIFOOFACT`"]
pub type FIFOOFACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOOFACT`"]
pub struct FIFOOFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOFACT_W<'a> {
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
#[doc = "Reader of field `PRSMODE`"]
pub type PRSMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSMODE`"]
pub struct PRSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSMODE_W<'a> {
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
#[doc = "Scan Sequence PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers scan sequence"]
    PRSCH0,
    #[doc = "1: PRS ch 1 triggers scan sequence"]
    PRSCH1,
    #[doc = "2: PRS ch 2 triggers scan sequence"]
    PRSCH2,
    #[doc = "3: PRS ch 3 triggers scan sequence"]
    PRSCH3,
    #[doc = "4: PRS ch 4 triggers scan sequence"]
    PRSCH4,
    #[doc = "5: PRS ch 5 triggers scan sequence"]
    PRSCH5,
    #[doc = "6: PRS ch 6 triggers scan sequence"]
    PRSCH6,
    #[doc = "7: PRS ch 7 triggers scan sequence"]
    PRSCH7,
    #[doc = "8: PRS ch 8 triggers scan sequence"]
    PRSCH8,
    #[doc = "9: PRS ch 9 triggers scan sequence"]
    PRSCH9,
    #[doc = "10: PRS ch 10 triggers scan sequence"]
    PRSCH10,
    #[doc = "11: PRS ch 11 triggers scan sequence"]
    PRSCH11,
    #[doc = "12: PRS ch 12 triggers scan sequence"]
    PRSCH12,
    #[doc = "13: PRS ch 13 triggers scan sequence"]
    PRSCH13,
    #[doc = "14: PRS ch 14 triggers scan sequence"]
    PRSCH14,
    #[doc = "15: PRS ch 15 triggers scan sequence"]
    PRSCH15,
    #[doc = "16: PRS ch 16 triggers scan sequence"]
    PRSCH16,
    #[doc = "17: PRS ch 17 triggers scan sequence"]
    PRSCH17,
    #[doc = "18: PRS ch 18 triggers scan sequence"]
    PRSCH18,
    #[doc = "19: PRS ch 19 triggers scan sequence"]
    PRSCH19,
    #[doc = "20: PRS ch 20 triggers scan sequence"]
    PRSCH20,
    #[doc = "21: PRS ch 21 triggers scan sequence"]
    PRSCH21,
    #[doc = "22: PRS ch 22 triggers scan sequence"]
    PRSCH22,
    #[doc = "23: PRS ch 23 triggers scan sequence"]
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
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `CONVSTARTDELAY`"]
pub type CONVSTARTDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONVSTARTDELAY`"]
pub struct CONVSTARTDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVSTARTDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | (((value as u32) & 0x1f) << 22);
        self.w
    }
}
#[doc = "Reader of field `CONVSTARTDELAYEN`"]
pub type CONVSTARTDELAYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONVSTARTDELAYEN`"]
pub struct CONVSTARTDELAYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVSTARTDELAYEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "REPDELAY Select for SCAN REP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPDELAY_A {
    #[doc = "0: No delay"]
    NODELAY,
    #[doc = "1: 4 conversion clock cycles"]
    _4CYCLES,
    #[doc = "2: 8 conversion clock cycles"]
    _8CYCLES,
    #[doc = "3: 16 conversion clock cycles"]
    _16CYCLES,
    #[doc = "4: 32 conversion clock cycles"]
    _32CYCLES,
    #[doc = "5: 64 conversion clock cycles"]
    _64CYCLES,
    #[doc = "6: 128 conversion clock cycles"]
    _128CYCLES,
    #[doc = "7: 256 conversion clock cycles"]
    _256CYCLES,
}
impl From<REPDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: REPDELAY_A) -> Self {
        match variant {
            REPDELAY_A::NODELAY => 0,
            REPDELAY_A::_4CYCLES => 1,
            REPDELAY_A::_8CYCLES => 2,
            REPDELAY_A::_16CYCLES => 3,
            REPDELAY_A::_32CYCLES => 4,
            REPDELAY_A::_64CYCLES => 5,
            REPDELAY_A::_128CYCLES => 6,
            REPDELAY_A::_256CYCLES => 7,
        }
    }
}
#[doc = "Reader of field `REPDELAY`"]
pub type REPDELAY_R = crate::R<u8, REPDELAY_A>;
impl REPDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPDELAY_A {
        match self.bits {
            0 => REPDELAY_A::NODELAY,
            1 => REPDELAY_A::_4CYCLES,
            2 => REPDELAY_A::_8CYCLES,
            3 => REPDELAY_A::_16CYCLES,
            4 => REPDELAY_A::_32CYCLES,
            5 => REPDELAY_A::_64CYCLES,
            6 => REPDELAY_A::_128CYCLES,
            7 => REPDELAY_A::_256CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NODELAY`"]
    #[inline(always)]
    pub fn is_nodelay(&self) -> bool {
        *self == REPDELAY_A::NODELAY
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == REPDELAY_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REPDELAY_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REPDELAY_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REPDELAY_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REPDELAY_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == REPDELAY_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == REPDELAY_A::_256CYCLES
    }
}
#[doc = "Write proxy for field `REPDELAY`"]
pub struct REPDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> REPDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPDELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No delay"]
    #[inline(always)]
    pub fn nodelay(self) -> &'a mut W {
        self.variant(REPDELAY_A::NODELAY)
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_256CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&self) -> VREFATTFIX_R {
        VREFATTFIX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&self) -> VREFATT_R {
        VREFATT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&self) -> VINATT_R {
        VINATT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FIFOOFACT_R {
        FIFOOFACT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&self) -> CONVSTARTDELAY_R {
        CONVSTARTDELAY_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&self) -> CONVSTARTDELAYEN_R {
        CONVSTARTDELAYEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SCAN REP Mode"]
    #[inline(always)]
    pub fn repdelay(&self) -> REPDELAY_R {
        REPDELAY_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&mut self) -> VREFSEL_W {
        VREFSEL_W { w: self }
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W {
        VREFATTFIX_W { w: self }
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&mut self) -> VREFATT_W {
        VREFATT_W { w: self }
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&mut self) -> VINATT_W {
        VINATT_W { w: self }
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DVL_W {
        DVL_W { w: self }
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W {
        FIFOOFACT_W { w: self }
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&mut self) -> PRSMODE_W {
        PRSMODE_W { w: self }
    }
    #[doc = "Bits 17:21 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
    #[doc = "Bits 22:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W {
        CONVSTARTDELAY_W { w: self }
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W {
        CONVSTARTDELAYEN_W { w: self }
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SCAN REP Mode"]
    #[inline(always)]
    pub fn repdelay(&mut self) -> REPDELAY_W {
        REPDELAY_W { w: self }
    }
}
