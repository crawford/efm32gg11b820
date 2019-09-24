#[doc = "Reader of register CC1_CTRL"]
pub type R = crate::R<u32, super::CC1_CTRL>;
#[doc = "Writer for register CC1_CTRL"]
pub type W = crate::W<u32, super::CC1_CTRL>;
#[doc = "Register CC1_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CC1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Compare/Capture channel turned off"]
    OFF,
    #[doc = "1: Input capture"]
    INPUTCAPTURE,
    #[doc = "2: Output compare"]
    OUTPUTCOMPARE,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::OFF => 0,
            MODE_A::INPUTCAPTURE => 1,
            MODE_A::OUTPUTCOMPARE => 2,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::OFF),
            1 => Val(MODE_A::INPUTCAPTURE),
            2 => Val(MODE_A::OUTPUTCOMPARE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUTCAPTURE`"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE_A::INPUTCAPTURE
    }
    #[doc = "Checks if the value of the field is `OUTPUTCOMPARE`"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE_A::OUTPUTCOMPARE
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut W {
        self.variant(MODE_A::INPUTCAPTURE)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut W {
        self.variant(MODE_A::OUTPUTCOMPARE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOA_A {
    #[doc = "0: A single clock cycle pulse is generated on output"]
    PULSE,
    #[doc = "1: Toggle output on compare match"]
    TOGGLE,
    #[doc = "2: Clear output on compare match"]
    CLEAR,
    #[doc = "3: Set output on compare match"]
    SET,
}
impl From<CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOA_A) -> Self {
        match variant {
            CMOA_A::PULSE => 0,
            CMOA_A::TOGGLE => 1,
            CMOA_A::CLEAR => 2,
            CMOA_A::SET => 3,
        }
    }
}
#[doc = "Reader of field `CMOA`"]
pub type CMOA_R = crate::R<u8, CMOA_A>;
impl CMOA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOA_A {
        match self.bits {
            0 => CMOA_A::PULSE,
            1 => CMOA_A::TOGGLE,
            2 => CMOA_A::CLEAR,
            3 => CMOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMOA_A::PULSE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA_A::SET
    }
}
#[doc = "Write proxy for field `CMOA`"]
pub struct CMOA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMOA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMOA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(CMOA_A::PULSE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMOA_A::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMOA_A::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMOA_A::SET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING,
    #[doc = "1: Falling edges detected"]
    FALLING,
    #[doc = "2: Both edges detected"]
    BOTH,
    #[doc = "3: No edge detection, signal is left as it is"]
    NONE,
}
impl From<ICEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE_A) -> Self {
        match variant {
            ICEDGE_A::RISING => 0,
            ICEDGE_A::FALLING => 1,
            ICEDGE_A::BOTH => 2,
            ICEDGE_A::NONE => 3,
        }
    }
}
#[doc = "Reader of field `ICEDGE`"]
pub type ICEDGE_R = crate::R<u8, ICEDGE_A>;
impl ICEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDGE_A {
        match self.bits {
            0 => ICEDGE_A::RISING,
            1 => ICEDGE_A::FALLING,
            2 => ICEDGE_A::BOTH,
            3 => ICEDGE_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE_A::NONE
    }
}
#[doc = "Write proxy for field `ICEDGE`"]
pub struct ICEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ICEDGE_A::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ICEDGE_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSEL_A {
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `COMPBASE`"]
pub type COMPBASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPBASE`"]
pub struct COMPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPBASE_W<'a> {
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
#[doc = "Reader of field `COMPMASK`"]
pub type COMPMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPMASK`"]
pub struct COMPMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DAYCC`"]
pub type DAYCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAYCC`"]
pub struct DAYCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:10 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    pub fn compbase(&self) -> COMPBASE_R {
        COMPBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    pub fn compmask(&self) -> COMPMASK_R {
        COMPMASK_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    pub fn daycc(&self) -> DAYCC_R {
        DAYCC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CMOA_W {
        CMOA_W { w: self }
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> ICEDGE_W {
        ICEDGE_W { w: self }
    }
    #[doc = "Bits 6:10 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W { w: self }
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    pub fn compbase(&mut self) -> COMPBASE_W {
        COMPBASE_W { w: self }
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    pub fn compmask(&mut self) -> COMPMASK_W {
        COMPMASK_W { w: self }
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    pub fn daycc(&mut self) -> DAYCC_W {
        DAYCC_W { w: self }
    }
}
