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
#[doc = "Repeat Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPMODE_A {
    #[doc = "0: When started, the LETIMER counts down until it is stopped by software"]
    FREE,
    #[doc = "1: The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    ONESHOT,
    #[doc = "2: The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    BUFFERED,
    #[doc = "3: Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE,
}
impl From<REPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REPMODE_A) -> Self {
        match variant {
            REPMODE_A::FREE => 0,
            REPMODE_A::ONESHOT => 1,
            REPMODE_A::BUFFERED => 2,
            REPMODE_A::DOUBLE => 3,
        }
    }
}
#[doc = "Reader of field `REPMODE`"]
pub type REPMODE_R = crate::R<u8, REPMODE_A>;
impl REPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPMODE_A {
        match self.bits {
            0 => REPMODE_A::FREE,
            1 => REPMODE_A::ONESHOT,
            2 => REPMODE_A::BUFFERED,
            3 => REPMODE_A::DOUBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == REPMODE_A::FREE
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == REPMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == REPMODE_A::BUFFERED
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == REPMODE_A::DOUBLE
    }
}
#[doc = "Write proxy for field `REPMODE`"]
pub struct REPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(REPMODE_A::FREE)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(REPMODE_A::ONESHOT)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(REPMODE_A::BUFFERED)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(REPMODE_A::DOUBLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Underflow Output Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFOA0_A {
    #[doc = "0: LETn_O0 is held at its idle value as defined by OPOL0"]
    NONE,
    #[doc = "1: LETn_O0 is toggled on CNT underflow"]
    TOGGLE,
    #[doc = "2: LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    PULSE,
    #[doc = "3: LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl From<UFOA0_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA0_A) -> Self {
        match variant {
            UFOA0_A::NONE => 0,
            UFOA0_A::TOGGLE => 1,
            UFOA0_A::PULSE => 2,
            UFOA0_A::PWM => 3,
        }
    }
}
#[doc = "Reader of field `UFOA0`"]
pub type UFOA0_R = crate::R<u8, UFOA0_A>;
impl UFOA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA0_A {
        match self.bits {
            0 => UFOA0_A::NONE,
            1 => UFOA0_A::TOGGLE,
            2 => UFOA0_A::PULSE,
            3 => UFOA0_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA0_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA0_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA0_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA0_A::PWM
    }
}
#[doc = "Write proxy for field `UFOA0`"]
pub struct UFOA0_W<'a> {
    w: &'a mut W,
}
impl<'a> UFOA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UFOA0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA0_A::NONE)
    }
    #[doc = "LETn_O0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA0_A::TOGGLE)
    }
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA0_A::PULSE)
    }
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA0_A::PWM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Underflow Output Action 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFOA1_A {
    #[doc = "0: LETn_O1 is held at its idle value as defined by OPOL1"]
    NONE,
    #[doc = "1: LETn_O1 is toggled on CNT underflow"]
    TOGGLE,
    #[doc = "2: LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    PULSE,
    #[doc = "3: LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl From<UFOA1_A> for u8 {
    #[inline(always)]
    fn from(variant: UFOA1_A) -> Self {
        match variant {
            UFOA1_A::NONE => 0,
            UFOA1_A::TOGGLE => 1,
            UFOA1_A::PULSE => 2,
            UFOA1_A::PWM => 3,
        }
    }
}
#[doc = "Reader of field `UFOA1`"]
pub type UFOA1_R = crate::R<u8, UFOA1_A>;
impl UFOA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFOA1_A {
        match self.bits {
            0 => UFOA1_A::NONE,
            1 => UFOA1_A::TOGGLE,
            2 => UFOA1_A::PULSE,
            3 => UFOA1_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UFOA1_A::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA1_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA1_A::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA1_A::PWM
    }
}
#[doc = "Write proxy for field `UFOA1`"]
pub struct UFOA1_W<'a> {
    w: &'a mut W,
}
impl<'a> UFOA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UFOA1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA1_A::NONE)
    }
    #[doc = "LETn_O1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA1_A::TOGGLE)
    }
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA1_A::PULSE)
    }
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA1_A::PWM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OPOL0`"]
pub type OPOL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPOL0`"]
pub struct OPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OPOL0_W<'a> {
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
#[doc = "Reader of field `OPOL1`"]
pub type OPOL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPOL1`"]
pub struct OPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OPOL1_W<'a> {
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
#[doc = "Reader of field `BUFTOP`"]
pub type BUFTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFTOP`"]
pub struct BUFTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFTOP_W<'a> {
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
#[doc = "Reader of field `COMP0TOP`"]
pub type COMP0TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP0TOP`"]
pub struct COMP0TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0TOP_W<'a> {
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
#[doc = "Reader of field `DEBUGRUN`"]
pub type DEBUGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGRUN`"]
pub struct DEBUGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGRUN_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&self) -> REPMODE_R {
        REPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&self) -> UFOA0_R {
        UFOA0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&self) -> UFOA1_R {
        UFOA1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&self) -> OPOL0_R {
        OPOL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&self) -> OPOL1_R {
        OPOL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&self) -> BUFTOP_R {
        BUFTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare Value 0 is Top Value"]
    #[inline(always)]
    pub fn comp0top(&self) -> COMP0TOP_R {
        COMP0TOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&mut self) -> REPMODE_W {
        REPMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&mut self) -> UFOA0_W {
        UFOA0_W { w: self }
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&mut self) -> UFOA1_W {
        UFOA1_W { w: self }
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&mut self) -> OPOL0_W {
        OPOL0_W { w: self }
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&mut self) -> OPOL1_W {
        OPOL1_W { w: self }
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&mut self) -> BUFTOP_W {
        BUFTOP_W { w: self }
    }
    #[doc = "Bit 9 - Compare Value 0 is Top Value"]
    #[inline(always)]
    pub fn comp0top(&mut self) -> COMP0TOP_W {
        COMP0TOP_W { w: self }
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
}
