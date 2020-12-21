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
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Up-count mode"]
    UP = 0,
    #[doc = "1: Down-count mode"]
    DOWN = 1,
    #[doc = "2: Up/down-count mode"]
    UPDOWN = 2,
    #[doc = "3: Quadrature decoder mode"]
    QDEC = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::UP,
            1 => MODE_A::DOWN,
            2 => MODE_A::UPDOWN,
            3 => MODE_A::QDEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MODE_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == MODE_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MODE_A::UPDOWN
    }
    #[doc = "Checks if the value of the field is `QDEC`"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODE_A::QDEC
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(MODE_A::UP)
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(MODE_A::DOWN)
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(MODE_A::UPDOWN)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut W {
        self.variant(MODE_A::QDEC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
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
#[doc = "Reader of field `OSMEN`"]
pub type OSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSMEN`"]
pub struct OSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSMEN_W<'a> {
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
#[doc = "Reader of field `QDM`"]
pub type QDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDM`"]
pub struct QDM_W<'a> {
    w: &'a mut W,
}
impl<'a> QDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMACLRACT`"]
pub type DMACLRACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACLRACT`"]
pub struct DMACLRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACLRACT_W<'a> {
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
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RISEA_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Start counter without reload"]
    START = 1,
    #[doc = "2: Stop counter without reload"]
    STOP = 2,
    #[doc = "3: Reload and start counter"]
    RELOADSTART = 3,
}
impl From<RISEA_A> for u8 {
    #[inline(always)]
    fn from(variant: RISEA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RISEA`"]
pub type RISEA_R = crate::R<u8, RISEA_A>;
impl RISEA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISEA_A {
        match self.bits {
            0 => RISEA_A::NONE,
            1 => RISEA_A::START,
            2 => RISEA_A::STOP,
            3 => RISEA_A::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RISEA_A::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RISEA_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RISEA_A::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == RISEA_A::RELOADSTART
    }
}
#[doc = "Write proxy for field `RISEA`"]
pub struct RISEA_W<'a> {
    w: &'a mut W,
}
impl<'a> RISEA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RISEA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RISEA_A::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RISEA_A::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RISEA_A::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(RISEA_A::RELOADSTART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FALLA_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Start counter without reload"]
    START = 1,
    #[doc = "2: Stop counter without reload"]
    STOP = 2,
    #[doc = "3: Reload and start counter"]
    RELOADSTART = 3,
}
impl From<FALLA_A> for u8 {
    #[inline(always)]
    fn from(variant: FALLA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FALLA`"]
pub type FALLA_R = crate::R<u8, FALLA_A>;
impl FALLA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLA_A {
        match self.bits {
            0 => FALLA_A::NONE,
            1 => FALLA_A::START,
            2 => FALLA_A::STOP,
            3 => FALLA_A::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FALLA_A::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == FALLA_A::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == FALLA_A::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == FALLA_A::RELOADSTART
    }
}
#[doc = "Write proxy for field `FALLA`"]
pub struct FALLA_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FALLA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FALLA_A::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(FALLA_A::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(FALLA_A::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(FALLA_A::RELOADSTART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `X2CNT`"]
pub type X2CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X2CNT`"]
pub struct X2CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> X2CNT_W<'a> {
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
#[doc = "Reader of field `DISSYNCOUT`"]
pub type DISSYNCOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISSYNCOUT`"]
pub struct DISSYNCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISSYNCOUT_W<'a> {
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
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Prescaled HFPERCLK"]
    PRESCHFPERCLK = 0,
    #[doc = "1: Compare/Capture Channel 1 Input"]
    CC1 = 1,
    #[doc = "2: Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    TIMEROUF = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::PRESCHFPERCLK),
            1 => Val(CLKSEL_A::CC1),
            2 => Val(CLKSEL_A::TIMEROUF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCHFPERCLK`"]
    #[inline(always)]
    pub fn is_preschfperclk(&self) -> bool {
        *self == CLKSEL_A::PRESCHFPERCLK
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == CLKSEL_A::CC1
    }
    #[doc = "Checks if the value of the field is `TIMEROUF`"]
    #[inline(always)]
    pub fn is_timerouf(&self) -> bool {
        *self == CLKSEL_A::TIMEROUF
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Prescaled HFPERCLK"]
    #[inline(always)]
    pub fn preschfperclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::PRESCHFPERCLK)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CLKSEL_A::CC1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn timerouf(self) -> &'a mut W {
        self.variant(CLKSEL_A::TIMEROUF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: The HFPERCLK is undivided"]
    DIV1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    DIV2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    DIV4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    DIV8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    DIV16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    DIV32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    DIV64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    DIV128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    DIV256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    DIV512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    DIV1024 = 10,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::DIV1),
            1 => Val(PRESC_A::DIV2),
            2 => Val(PRESC_A::DIV4),
            3 => Val(PRESC_A::DIV8),
            4 => Val(PRESC_A::DIV16),
            5 => Val(PRESC_A::DIV32),
            6 => Val(PRESC_A::DIV64),
            7 => Val(PRESC_A::DIV128),
            8 => Val(PRESC_A::DIV256),
            9 => Val(PRESC_A::DIV512),
            10 => Val(PRESC_A::DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESC_A::DIV1024
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESC_A::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ATI`"]
pub type ATI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATI`"]
pub struct ATI_W<'a> {
    w: &'a mut W,
}
impl<'a> ATI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RSSCOIST`"]
pub type RSSCOIST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSSCOIST`"]
pub struct RSSCOIST_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSCOIST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OSMEN_R {
        OSMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QDM_R {
        QDM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DMACLRACT_R {
        DMACLRACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RISEA_R {
        RISEA_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FALLA_R {
        FALLA_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2CNT_R {
        X2CNT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Disable Timer From Start/Stop/Reload Other Synchronized Timers"]
    #[inline(always)]
    pub fn dissyncout(&self) -> DISSYNCOUT_R {
        DISSYNCOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> ATI_R {
        ATI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RSSCOIST_R {
        RSSCOIST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&mut self) -> OSMEN_W {
        OSMEN_W { w: self }
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&mut self) -> QDM_W {
        QDM_W { w: self }
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&mut self) -> DMACLRACT_W {
        DMACLRACT_W { w: self }
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&mut self) -> RISEA_W {
        RISEA_W { w: self }
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&mut self) -> FALLA_W {
        FALLA_W { w: self }
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&mut self) -> X2CNT_W {
        X2CNT_W { w: self }
    }
    #[doc = "Bit 14 - Disable Timer From Start/Stop/Reload Other Synchronized Timers"]
    #[inline(always)]
    pub fn dissyncout(&mut self) -> DISSYNCOUT_W {
        DISSYNCOUT_W { w: self }
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&mut self) -> ATI_W {
        ATI_W { w: self }
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output Initial State"]
    #[inline(always)]
    pub fn rsscoist(&mut self) -> RSSCOIST_W {
        RSSCOIST_W { w: self }
    }
}
