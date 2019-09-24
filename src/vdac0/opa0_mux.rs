#[doc = "Reader of register OPA0_MUX"]
pub type R = crate::R<u32, super::OPA0_MUX>;
#[doc = "Writer for register OPA0_MUX"]
pub type W = crate::W<u32, super::OPA0_MUX>;
#[doc = "Register OPA0_MUX `reset()`'s with value 0x0016_f2f1"]
impl crate::ResetValue for super::OPA0_MUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0016_f2f1
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
#[doc = "OPAx Resistor Ladder Input Mux\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESINMUX_A {
    #[doc = "0: Set for Unity Gain"]
    DISABLE,
    #[doc = "1: Set for NEXTOUT(x-1) input"]
    OPANEXT,
    #[doc = "2: NEG pad connected"]
    NEGPAD,
    #[doc = "3: POS pad connected"]
    POSPAD,
    #[doc = "4: Neg pad of OPA0 connected. Direct input to support common reference."]
    COMPAD,
    #[doc = "5: OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    CENTER,
    #[doc = "6: VSS connected"]
    VSS,
}
impl From<RESINMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: RESINMUX_A) -> Self {
        match variant {
            RESINMUX_A::DISABLE => 0,
            RESINMUX_A::OPANEXT => 1,
            RESINMUX_A::NEGPAD => 2,
            RESINMUX_A::POSPAD => 3,
            RESINMUX_A::COMPAD => 4,
            RESINMUX_A::CENTER => 5,
            RESINMUX_A::VSS => 6,
        }
    }
}
#[doc = "Reader of field `RESINMUX`"]
pub type RESINMUX_R = crate::R<u8, RESINMUX_A>;
impl RESINMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RESINMUX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RESINMUX_A::DISABLE),
            1 => Val(RESINMUX_A::OPANEXT),
            2 => Val(RESINMUX_A::NEGPAD),
            3 => Val(RESINMUX_A::POSPAD),
            4 => Val(RESINMUX_A::COMPAD),
            5 => Val(RESINMUX_A::CENTER),
            6 => Val(RESINMUX_A::VSS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESINMUX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `OPANEXT`"]
    #[inline(always)]
    pub fn is_opanext(&self) -> bool {
        *self == RESINMUX_A::OPANEXT
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline(always)]
    pub fn is_negpad(&self) -> bool {
        *self == RESINMUX_A::NEGPAD
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline(always)]
    pub fn is_pospad(&self) -> bool {
        *self == RESINMUX_A::POSPAD
    }
    #[doc = "Checks if the value of the field is `COMPAD`"]
    #[inline(always)]
    pub fn is_compad(&self) -> bool {
        *self == RESINMUX_A::COMPAD
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == RESINMUX_A::CENTER
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == RESINMUX_A::VSS
    }
}
#[doc = "Write proxy for field `RESINMUX`"]
pub struct RESINMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESINMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESINMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set for Unity Gain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESINMUX_A::DISABLE)
    }
    #[doc = "Set for NEXTOUT(x-1) input"]
    #[inline(always)]
    pub fn opanext(self) -> &'a mut W {
        self.variant(RESINMUX_A::OPANEXT)
    }
    #[doc = "NEG pad connected"]
    #[inline(always)]
    pub fn negpad(self) -> &'a mut W {
        self.variant(RESINMUX_A::NEGPAD)
    }
    #[doc = "POS pad connected"]
    #[inline(always)]
    pub fn pospad(self) -> &'a mut W {
        self.variant(RESINMUX_A::POSPAD)
    }
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    #[inline(always)]
    pub fn compad(self) -> &'a mut W {
        self.variant(RESINMUX_A::COMPAD)
    }
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(RESINMUX_A::CENTER)
    }
    #[doc = "VSS connected"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(RESINMUX_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `GAIN3X`"]
pub type GAIN3X_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GAIN3X`"]
pub struct GAIN3X_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN3X_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "OPAx Resistor Ladder Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSEL_A {
    #[doc = "0: Gain of 1/3"]
    RES0,
    #[doc = "1: Gain of 1"]
    RES1,
    #[doc = "2: Gain of 1 2/3"]
    RES2,
    #[doc = "3: Gain of 2 1/5"]
    RES3,
    #[doc = "4: Gain of 3"]
    RES4,
    #[doc = "5: Gain of 4 1/3"]
    RES5,
    #[doc = "6: Gain of 7"]
    RES6,
    #[doc = "7: Gain of 15"]
    RES7,
}
impl From<RESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        match variant {
            RESSEL_A::RES0 => 0,
            RESSEL_A::RES1 => 1,
            RESSEL_A::RES2 => 2,
            RESSEL_A::RES3 => 3,
            RESSEL_A::RES4 => 4,
            RESSEL_A::RES5 => 5,
            RESSEL_A::RES6 => 6,
            RESSEL_A::RES7 => 7,
        }
    }
}
#[doc = "Reader of field `RESSEL`"]
pub type RESSEL_R = crate::R<u8, RESSEL_A>;
impl RESSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            0 => RESSEL_A::RES0,
            1 => RESSEL_A::RES1,
            2 => RESSEL_A::RES2,
            3 => RESSEL_A::RES3,
            4 => RESSEL_A::RES4,
            5 => RESSEL_A::RES5,
            6 => RESSEL_A::RES6,
            7 => RESSEL_A::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == RESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == RESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == RESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == RESSEL_A::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == RESSEL_A::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == RESSEL_A::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == RESSEL_A::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == RESSEL_A::RES7
    }
}
#[doc = "Write proxy for field `RESSEL`"]
pub struct RESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Gain of 1/3"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESSEL_A::RES0)
    }
    #[doc = "Gain of 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(RESSEL_A::RES1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(RESSEL_A::RES2)
    }
    #[doc = "Gain of 2 1/5"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(RESSEL_A::RES3)
    }
    #[doc = "Gain of 3"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut W {
        self.variant(RESSEL_A::RES4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut W {
        self.variant(RESSEL_A::RES5)
    }
    #[doc = "Gain of 7"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut W {
        self.variant(RESSEL_A::RES6)
    }
    #[doc = "Gain of 15"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut W {
        self.variant(RESSEL_A::RES7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&self) -> RESINMUX_R {
        RESINMUX_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    pub fn gain3x(&self) -> GAIN3X_R {
        GAIN3X_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&mut self) -> RESINMUX_W {
        RESINMUX_W { w: self }
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    pub fn gain3x(&mut self) -> GAIN3X_W {
        GAIN3X_W { w: self }
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W {
        RESSEL_W { w: self }
    }
}
