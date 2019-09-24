#[doc = "Reader of register ROUTELOC2"]
pub type R = crate::R<u32, super::ROUTELOC2>;
#[doc = "Writer for register ROUTELOC2"]
pub type W = crate::W<u32, super::ROUTELOC2>;
#[doc = "Register ROUTELOC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTI0LOC_A {
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
}
impl From<CDTI0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI0LOC_A) -> Self {
        match variant {
            CDTI0LOC_A::LOC0 => 0,
            CDTI0LOC_A::LOC1 => 1,
            CDTI0LOC_A::LOC2 => 2,
            CDTI0LOC_A::LOC3 => 3,
            CDTI0LOC_A::LOC4 => 4,
        }
    }
}
#[doc = "Reader of field `CDTI0LOC`"]
pub type CDTI0LOC_R = crate::R<u8, CDTI0LOC_A>;
impl CDTI0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CDTI0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CDTI0LOC_A::LOC0),
            1 => Val(CDTI0LOC_A::LOC1),
            2 => Val(CDTI0LOC_A::LOC2),
            3 => Val(CDTI0LOC_A::LOC3),
            4 => Val(CDTI0LOC_A::LOC4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI0LOC_A::LOC4
    }
}
#[doc = "Write proxy for field `CDTI0LOC`"]
pub struct CDTI0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTI0LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC4)
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
pub enum CDTI1LOC_A {
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
}
impl From<CDTI1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI1LOC_A) -> Self {
        match variant {
            CDTI1LOC_A::LOC0 => 0,
            CDTI1LOC_A::LOC1 => 1,
            CDTI1LOC_A::LOC2 => 2,
            CDTI1LOC_A::LOC3 => 3,
            CDTI1LOC_A::LOC4 => 4,
        }
    }
}
#[doc = "Reader of field `CDTI1LOC`"]
pub type CDTI1LOC_R = crate::R<u8, CDTI1LOC_A>;
impl CDTI1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CDTI1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CDTI1LOC_A::LOC0),
            1 => Val(CDTI1LOC_A::LOC1),
            2 => Val(CDTI1LOC_A::LOC2),
            3 => Val(CDTI1LOC_A::LOC3),
            4 => Val(CDTI1LOC_A::LOC4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI1LOC_A::LOC4
    }
}
#[doc = "Write proxy for field `CDTI1LOC`"]
pub struct CDTI1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTI1LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC4)
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
pub enum CDTI2LOC_A {
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
}
impl From<CDTI2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI2LOC_A) -> Self {
        match variant {
            CDTI2LOC_A::LOC0 => 0,
            CDTI2LOC_A::LOC1 => 1,
            CDTI2LOC_A::LOC2 => 2,
            CDTI2LOC_A::LOC3 => 3,
            CDTI2LOC_A::LOC4 => 4,
        }
    }
}
#[doc = "Reader of field `CDTI2LOC`"]
pub type CDTI2LOC_R = crate::R<u8, CDTI2LOC_A>;
impl CDTI2LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CDTI2LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CDTI2LOC_A::LOC0),
            1 => Val(CDTI2LOC_A::LOC1),
            2 => Val(CDTI2LOC_A::LOC2),
            3 => Val(CDTI2LOC_A::LOC3),
            4 => Val(CDTI2LOC_A::LOC4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI2LOC_A::LOC4
    }
}
#[doc = "Write proxy for field `CDTI2LOC`"]
pub struct CDTI2LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI2LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTI2LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&self) -> CDTI0LOC_R {
        CDTI0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&self) -> CDTI1LOC_R {
        CDTI1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&self) -> CDTI2LOC_R {
        CDTI2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&mut self) -> CDTI0LOC_W {
        CDTI0LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&mut self) -> CDTI1LOC_W {
        CDTI1LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&mut self) -> CDTI2LOC_W {
        CDTI2LOC_W { w: self }
    }
}
