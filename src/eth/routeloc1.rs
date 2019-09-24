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
pub enum TSUEXTCLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<TSUEXTCLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUEXTCLKLOC_A) -> Self {
        match variant {
            TSUEXTCLKLOC_A::LOC0 => 0,
            TSUEXTCLKLOC_A::LOC1 => 1,
            TSUEXTCLKLOC_A::LOC2 => 2,
            TSUEXTCLKLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `TSUEXTCLKLOC`"]
pub type TSUEXTCLKLOC_R = crate::R<u8, TSUEXTCLKLOC_A>;
impl TSUEXTCLKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSUEXTCLKLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSUEXTCLKLOC_A::LOC0),
            1 => Val(TSUEXTCLKLOC_A::LOC1),
            2 => Val(TSUEXTCLKLOC_A::LOC2),
            3 => Val(TSUEXTCLKLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `TSUEXTCLKLOC`"]
pub struct TSUEXTCLKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUEXTCLKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSUEXTCLKLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSUTMRTOGLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<TSUTMRTOGLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUTMRTOGLOC_A) -> Self {
        match variant {
            TSUTMRTOGLOC_A::LOC0 => 0,
            TSUTMRTOGLOC_A::LOC1 => 1,
            TSUTMRTOGLOC_A::LOC2 => 2,
            TSUTMRTOGLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `TSUTMRTOGLOC`"]
pub type TSUTMRTOGLOC_R = crate::R<u8, TSUTMRTOGLOC_A>;
impl TSUTMRTOGLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSUTMRTOGLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSUTMRTOGLOC_A::LOC0),
            1 => Val(TSUTMRTOGLOC_A::LOC1),
            2 => Val(TSUTMRTOGLOC_A::LOC2),
            3 => Val(TSUTMRTOGLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `TSUTMRTOGLOC`"]
pub struct TSUTMRTOGLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUTMRTOGLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSUTMRTOGLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIOLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<MDIOLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIOLOC_A) -> Self {
        match variant {
            MDIOLOC_A::LOC0 => 0,
            MDIOLOC_A::LOC1 => 1,
            MDIOLOC_A::LOC2 => 2,
            MDIOLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `MDIOLOC`"]
pub type MDIOLOC_R = crate::R<u8, MDIOLOC_A>;
impl MDIOLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MDIOLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MDIOLOC_A::LOC0),
            1 => Val(MDIOLOC_A::LOC1),
            2 => Val(MDIOLOC_A::LOC2),
            3 => Val(MDIOLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MDIOLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MDIOLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MDIOLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == MDIOLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `MDIOLOC`"]
pub struct MDIOLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMIILOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
}
impl From<RMIILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RMIILOC_A) -> Self {
        match variant {
            RMIILOC_A::LOC0 => 0,
            RMIILOC_A::LOC1 => 1,
        }
    }
}
#[doc = "Reader of field `RMIILOC`"]
pub type RMIILOC_R = crate::R<u8, RMIILOC_A>;
impl RMIILOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RMIILOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RMIILOC_A::LOC0),
            1 => Val(RMIILOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RMIILOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RMIILOC_A::LOC1
    }
}
#[doc = "Write proxy for field `RMIILOC`"]
pub struct RMIILOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIILOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMIILOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RMIILOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RMIILOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&self) -> TSUEXTCLKLOC_R {
        TSUEXTCLKLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&self) -> TSUTMRTOGLOC_R {
        TSUTMRTOGLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&self) -> MDIOLOC_R {
        MDIOLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&self) -> RMIILOC_R {
        RMIILOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&mut self) -> TSUEXTCLKLOC_W {
        TSUEXTCLKLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&mut self) -> TSUTMRTOGLOC_W {
        TSUTMRTOGLOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&mut self) -> MDIOLOC_W {
        MDIOLOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&mut self) -> RMIILOC_W {
        RMIILOC_W { w: self }
    }
}
