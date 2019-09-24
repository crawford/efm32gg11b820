#[doc = "Reader of register ADCCTRL"]
pub type R = crate::R<u32, super::ADCCTRL>;
#[doc = "Writer for register ADCCTRL"]
pub type W = crate::W<u32, super::ADCCTRL>;
#[doc = "Register ADCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC0 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0CLKDIV_A {
    #[doc = "0: `0`"]
    NODIVISION,
}
impl From<ADC0CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKDIV_A) -> Self {
        match variant {
            ADC0CLKDIV_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `ADC0CLKDIV`"]
pub type ADC0CLKDIV_R = crate::R<u8, ADC0CLKDIV_A>;
impl ADC0CLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC0CLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC0CLKDIV_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC0CLKDIV_A::NODIVISION
    }
}
#[doc = "Write proxy for field `ADC0CLKDIV`"]
pub struct ADC0CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0CLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0CLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(ADC0CLKDIV_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0CLKSEL_A {
    #[doc = "0: ADC0 is not clocked"]
    DISABLED,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    AUXHFRCO,
    #[doc = "2: HFXO is clocking ADC0"]
    HFXO,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    HFSRCCLK,
}
impl From<ADC0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKSEL_A) -> Self {
        match variant {
            ADC0CLKSEL_A::DISABLED => 0,
            ADC0CLKSEL_A::AUXHFRCO => 1,
            ADC0CLKSEL_A::HFXO => 2,
            ADC0CLKSEL_A::HFSRCCLK => 3,
        }
    }
}
#[doc = "Reader of field `ADC0CLKSEL`"]
pub type ADC0CLKSEL_R = crate::R<u8, ADC0CLKSEL_A>;
impl ADC0CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0CLKSEL_A {
        match self.bits {
            0 => ADC0CLKSEL_A::DISABLED,
            1 => ADC0CLKSEL_A::AUXHFRCO,
            2 => ADC0CLKSEL_A::HFXO,
            3 => ADC0CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Write proxy for field `ADC0CLKSEL`"]
pub struct ADC0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::HFSRCCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC0CLKINV`"]
pub type ADC0CLKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0CLKINV`"]
pub struct ADC0CLKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0CLKINV_W<'a> {
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
#[doc = "ADC1 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1CLKDIV_A {
    #[doc = "0: `0`"]
    NODIVISION,
}
impl From<ADC1CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1CLKDIV_A) -> Self {
        match variant {
            ADC1CLKDIV_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `ADC1CLKDIV`"]
pub type ADC1CLKDIV_R = crate::R<u8, ADC1CLKDIV_A>;
impl ADC1CLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC1CLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC1CLKDIV_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC1CLKDIV_A::NODIVISION
    }
}
#[doc = "Write proxy for field `ADC1CLKDIV`"]
pub struct ADC1CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1CLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1CLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(ADC1CLKDIV_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "ADC1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1CLKSEL_A {
    #[doc = "0: ADC1 is not clocked"]
    DISABLED,
    #[doc = "1: AUXHFRCO is clocking ADC1"]
    AUXHFRCO,
    #[doc = "2: HFXO is clocking ADC1"]
    HFXO,
    #[doc = "3: HFSRCCLK is clocking ADC1"]
    HFSRCCLK,
}
impl From<ADC1CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1CLKSEL_A) -> Self {
        match variant {
            ADC1CLKSEL_A::DISABLED => 0,
            ADC1CLKSEL_A::AUXHFRCO => 1,
            ADC1CLKSEL_A::HFXO => 2,
            ADC1CLKSEL_A::HFSRCCLK => 3,
        }
    }
}
#[doc = "Reader of field `ADC1CLKSEL`"]
pub type ADC1CLKSEL_R = crate::R<u8, ADC1CLKSEL_A>;
impl ADC1CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1CLKSEL_A {
        match self.bits {
            0 => ADC1CLKSEL_A::DISABLED,
            1 => ADC1CLKSEL_A::AUXHFRCO,
            2 => ADC1CLKSEL_A::HFXO,
            3 => ADC1CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC1CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC1CLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC1CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC1CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Write proxy for field `ADC1CLKSEL`"]
pub struct ADC1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC1 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC1CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC1CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC1CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC1CLKSEL_A::HFSRCCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `ADC1CLKINV`"]
pub type ADC1CLKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1CLKINV`"]
pub struct ADC1CLKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1CLKINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&self) -> ADC0CLKDIV_R {
        ADC0CLKDIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> ADC0CLKSEL_R {
        ADC0CLKSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> ADC0CLKINV_R {
        ADC0CLKINV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    pub fn adc1clkdiv(&self) -> ADC1CLKDIV_R {
        ADC1CLKDIV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    pub fn adc1clksel(&self) -> ADC1CLKSEL_R {
        ADC1CLKSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    pub fn adc1clkinv(&self) -> ADC1CLKINV_R {
        ADC1CLKINV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&mut self) -> ADC0CLKDIV_W {
        ADC0CLKDIV_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&mut self) -> ADC0CLKSEL_W {
        ADC0CLKSEL_W { w: self }
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&mut self) -> ADC0CLKINV_W {
        ADC0CLKINV_W { w: self }
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline(always)]
    pub fn adc1clkdiv(&mut self) -> ADC1CLKDIV_W {
        ADC1CLKDIV_W { w: self }
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline(always)]
    pub fn adc1clksel(&mut self) -> ADC1CLKSEL_W {
        ADC1CLKSEL_W { w: self }
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline(always)]
    pub fn adc1clkinv(&mut self) -> ADC1CLKINV_W {
        ADC1CLKINV_W { w: self }
    }
}
