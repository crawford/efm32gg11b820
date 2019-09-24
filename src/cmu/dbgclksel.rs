#[doc = "Reader of register DBGCLKSEL"]
pub type R = crate::R<u32, super::DBGCLKSEL>;
#[doc = "Writer for register DBGCLKSEL"]
pub type W = crate::W<u32, super::DBGCLKSEL>;
#[doc = "Register DBGCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK,
    #[doc = "2: HFRCO divided by 2 is the debug trace clock"]
    HFRCODIV2,
}
impl From<DBG_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        match variant {
            DBG_A::AUXHFRCO => 0,
            DBG_A::HFCLK => 1,
            DBG_A::HFRCODIV2 => 2,
        }
    }
}
#[doc = "Reader of field `DBG`"]
pub type DBG_R = crate::R<u8, DBG_A>;
impl DBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBG_A::AUXHFRCO),
            1 => Val(DBG_A::HFCLK),
            2 => Val(DBG_A::HFRCODIV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == DBG_A::HFRCODIV2
    }
}
#[doc = "Write proxy for field `DBG`"]
pub struct DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DBG_A::HFCLK)
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut W {
        self.variant(DBG_A::HFRCODIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W { w: self }
    }
}
