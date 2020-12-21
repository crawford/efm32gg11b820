#[doc = "Reader of register NANDCTRL"]
pub type R = crate::R<u32, super::NANDCTRL>;
#[doc = "Writer for register NANDCTRL"]
pub type W = crate::W<u32, super::NANDCTRL>;
#[doc = "Register NANDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::NANDCTRL {
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
#[doc = "NAND Flash Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BANKSEL_A {
    #[doc = "0: Memory bank 0 is connected to a NAND Flash device."]
    BANK0 = 0,
    #[doc = "1: Memory bank 1 is connected to a NAND Flash device."]
    BANK1 = 1,
    #[doc = "2: Memory bank 2 is connected to a NAND Flash device."]
    BANK2 = 2,
    #[doc = "3: Memory bank 3 is connected to a NAND Flash device."]
    BANK3 = 3,
}
impl From<BANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BANKSEL`"]
pub type BANKSEL_R = crate::R<u8, BANKSEL_A>;
impl BANKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANKSEL_A {
        match self.bits {
            0 => BANKSEL_A::BANK0,
            1 => BANKSEL_A::BANK1,
            2 => BANKSEL_A::BANK2,
            3 => BANKSEL_A::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
}
#[doc = "Write proxy for field `BANKSEL`"]
pub struct BANKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK0)
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&mut self) -> BANKSEL_W {
        BANKSEL_W { w: self }
    }
}
