#[doc = "Reader of register BACTRL"]
pub type R = crate::R<u32, super::BACTRL>;
#[doc = "Writer for register BACTRL"]
pub type W = crate::W<u32, super::BACTRL>;
#[doc = "Register BACTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLINKEN`"]
pub type BLINKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLINKEN`"]
pub struct BLINKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINKEN_W<'a> {
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
#[doc = "Reader of field `BLANK`"]
pub type BLANK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLANK`"]
pub struct BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AEN`"]
pub type AEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEN`"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `AREGASC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREGASC_A {
    #[doc = "No Shift operation on Animation Register A"]
    NOSHIFT,
    #[doc = "Animation Register A is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register A is shifted right"]
    SHIFTRIGHT,
}
impl crate::ToBits<u8> for AREGASC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            AREGASC_A::NOSHIFT => 0,
            AREGASC_A::SHIFTLEFT => 1,
            AREGASC_A::SHIFTRIGHT => 2,
        }
    }
}
#[doc = "Reader of field `AREGASC`"]
pub type AREGASC_R = crate::R<u8, AREGASC_A>;
impl AREGASC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AREGASC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AREGASC_A::NOSHIFT),
            1 => Val(AREGASC_A::SHIFTLEFT),
            2 => Val(AREGASC_A::SHIFTRIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == AREGASC_A::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFTLEFT`"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGASC_A::SHIFTLEFT
    }
    #[doc = "Checks if the value of the field is `SHIFTRIGHT`"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGASC_A::SHIFTRIGHT
    }
}
#[doc = "Write proxy for field `AREGASC`"]
pub struct AREGASC_W<'a> {
    w: &'a mut W,
}
impl<'a> AREGASC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AREGASC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut W {
        self.variant(AREGASC_A::NOSHIFT)
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut W {
        self.variant(AREGASC_A::SHIFTLEFT)
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut W {
        self.variant(AREGASC_A::SHIFTRIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `AREGBSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREGBSC_A {
    #[doc = "No Shift operation on Animation Register B"]
    NOSHIFT,
    #[doc = "Animation Register B is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register B is shifted right"]
    SHIFTRIGHT,
}
impl crate::ToBits<u8> for AREGBSC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            AREGBSC_A::NOSHIFT => 0,
            AREGBSC_A::SHIFTLEFT => 1,
            AREGBSC_A::SHIFTRIGHT => 2,
        }
    }
}
#[doc = "Reader of field `AREGBSC`"]
pub type AREGBSC_R = crate::R<u8, AREGBSC_A>;
impl AREGBSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AREGBSC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AREGBSC_A::NOSHIFT),
            1 => Val(AREGBSC_A::SHIFTLEFT),
            2 => Val(AREGBSC_A::SHIFTRIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == AREGBSC_A::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFTLEFT`"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGBSC_A::SHIFTLEFT
    }
    #[doc = "Checks if the value of the field is `SHIFTRIGHT`"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGBSC_A::SHIFTRIGHT
    }
}
#[doc = "Write proxy for field `AREGBSC`"]
pub struct AREGBSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AREGBSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AREGBSC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut W {
        self.variant(AREGBSC_A::NOSHIFT)
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut W {
        self.variant(AREGBSC_A::SHIFTLEFT)
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut W {
        self.variant(AREGBSC_A::SHIFTRIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `ALOGSEL`"]
pub type ALOGSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALOGSEL`"]
pub struct ALOGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALOGSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FCEN`"]
pub type FCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCEN`"]
pub struct FCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `FCPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCPRESC_A {
    #[doc = "CLKFC = CLKFRAME / 1"]
    DIV1,
    #[doc = "CLKFC = CLKFRAME / 2"]
    DIV2,
    #[doc = "CLKFC = CLKFRAME / 4"]
    DIV4,
    #[doc = "CLKFC = CLKFRAME / 8"]
    DIV8,
}
impl crate::ToBits<u8> for FCPRESC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FCPRESC_A::DIV1 => 0,
            FCPRESC_A::DIV2 => 1,
            FCPRESC_A::DIV4 => 2,
            FCPRESC_A::DIV8 => 3,
        }
    }
}
#[doc = "Reader of field `FCPRESC`"]
pub type FCPRESC_R = crate::R<u8, FCPRESC_A>;
impl FCPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCPRESC_A {
        match self.bits {
            0 => FCPRESC_A::DIV1,
            1 => FCPRESC_A::DIV2,
            2 => FCPRESC_A::DIV4,
            3 => FCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FCPRESC_A::DIV8
    }
}
#[doc = "Write proxy for field `FCPRESC`"]
pub struct FCPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> FCPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCPRESC_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV1)
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV2)
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV4)
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `FCTOP`"]
pub type FCTOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCTOP`"]
pub struct FCTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FCTOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ALOC`"]
pub type ALOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALOC`"]
pub struct ALOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&self) -> BLINKEN_R {
        BLINKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    pub fn aregasc(&self) -> AREGASC_R {
        AREGASC_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&self) -> AREGBSC_R {
        AREGBSC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&self) -> ALOGSEL_R {
        ALOGSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&self) -> FCEN_R {
        FCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&self) -> FCPRESC_R {
        FCPRESC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    pub fn fctop(&self) -> FCTOP_R {
        FCTOP_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&self) -> ALOC_R {
        ALOC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&mut self) -> BLINKEN_W {
        BLINKEN_W { w: self }
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&mut self) -> BLANK_W {
        BLANK_W { w: self }
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bits 3:4 - Animate Register a Shift Control"]
    #[inline(always)]
    pub fn aregasc(&mut self) -> AREGASC_W {
        AREGASC_W { w: self }
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&mut self) -> AREGBSC_W {
        AREGBSC_W { w: self }
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&mut self) -> ALOGSEL_W {
        ALOGSEL_W { w: self }
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&mut self) -> FCEN_W {
        FCEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&mut self) -> FCPRESC_W {
        FCPRESC_W { w: self }
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline(always)]
    pub fn fctop(&mut self) -> FCTOP_W {
        FCTOP_W { w: self }
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> ALOC_W {
        ALOC_W { w: self }
    }
}
