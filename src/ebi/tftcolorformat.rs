#[doc = "Reader of register TFTCOLORFORMAT"]
pub type R = crate::R<u32, super::TFTCOLORFORMAT>;
#[doc = "Writer for register TFTCOLORFORMAT"]
pub type W = crate::W<u32, super::TFTCOLORFORMAT>;
#[doc = "Register TFTCOLORFORMAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTCOLORFORMAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `PIXEL0FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL0FORMAT_A {
    #[doc = "ARGB data is 0555"]
    ARGB0555,
    #[doc = "ARGB data is 0565"]
    ARGB0565,
    #[doc = "ARGB data is 0666"]
    ARGB0666,
    #[doc = "ARGB data is 0888"]
    ARGB0888,
    #[doc = "ARGB data is 5555"]
    ARGB5555,
    #[doc = "ARGB data is 6565"]
    ARGB6565,
    #[doc = "ARGB data is 6666"]
    ARGB6666,
    #[doc = "ARGB data is 8888"]
    ARGB8888,
}
impl crate::ToBits<u8> for PIXEL0FORMAT_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PIXEL0FORMAT_A::ARGB0555 => 0,
            PIXEL0FORMAT_A::ARGB0565 => 1,
            PIXEL0FORMAT_A::ARGB0666 => 2,
            PIXEL0FORMAT_A::ARGB0888 => 3,
            PIXEL0FORMAT_A::ARGB5555 => 4,
            PIXEL0FORMAT_A::ARGB6565 => 5,
            PIXEL0FORMAT_A::ARGB6666 => 6,
            PIXEL0FORMAT_A::ARGB8888 => 7,
        }
    }
}
#[doc = "Reader of field `PIXEL0FORMAT`"]
pub type PIXEL0FORMAT_R = crate::R<u8, PIXEL0FORMAT_A>;
impl PIXEL0FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL0FORMAT_A {
        match self.bits {
            0 => PIXEL0FORMAT_A::ARGB0555,
            1 => PIXEL0FORMAT_A::ARGB0565,
            2 => PIXEL0FORMAT_A::ARGB0666,
            3 => PIXEL0FORMAT_A::ARGB0888,
            4 => PIXEL0FORMAT_A::ARGB5555,
            5 => PIXEL0FORMAT_A::ARGB6565,
            6 => PIXEL0FORMAT_A::ARGB6666,
            7 => PIXEL0FORMAT_A::ARGB8888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB0555`"]
    #[inline(always)]
    pub fn is_argb0555(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0555
    }
    #[doc = "Checks if the value of the field is `ARGB0565`"]
    #[inline(always)]
    pub fn is_argb0565(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0565
    }
    #[doc = "Checks if the value of the field is `ARGB0666`"]
    #[inline(always)]
    pub fn is_argb0666(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0666
    }
    #[doc = "Checks if the value of the field is `ARGB0888`"]
    #[inline(always)]
    pub fn is_argb0888(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0888
    }
    #[doc = "Checks if the value of the field is `ARGB5555`"]
    #[inline(always)]
    pub fn is_argb5555(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB5555
    }
    #[doc = "Checks if the value of the field is `ARGB6565`"]
    #[inline(always)]
    pub fn is_argb6565(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB6565
    }
    #[doc = "Checks if the value of the field is `ARGB6666`"]
    #[inline(always)]
    pub fn is_argb6666(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB6666
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB8888
    }
}
#[doc = "Write proxy for field `PIXEL0FORMAT`"]
pub struct PIXEL0FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL0FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL0FORMAT_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ARGB data is 0555"]
    #[inline(always)]
    pub fn argb0555(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0555)
    }
    #[doc = "ARGB data is 0565"]
    #[inline(always)]
    pub fn argb0565(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0565)
    }
    #[doc = "ARGB data is 0666"]
    #[inline(always)]
    pub fn argb0666(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0666)
    }
    #[doc = "ARGB data is 0888"]
    #[inline(always)]
    pub fn argb0888(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0888)
    }
    #[doc = "ARGB data is 5555"]
    #[inline(always)]
    pub fn argb5555(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB5555)
    }
    #[doc = "ARGB data is 6565"]
    #[inline(always)]
    pub fn argb6565(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB6565)
    }
    #[doc = "ARGB data is 6666"]
    #[inline(always)]
    pub fn argb6666(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB6666)
    }
    #[doc = "ARGB data is 8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB8888)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `PIXEL1FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL1FORMAT_A {
    #[doc = "RGB data is 555"]
    RGB555,
    #[doc = "RGB data is 565"]
    RGB565,
    #[doc = "RGB data is 666"]
    RGB666,
    #[doc = "RGB data is 888"]
    RGB888,
}
impl crate::ToBits<u8> for PIXEL1FORMAT_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PIXEL1FORMAT_A::RGB555 => 0,
            PIXEL1FORMAT_A::RGB565 => 1,
            PIXEL1FORMAT_A::RGB666 => 2,
            PIXEL1FORMAT_A::RGB888 => 3,
        }
    }
}
#[doc = "Reader of field `PIXEL1FORMAT`"]
pub type PIXEL1FORMAT_R = crate::R<u8, PIXEL1FORMAT_A>;
impl PIXEL1FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL1FORMAT_A {
        match self.bits {
            0 => PIXEL1FORMAT_A::RGB555,
            1 => PIXEL1FORMAT_A::RGB565,
            2 => PIXEL1FORMAT_A::RGB666,
            3 => PIXEL1FORMAT_A::RGB888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB666`"]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB666
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB888
    }
}
#[doc = "Write proxy for field `PIXEL1FORMAT`"]
pub struct PIXEL1FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL1FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL1FORMAT_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RGB data is 555"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB555)
    }
    #[doc = "RGB data is 565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB565)
    }
    #[doc = "RGB data is 666"]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB666)
    }
    #[doc = "RGB data is 888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB888)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&self) -> PIXEL0FORMAT_R {
        PIXEL0FORMAT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&self) -> PIXEL1FORMAT_R {
        PIXEL1FORMAT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&mut self) -> PIXEL0FORMAT_W {
        PIXEL0FORMAT_W { w: self }
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&mut self) -> PIXEL1FORMAT_W {
        PIXEL1FORMAT_W { w: self }
    }
}
