#[doc = "Reader of register ROUTELOC0"]
pub type R = crate::R<u32, super::ROUTELOC0>;
#[doc = "Writer for register ROUTELOC0"]
pub type W = crate::W<u32, super::ROUTELOC0>;
#[doc = "Register ROUTELOC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location for D0-7 Pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
}
impl From<DATLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLOC_A) -> Self {
        match variant {
            DATLOC_A::LOC0 => 0,
            DATLOC_A::LOC1 => 1,
        }
    }
}
#[doc = "Reader of field `DATLOC`"]
pub type DATLOC_R = crate::R<u8, DATLOC_A>;
impl DATLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATLOC_A::LOC0),
            1 => Val(DATLOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DATLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DATLOC_A::LOC1
    }
}
#[doc = "Write proxy for field `DATLOC`"]
pub struct DATLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DATLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DATLOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location for CD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<CDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDLOC_A) -> Self {
        match variant {
            CDLOC_A::LOC0 => 0,
            CDLOC_A::LOC1 => 1,
            CDLOC_A::LOC2 => 2,
            CDLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `CDLOC`"]
pub type CDLOC_R = crate::R<u8, CDLOC_A>;
impl CDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CDLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CDLOC_A::LOC0),
            1 => Val(CDLOC_A::LOC1),
            2 => Val(CDLOC_A::LOC2),
            3 => Val(CDLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `CDLOC`"]
pub struct CDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "I/O Location for WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
}
impl From<WPLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: WPLOC_A) -> Self {
        match variant {
            WPLOC_A::LOC0 => 0,
            WPLOC_A::LOC1 => 1,
            WPLOC_A::LOC2 => 2,
            WPLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `WPLOC`"]
pub type WPLOC_R = crate::R<u8, WPLOC_A>;
impl WPLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WPLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WPLOC_A::LOC0),
            1 => Val(WPLOC_A::LOC1),
            2 => Val(WPLOC_A::LOC2),
            3 => Val(WPLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == WPLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == WPLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == WPLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == WPLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `WPLOC`"]
pub struct WPLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> WPLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "I/O Location for CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
}
impl From<CLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKLOC_A) -> Self {
        match variant {
            CLKLOC_A::LOC0 => 0,
            CLKLOC_A::LOC1 => 1,
        }
    }
}
#[doc = "Reader of field `CLKLOC`"]
pub type CLKLOC_R = crate::R<u8, CLKLOC_A>;
impl CLKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKLOC_A::LOC0),
            1 => Val(CLKLOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOC_A::LOC1
    }
}
#[doc = "Write proxy for field `CLKLOC`"]
pub struct CLKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&self) -> DATLOC_R {
        DATLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&self) -> CDLOC_R {
        CDLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&self) -> WPLOC_R {
        WPLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&self) -> CLKLOC_R {
        CLKLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&mut self) -> DATLOC_W {
        DATLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&mut self) -> CDLOC_W {
        CDLOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&mut self) -> WPLOC_W {
        WPLOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&mut self) -> CLKLOC_W {
        CLKLOC_W { w: self }
    }
}
