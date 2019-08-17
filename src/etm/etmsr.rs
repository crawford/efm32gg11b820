#[doc = "Reader of register ETMSR"]
pub type R = crate::R<u32, super::ETMSR>;
#[doc = "Writer for register ETMSR"]
pub type W = crate::W<u32, super::ETMSR>;
#[doc = "Register ETMSR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::ETMSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `ETHOF`"]
pub type ETHOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ETMPROGBIT`"]
pub type ETMPROGBIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRACESTAT`"]
pub type TRACESTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACESTAT`"]
pub struct TRACESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACESTAT_W<'a> {
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
#[doc = "Reader of field `TRIGBIT`"]
pub type TRIGBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIGBIT`"]
pub struct TRIGBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGBIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETM Overflow"]
    #[inline(always)]
    pub fn ethof(&self) -> ETHOF_R {
        ETHOF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ETM Programming Bit Status"]
    #[inline(always)]
    pub fn etmprogbit(&self) -> ETMPROGBIT_R {
        ETMPROGBIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&self) -> TRACESTAT_R {
        TRACESTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&self) -> TRIGBIT_R {
        TRIGBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&mut self) -> TRACESTAT_W {
        TRACESTAT_W { w: self }
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&mut self) -> TRIGBIT_W {
        TRIGBIT_W { w: self }
    }
}
