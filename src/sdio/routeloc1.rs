#[doc = "Reader of register ROUTELOC1"]
pub type R = crate::R<u32, super::ROUTELOC1>;
#[doc = "Writer for register ROUTELOC1"]
pub type W = crate::W<u32, super::ROUTELOC1>;
#[doc = "Register ROUTELOC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location for CMD Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<CMDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDLOC`"]
pub type CMDLOC_R = crate::R<u8, CMDLOC_A>;
impl CMDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMDLOC_A::LOC0),
            1 => Val(CMDLOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CMDLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CMDLOC_A::LOC1
    }
}
#[doc = "Write proxy for field `CMDLOC`"]
pub struct CMDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CMDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CMDLOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&self) -> CMDLOC_R {
        CMDLOC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline(always)]
    pub fn cmdloc(&mut self) -> CMDLOC_W {
        CMDLOC_W { w: self }
    }
}
