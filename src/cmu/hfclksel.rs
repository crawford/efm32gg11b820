#[doc = "Writer for register HFCLKSEL"]
pub type W = crate::W<u32, super::HFCLKSEL>;
#[doc = "Register HFCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HFCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `HF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HF_AW {
    #[doc = "Select HFRCO as HFCLK"]
    HFRCO,
    #[doc = "Select HFXO as HFCLK"]
    HFXO,
    #[doc = "Select LFRCO as HFCLK"]
    LFRCO,
    #[doc = "Select LFXO as HFCLK"]
    LFXO,
    #[doc = "Select HFRCO divided by 2 as HFCLK"]
    HFRCODIV2,
    #[doc = "Select USHFRCO as HFCLK"]
    USHFRCO,
    #[doc = "Select CLKIN0 as HFCLK"]
    CLKIN0,
}
impl crate::ToBits<u8> for HF_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            HF_AW::HFRCO => 1,
            HF_AW::HFXO => 2,
            HF_AW::LFRCO => 3,
            HF_AW::LFXO => 4,
            HF_AW::HFRCODIV2 => 5,
            HF_AW::USHFRCO => 6,
            HF_AW::CLKIN0 => 7,
        }
    }
}
#[doc = "Write proxy for field `HF`"]
pub struct HF_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HF_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFRCO as HFCLK"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HF_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HF_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HF_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HF_AW::LFXO)
    }
    #[doc = "Select HFRCO divided by 2 as HFCLK"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut W {
        self.variant(HF_AW::HFRCODIV2)
    }
    #[doc = "Select USHFRCO as HFCLK"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(HF_AW::USHFRCO)
    }
    #[doc = "Select CLKIN0 as HFCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(HF_AW::CLKIN0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hf(&mut self) -> HF_W {
        HF_W { w: self }
    }
}
