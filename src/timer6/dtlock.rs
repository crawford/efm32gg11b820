#[doc = "Reader of register DTLOCK"]
pub type R = crate::R<u32, super::DTLOCK>;
#[doc = "Writer for register DTLOCK"]
pub type W = crate::W<u32, super::DTLOCK>;
#[doc = "Register DTLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::DTLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DTI Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED,
    #[doc = "1: `1`"]
    LOCKED,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        match variant {
            LOCKKEY_A::UNLOCKED => 0,
            LOCKKEY_A::LOCKED => 1,
        }
    }
}
#[doc = "Reader of field `LOCKKEY`"]
pub type LOCKKEY_R = crate::R<u16, LOCKKEY_A>;
impl LOCKKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, LOCKKEY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCKKEY_A::UNLOCKED),
            1 => Val(LOCKKEY_A::LOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCKKEY`"]
pub struct LOCKKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LOCKKEY_W {
        LOCKKEY_W { w: self }
    }
}
