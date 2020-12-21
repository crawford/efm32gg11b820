#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x4204"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4204
    }
}
#[doc = "WDOG Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDOGRMODE_A {
    #[doc = "0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<WDOGRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOGRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDOGRMODE`"]
pub type WDOGRMODE_R = crate::R<u8, WDOGRMODE_A>;
impl WDOGRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WDOGRMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WDOGRMODE_A::DISABLED),
            1 => Val(WDOGRMODE_A::LIMITED),
            2 => Val(WDOGRMODE_A::EXTENDED),
            4 => Val(WDOGRMODE_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODE_A::FULL
    }
}
#[doc = "Write proxy for field `WDOGRMODE`"]
pub struct WDOGRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOGRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Core LOCKUP Reset Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCKUPRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<LOCKUPRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCKUPRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCKUPRMODE`"]
pub type LOCKUPRMODE_R = crate::R<u8, LOCKUPRMODE_A>;
impl LOCKUPRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCKUPRMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCKUPRMODE_A::DISABLED),
            1 => Val(LOCKUPRMODE_A::LIMITED),
            2 => Val(LOCKUPRMODE_A::EXTENDED),
            4 => Val(LOCKUPRMODE_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODE_A::FULL
    }
}
#[doc = "Write proxy for field `LOCKUPRMODE`"]
pub struct LOCKUPRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUPRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUPRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Core Sysreset Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSRMODE_A {
    #[doc = "0: Reset request is blocked. "]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<SYSRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSRMODE`"]
pub type SYSRMODE_R = crate::R<u8, SYSRMODE_A>;
impl SYSRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSRMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSRMODE_A::DISABLED),
            1 => Val(SYSRMODE_A::LIMITED),
            2 => Val(SYSRMODE_A::EXTENDED),
            4 => Val(SYSRMODE_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODE_A::FULL
    }
}
#[doc = "Write proxy for field `SYSRMODE`"]
pub struct SYSRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(SYSRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(SYSRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(SYSRMODE_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "PIN Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINRMODE_A {
    #[doc = "0: Reset request is blocked. "]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset. "]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<PINRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINRMODE`"]
pub type PINRMODE_R = crate::R<u8, PINRMODE_A>;
impl PINRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINRMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PINRMODE_A::DISABLED),
            1 => Val(PINRMODE_A::LIMITED),
            2 => Val(PINRMODE_A::EXTENDED),
            4 => Val(PINRMODE_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PINRMODE_A::FULL
    }
}
#[doc = "Write proxy for field `PINRMODE`"]
pub struct PINRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PINRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(PINRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(PINRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(PINRMODE_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `RESETSTATE`"]
pub type RESETSTATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESETSTATE`"]
pub struct RESETSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&self) -> WDOGRMODE_R {
        WDOGRMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LOCKUPRMODE_R {
        LOCKUPRMODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SYSRMODE_R {
        SYSRMODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&self) -> PINRMODE_R {
        PINRMODE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&self) -> RESETSTATE_R {
        RESETSTATE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&mut self) -> WDOGRMODE_W {
        WDOGRMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&mut self) -> LOCKUPRMODE_W {
        LOCKUPRMODE_W { w: self }
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&mut self) -> SYSRMODE_W {
        SYSRMODE_W { w: self }
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&mut self) -> PINRMODE_W {
        PINRMODE_W { w: self }
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&mut self) -> RESETSTATE_W {
        RESETSTATE_W { w: self }
    }
}
