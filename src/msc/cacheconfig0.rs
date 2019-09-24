#[doc = "Reader of register CACHECONFIG0"]
pub type R = crate::R<u32, super::CACHECONFIG0>;
#[doc = "Writer for register CACHECONFIG0"]
pub type W = crate::W<u32, super::CACHECONFIG0>;
#[doc = "Register CACHECONFIG0 `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CACHECONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Instruction Cache Low-Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHELPLEVEL_A {
    #[doc = "0: Base instruction cache functionality."]
    BASE,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    ADVANCED,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY,
}
impl From<CACHELPLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CACHELPLEVEL_A) -> Self {
        match variant {
            CACHELPLEVEL_A::BASE => 0,
            CACHELPLEVEL_A::ADVANCED => 1,
            CACHELPLEVEL_A::MINACTIVITY => 3,
        }
    }
}
#[doc = "Reader of field `CACHELPLEVEL`"]
pub type CACHELPLEVEL_R = crate::R<u8, CACHELPLEVEL_A>;
impl CACHELPLEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CACHELPLEVEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CACHELPLEVEL_A::BASE),
            1 => Val(CACHELPLEVEL_A::ADVANCED),
            3 => Val(CACHELPLEVEL_A::MINACTIVITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BASE`"]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == CACHELPLEVEL_A::BASE
    }
    #[doc = "Checks if the value of the field is `ADVANCED`"]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == CACHELPLEVEL_A::ADVANCED
    }
    #[doc = "Checks if the value of the field is `MINACTIVITY`"]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == CACHELPLEVEL_A::MINACTIVITY
    }
}
#[doc = "Write proxy for field `CACHELPLEVEL`"]
pub struct CACHELPLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHELPLEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHELPLEVEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn base(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::BASE)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::ADVANCED)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::MINACTIVITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&self) -> CACHELPLEVEL_R {
        CACHELPLEVEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&mut self) -> CACHELPLEVEL_W {
        CACHELPLEVEL_W { w: self }
    }
}
