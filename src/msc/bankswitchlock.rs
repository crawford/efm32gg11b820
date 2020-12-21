#[doc = "Reader of register BANKSWITCHLOCK"]
pub type R = crate::R<u32, super::BANKSWITCHLOCK>;
#[doc = "Writer for register BANKSWITCHLOCK"]
pub type W = crate::W<u32, super::BANKSWITCHLOCK>;
#[doc = "Register BANKSWITCHLOCK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BANKSWITCHLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Bank Switching Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BANKSWITCHLOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<BANKSWITCHLOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: BANKSWITCHLOCKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BANKSWITCHLOCKKEY`"]
pub type BANKSWITCHLOCKKEY_R = crate::R<u16, BANKSWITCHLOCKKEY_A>;
impl BANKSWITCHLOCKKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BANKSWITCHLOCKKEY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BANKSWITCHLOCKKEY_A::UNLOCKED),
            1 => Val(BANKSWITCHLOCKKEY_A::LOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::LOCKED
    }
}
#[doc = "Write proxy for field `BANKSWITCHLOCKKEY`"]
pub struct BANKSWITCHLOCKKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKSWITCHLOCKKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANKSWITCHLOCKKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEY_A::LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&self) -> BANKSWITCHLOCKKEY_R {
        BANKSWITCHLOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&mut self) -> BANKSWITCHLOCKKEY_W {
        BANKSWITCHLOCKKEY_W { w: self }
    }
}
