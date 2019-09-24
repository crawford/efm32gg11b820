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
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: The module is disabled."]
    DISABLE,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM3)."]
    OVSSINGLE,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD,
    #[doc = "4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    OVSQUAD1X,
    #[doc = "5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    OVSQUAD2X,
    #[doc = "6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    OVSQUAD4X,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::DISABLE => 0,
            MODE_A::OVSSINGLE => 1,
            MODE_A::EXTCLKSINGLE => 2,
            MODE_A::EXTCLKQUAD => 3,
            MODE_A::OVSQUAD1X => 4,
            MODE_A::OVSQUAD2X => 5,
            MODE_A::OVSQUAD4X => 6,
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
            0 => Val(MODE_A::DISABLE),
            1 => Val(MODE_A::OVSSINGLE),
            2 => Val(MODE_A::EXTCLKSINGLE),
            3 => Val(MODE_A::EXTCLKQUAD),
            4 => Val(MODE_A::OVSQUAD1X),
            5 => Val(MODE_A::OVSQUAD2X),
            6 => Val(MODE_A::OVSQUAD4X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `OVSSINGLE`"]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE_A::OVSSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKSINGLE`"]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE_A::EXTCLKSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKQUAD`"]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE_A::EXTCLKQUAD
    }
    #[doc = "Checks if the value of the field is `OVSQUAD1X`"]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE_A::OVSQUAD1X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD2X`"]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE_A::OVSQUAD2X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD4X`"]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE_A::OVSQUAD4X
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
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut W {
        self.variant(MODE_A::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKQUAD)
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD1X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD2X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD4X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FILT`"]
pub type FILT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILT`"]
pub struct FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_W<'a> {
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
#[doc = "Reader of field `RSTEN`"]
pub type RSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTEN`"]
pub struct RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEN_W<'a> {
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
#[doc = "Reader of field `CNTRSTEN`"]
pub type CNTRSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTRSTEN`"]
pub struct CNTRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTRSTEN_W<'a> {
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
#[doc = "Reader of field `AUXCNTRSTEN`"]
pub type AUXCNTRSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXCNTRSTEN`"]
pub struct AUXCNTRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXCNTRSTEN_W<'a> {
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
#[doc = "Reader of field `DEBUGHALT`"]
pub type DEBUGHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGHALT`"]
pub struct DEBUGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGHALT_W<'a> {
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
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
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
#[doc = "Reader of field `S1CDIR`"]
pub type S1CDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1CDIR`"]
pub struct S1CDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> S1CDIR_W<'a> {
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
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEV_A {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    BOTH,
    #[doc = "1: Only counts up on up-count events."]
    UP,
    #[doc = "2: Only counts down on down-count events."]
    DOWN,
    #[doc = "3: Never counts."]
    NONE,
}
impl From<CNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV_A) -> Self {
        match variant {
            CNTEV_A::BOTH => 0,
            CNTEV_A::UP => 1,
            CNTEV_A::DOWN => 2,
            CNTEV_A::NONE => 3,
        }
    }
}
#[doc = "Reader of field `CNTEV`"]
pub type CNTEV_R = crate::R<u8, CNTEV_A>;
impl CNTEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEV_A {
        match self.bits {
            0 => CNTEV_A::BOTH,
            1 => CNTEV_A::UP,
            2 => CNTEV_A::DOWN,
            3 => CNTEV_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV_A::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV_A::DOWN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV_A::NONE
    }
}
#[doc = "Write proxy for field `CNTEV`"]
pub struct CNTEV_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CNTEV_A::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTEV_A::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTEV_A::DOWN)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CNTEV_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Controls When the Auxiliary Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXCNTEV_A {
    #[doc = "0: Never counts."]
    NONE,
    #[doc = "1: Counts up on up-count events."]
    UP,
    #[doc = "2: Counts up on down-count events."]
    DOWN,
    #[doc = "3: Counts up on both up-count and down-count events."]
    BOTH,
}
impl From<AUXCNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV_A) -> Self {
        match variant {
            AUXCNTEV_A::NONE => 0,
            AUXCNTEV_A::UP => 1,
            AUXCNTEV_A::DOWN => 2,
            AUXCNTEV_A::BOTH => 3,
        }
    }
}
#[doc = "Reader of field `AUXCNTEV`"]
pub type AUXCNTEV_R = crate::R<u8, AUXCNTEV_A>;
impl AUXCNTEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXCNTEV_A {
        match self.bits {
            0 => AUXCNTEV_A::NONE,
            1 => AUXCNTEV_A::UP,
            2 => AUXCNTEV_A::DOWN,
            3 => AUXCNTEV_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV_A::DOWN
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV_A::BOTH
    }
}
#[doc = "Write proxy for field `AUXCNTEV`"]
pub struct AUXCNTEV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXCNTEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUXCNTEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CNTDIR`"]
pub type CNTDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTDIR`"]
pub struct CNTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTDIR_W<'a> {
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
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Sets the Mode for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCMODE_A {
    #[doc = "0: Triggered compare and clear not enabled."]
    DISABLED,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    PRS,
}
impl From<TCCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCMODE_A) -> Self {
        match variant {
            TCCMODE_A::DISABLED => 0,
            TCCMODE_A::LFA => 1,
            TCCMODE_A::PRS => 2,
        }
    }
}
#[doc = "Reader of field `TCCMODE`"]
pub type TCCMODE_R = crate::R<u8, TCCMODE_A>;
impl TCCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCCMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCCMODE_A::DISABLED),
            1 => Val(TCCMODE_A::LFA),
            2 => Val(TCCMODE_A::PRS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFA`"]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE_A::LFA
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE_A::PRS
    }
}
#[doc = "Write proxy for field `TCCMODE`"]
pub struct TCCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCCMODE_A::DISABLED)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut W {
        self.variant(TCCMODE_A::LFA)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(TCCMODE_A::PRS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Set the LFA Prescaler for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCPRESC_A {
    #[doc = "0: Compare and clear event each LFA cycle."]
    DIV1,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    DIV2,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    DIV4,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    DIV8,
}
impl From<TCCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRESC_A) -> Self {
        match variant {
            TCCPRESC_A::DIV1 => 0,
            TCCPRESC_A::DIV2 => 1,
            TCCPRESC_A::DIV4 => 2,
            TCCPRESC_A::DIV8 => 3,
        }
    }
}
#[doc = "Reader of field `TCCPRESC`"]
pub type TCCPRESC_R = crate::R<u8, TCCPRESC_A>;
impl TCCPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCPRESC_A {
        match self.bits {
            0 => TCCPRESC_A::DIV1,
            1 => TCCPRESC_A::DIV2,
            2 => TCCPRESC_A::DIV4,
            3 => TCCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC_A::DIV8
    }
}
#[doc = "Write proxy for field `TCCPRESC`"]
pub struct TCCPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Triggered Compare and Clear Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCCOMP_A {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    RANGE,
}
impl From<TCCCOMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCCOMP_A) -> Self {
        match variant {
            TCCCOMP_A::LTOE => 0,
            TCCCOMP_A::GTOE => 1,
            TCCCOMP_A::RANGE => 2,
        }
    }
}
#[doc = "Reader of field `TCCCOMP`"]
pub type TCCCOMP_R = crate::R<u8, TCCCOMP_A>;
impl TCCCOMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCCCOMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCCCOMP_A::LTOE),
            1 => Val(TCCCOMP_A::GTOE),
            2 => Val(TCCCOMP_A::RANGE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LTOE`"]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP_A::LTOE
    }
    #[doc = "Checks if the value of the field is `GTOE`"]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP_A::GTOE
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP_A::RANGE
    }
}
#[doc = "Write proxy for field `TCCCOMP`"]
pub struct TCCCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCCOMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCCOMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut W {
        self.variant(TCCCOMP_A::LTOE)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut W {
        self.variant(TCCCOMP_A::GTOE)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(TCCCOMP_A::RANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRSGATEEN`"]
pub type PRSGATEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSGATEEN`"]
pub struct PRSGATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSGATEEN_W<'a> {
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
#[doc = "Reader of field `TCCPRSPOL`"]
pub type TCCPRSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCCPRSPOL`"]
pub struct TCCPRSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCPRSPOL_W<'a> {
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
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCPRSSEL_A {
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
impl From<TCCPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRSSEL_A) -> Self {
        match variant {
            TCCPRSSEL_A::PRSCH0 => 0,
            TCCPRSSEL_A::PRSCH1 => 1,
            TCCPRSSEL_A::PRSCH2 => 2,
            TCCPRSSEL_A::PRSCH3 => 3,
            TCCPRSSEL_A::PRSCH4 => 4,
            TCCPRSSEL_A::PRSCH5 => 5,
            TCCPRSSEL_A::PRSCH6 => 6,
            TCCPRSSEL_A::PRSCH7 => 7,
            TCCPRSSEL_A::PRSCH8 => 8,
            TCCPRSSEL_A::PRSCH9 => 9,
            TCCPRSSEL_A::PRSCH10 => 10,
            TCCPRSSEL_A::PRSCH11 => 11,
            TCCPRSSEL_A::PRSCH12 => 12,
            TCCPRSSEL_A::PRSCH13 => 13,
            TCCPRSSEL_A::PRSCH14 => 14,
            TCCPRSSEL_A::PRSCH15 => 15,
            TCCPRSSEL_A::PRSCH16 => 16,
            TCCPRSSEL_A::PRSCH17 => 17,
            TCCPRSSEL_A::PRSCH18 => 18,
            TCCPRSSEL_A::PRSCH19 => 19,
            TCCPRSSEL_A::PRSCH20 => 20,
            TCCPRSSEL_A::PRSCH21 => 21,
            TCCPRSSEL_A::PRSCH22 => 22,
            TCCPRSSEL_A::PRSCH23 => 23,
        }
    }
}
#[doc = "Reader of field `TCCPRSSEL`"]
pub type TCCPRSSEL_R = crate::R<u8, TCCPRSSEL_A>;
impl TCCPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCCPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCCPRSSEL_A::PRSCH0),
            1 => Val(TCCPRSSEL_A::PRSCH1),
            2 => Val(TCCPRSSEL_A::PRSCH2),
            3 => Val(TCCPRSSEL_A::PRSCH3),
            4 => Val(TCCPRSSEL_A::PRSCH4),
            5 => Val(TCCPRSSEL_A::PRSCH5),
            6 => Val(TCCPRSSEL_A::PRSCH6),
            7 => Val(TCCPRSSEL_A::PRSCH7),
            8 => Val(TCCPRSSEL_A::PRSCH8),
            9 => Val(TCCPRSSEL_A::PRSCH9),
            10 => Val(TCCPRSSEL_A::PRSCH10),
            11 => Val(TCCPRSSEL_A::PRSCH11),
            12 => Val(TCCPRSSEL_A::PRSCH12),
            13 => Val(TCCPRSSEL_A::PRSCH13),
            14 => Val(TCCPRSSEL_A::PRSCH14),
            15 => Val(TCCPRSSEL_A::PRSCH15),
            16 => Val(TCCPRSSEL_A::PRSCH16),
            17 => Val(TCCPRSSEL_A::PRSCH17),
            18 => Val(TCCPRSSEL_A::PRSCH18),
            19 => Val(TCCPRSSEL_A::PRSCH19),
            20 => Val(TCCPRSSEL_A::PRSCH20),
            21 => Val(TCCPRSSEL_A::PRSCH21),
            22 => Val(TCCPRSSEL_A::PRSCH22),
            23 => Val(TCCPRSSEL_A::PRSCH23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH23
    }
}
#[doc = "Write proxy for field `TCCPRSSEL`"]
pub struct TCCPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `TOPBHFSEL`"]
pub type TOPBHFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOPBHFSEL`"]
pub struct TOPBHFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOPBHFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&self) -> CNTRSTEN_R {
        CNTRSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AUXCNTRSTEN_R {
        AUXCNTRSTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DEBUGHALT_R {
        DEBUGHALT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TCCMODE_R {
        TCCMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TCCPRESC_R {
        TCCPRESC_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TCCCOMP_R {
        TCCCOMP_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PRSGATEEN_R {
        PRSGATEEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TCCPRSPOL_R {
        TCCPRSPOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TCCPRSSEL_R {
        TCCPRSSEL_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&self) -> TOPBHFSEL_R {
        TOPBHFSEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W { w: self }
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&mut self) -> CNTRSTEN_W {
        CNTRSTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&mut self) -> AUXCNTRSTEN_W {
        AUXCNTRSTEN_W { w: self }
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&mut self) -> DEBUGHALT_W {
        DEBUGHALT_W { w: self }
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&mut self) -> S1CDIR_W {
        S1CDIR_W { w: self }
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&mut self) -> CNTEV_W {
        CNTEV_W { w: self }
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W {
        AUXCNTEV_W { w: self }
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CNTDIR_W {
        CNTDIR_W { w: self }
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&mut self) -> TCCMODE_W {
        TCCMODE_W { w: self }
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&mut self) -> TCCPRESC_W {
        TCCPRESC_W { w: self }
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&mut self) -> TCCCOMP_W {
        TCCCOMP_W { w: self }
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&mut self) -> PRSGATEEN_W {
        PRSGATEEN_W { w: self }
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&mut self) -> TCCPRSPOL_W {
        TCCPRSPOL_W { w: self }
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&mut self) -> TCCPRSSEL_W {
        TCCPRSSEL_W { w: self }
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&mut self) -> TOPBHFSEL_W {
        TOPBHFSEL_W { w: self }
    }
}
