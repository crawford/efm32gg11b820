#[doc = "Reader of register EXTIFCTRL"]
pub type R = crate::R<u32, super::EXTIFCTRL>;
#[doc = "Writer for register EXTIFCTRL"]
pub type W = crate::W<u32, super::EXTIFCTRL>;
#[doc = "Register EXTIFCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIFCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "APORT Selection for External Interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APORTSEL_A {
    #[doc = "0: APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    APORT0X,
    #[doc = "1: APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    APORT0Y,
    #[doc = "2: APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1X,
    #[doc = "3: APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1Y,
    #[doc = "4: APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1XY,
    #[doc = "5: APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2X,
    #[doc = "6: APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2Y,
    #[doc = "7: APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2YX,
    #[doc = "8: APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3X,
    #[doc = "9: APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3Y,
    #[doc = "10: APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3XY,
    #[doc = "11: APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4X,
    #[doc = "12: APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4Y,
    #[doc = "13: APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4YX,
}
impl From<APORTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: APORTSEL_A) -> Self {
        match variant {
            APORTSEL_A::APORT0X => 0,
            APORTSEL_A::APORT0Y => 1,
            APORTSEL_A::APORT1X => 2,
            APORTSEL_A::APORT1Y => 3,
            APORTSEL_A::APORT1XY => 4,
            APORTSEL_A::APORT2X => 5,
            APORTSEL_A::APORT2Y => 6,
            APORTSEL_A::APORT2YX => 7,
            APORTSEL_A::APORT3X => 8,
            APORTSEL_A::APORT3Y => 9,
            APORTSEL_A::APORT3XY => 10,
            APORTSEL_A::APORT4X => 11,
            APORTSEL_A::APORT4Y => 12,
            APORTSEL_A::APORT4YX => 13,
        }
    }
}
#[doc = "Reader of field `APORTSEL`"]
pub type APORTSEL_R = crate::R<u8, APORTSEL_A>;
impl APORTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APORTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(APORTSEL_A::APORT0X),
            1 => Val(APORTSEL_A::APORT0Y),
            2 => Val(APORTSEL_A::APORT1X),
            3 => Val(APORTSEL_A::APORT1Y),
            4 => Val(APORTSEL_A::APORT1XY),
            5 => Val(APORTSEL_A::APORT2X),
            6 => Val(APORTSEL_A::APORT2Y),
            7 => Val(APORTSEL_A::APORT2YX),
            8 => Val(APORTSEL_A::APORT3X),
            9 => Val(APORTSEL_A::APORT3Y),
            10 => Val(APORTSEL_A::APORT3XY),
            11 => Val(APORTSEL_A::APORT4X),
            12 => Val(APORTSEL_A::APORT4Y),
            13 => Val(APORTSEL_A::APORT4YX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0X`"]
    #[inline(always)]
    pub fn is_aport0x(&self) -> bool {
        *self == APORTSEL_A::APORT0X
    }
    #[doc = "Checks if the value of the field is `APORT0Y`"]
    #[inline(always)]
    pub fn is_aport0y(&self) -> bool {
        *self == APORTSEL_A::APORT0Y
    }
    #[doc = "Checks if the value of the field is `APORT1X`"]
    #[inline(always)]
    pub fn is_aport1x(&self) -> bool {
        *self == APORTSEL_A::APORT1X
    }
    #[doc = "Checks if the value of the field is `APORT1Y`"]
    #[inline(always)]
    pub fn is_aport1y(&self) -> bool {
        *self == APORTSEL_A::APORT1Y
    }
    #[doc = "Checks if the value of the field is `APORT1XY`"]
    #[inline(always)]
    pub fn is_aport1xy(&self) -> bool {
        *self == APORTSEL_A::APORT1XY
    }
    #[doc = "Checks if the value of the field is `APORT2X`"]
    #[inline(always)]
    pub fn is_aport2x(&self) -> bool {
        *self == APORTSEL_A::APORT2X
    }
    #[doc = "Checks if the value of the field is `APORT2Y`"]
    #[inline(always)]
    pub fn is_aport2y(&self) -> bool {
        *self == APORTSEL_A::APORT2Y
    }
    #[doc = "Checks if the value of the field is `APORT2YX`"]
    #[inline(always)]
    pub fn is_aport2yx(&self) -> bool {
        *self == APORTSEL_A::APORT2YX
    }
    #[doc = "Checks if the value of the field is `APORT3X`"]
    #[inline(always)]
    pub fn is_aport3x(&self) -> bool {
        *self == APORTSEL_A::APORT3X
    }
    #[doc = "Checks if the value of the field is `APORT3Y`"]
    #[inline(always)]
    pub fn is_aport3y(&self) -> bool {
        *self == APORTSEL_A::APORT3Y
    }
    #[doc = "Checks if the value of the field is `APORT3XY`"]
    #[inline(always)]
    pub fn is_aport3xy(&self) -> bool {
        *self == APORTSEL_A::APORT3XY
    }
    #[doc = "Checks if the value of the field is `APORT4X`"]
    #[inline(always)]
    pub fn is_aport4x(&self) -> bool {
        *self == APORTSEL_A::APORT4X
    }
    #[doc = "Checks if the value of the field is `APORT4Y`"]
    #[inline(always)]
    pub fn is_aport4y(&self) -> bool {
        *self == APORTSEL_A::APORT4Y
    }
    #[doc = "Checks if the value of the field is `APORT4YX`"]
    #[inline(always)]
    pub fn is_aport4yx(&self) -> bool {
        *self == APORTSEL_A::APORT4YX
    }
}
#[doc = "Write proxy for field `APORTSEL`"]
pub struct APORTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APORTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    #[inline(always)]
    pub fn aport0x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT0X)
    }
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    #[inline(always)]
    pub fn aport0y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT0Y)
    }
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1X)
    }
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1Y)
    }
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1xy(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1XY)
    }
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2X)
    }
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2Y)
    }
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2yx(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2YX)
    }
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3X)
    }
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3Y)
    }
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3xy(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3XY)
    }
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4X)
    }
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4Y)
    }
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4yx(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4YX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&self) -> APORTSEL_R {
        APORTSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&mut self) -> APORTSEL_W {
        APORTSEL_W { w: self }
    }
}
