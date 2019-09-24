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
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIN0LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
    #[doc = "4: Location 4"]
    LOC4,
    #[doc = "5: Location 5"]
    LOC5,
    #[doc = "6: Location 6"]
    LOC6,
    #[doc = "7: Location 7"]
    LOC7,
}
impl From<CLKIN0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKIN0LOC_A) -> Self {
        match variant {
            CLKIN0LOC_A::LOC0 => 0,
            CLKIN0LOC_A::LOC1 => 1,
            CLKIN0LOC_A::LOC2 => 2,
            CLKIN0LOC_A::LOC3 => 3,
            CLKIN0LOC_A::LOC4 => 4,
            CLKIN0LOC_A::LOC5 => 5,
            CLKIN0LOC_A::LOC6 => 6,
            CLKIN0LOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `CLKIN0LOC`"]
pub type CLKIN0LOC_R = crate::R<u8, CLKIN0LOC_A>;
impl CLKIN0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKIN0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKIN0LOC_A::LOC0),
            1 => Val(CLKIN0LOC_A::LOC1),
            2 => Val(CLKIN0LOC_A::LOC2),
            3 => Val(CLKIN0LOC_A::LOC3),
            4 => Val(CLKIN0LOC_A::LOC4),
            5 => Val(CLKIN0LOC_A::LOC5),
            6 => Val(CLKIN0LOC_A::LOC6),
            7 => Val(CLKIN0LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKIN0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKIN0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKIN0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKIN0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKIN0LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKIN0LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKIN0LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CLKIN0LOC_A::LOC7
    }
}
#[doc = "Write proxy for field `CLKIN0LOC`"]
pub struct CLKIN0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKIN0LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CLKIN0LOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkin0loc(&self) -> CLKIN0LOC_R {
        CLKIN0LOC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkin0loc(&mut self) -> CLKIN0LOC_W {
        CLKIN0LOC_W { w: self }
    }
}
