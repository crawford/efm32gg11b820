#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TIMERLOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<TIMERLOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: TIMERLOCKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMERLOCKKEY`"]
pub type TIMERLOCKKEY_R = crate::R<u16, TIMERLOCKKEY_A>;
impl TIMERLOCKKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TIMERLOCKKEY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMERLOCKKEY_A::UNLOCKED),
            1 => Val(TIMERLOCKKEY_A::LOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == TIMERLOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TIMERLOCKKEY_A::LOCKED
    }
}
#[doc = "Write proxy for field `TIMERLOCKKEY`"]
pub struct TIMERLOCKKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERLOCKKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMERLOCKKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEY_A::LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&self) -> TIMERLOCKKEY_R {
        TIMERLOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&mut self) -> TIMERLOCKKEY_W {
        TIMERLOCKKEY_W { w: self }
    }
}
