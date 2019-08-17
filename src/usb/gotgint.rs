#[doc = "Reader of register GOTGINT"]
pub type R = crate::R<u32, super::GOTGINT>;
#[doc = "Writer for register GOTGINT"]
pub type W = crate::W<u32, super::GOTGINT>;
#[doc = "Register GOTGINT `reset()`'s with value 0"]
impl crate::ResetValue for super::GOTGINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SESENDDET`"]
pub type SESENDDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESENDDET`"]
pub struct SESENDDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDDET_W<'a> {
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
#[doc = "Reader of field `SESREQSUCSTSCHNG`"]
pub type SESREQSUCSTSCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESREQSUCSTSCHNG`"]
pub struct SESREQSUCSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> SESREQSUCSTSCHNG_W<'a> {
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
#[doc = "Reader of field `HSTNEGSUCSTSCHNG`"]
pub type HSTNEGSUCSTSCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTNEGSUCSTSCHNG`"]
pub struct HSTNEGSUCSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTNEGSUCSTSCHNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HSTNEGDET`"]
pub type HSTNEGDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTNEGDET`"]
pub struct HSTNEGDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTNEGDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ADEVTOUTCHG`"]
pub type ADEVTOUTCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEVTOUTCHG`"]
pub struct ADEVTOUTCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEVTOUTCHG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DBNCEDONE`"]
pub type DBNCEDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBNCEDONE`"]
pub struct DBNCEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBNCEDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbncedone(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session End Detected"]
    #[inline(always)]
    pub fn sesenddet(&mut self) -> SESENDDET_W {
        SESENDDET_W { w: self }
    }
    #[doc = "Bit 8 - Session Request Success Status Change"]
    #[inline(always)]
    pub fn sesreqsucstschng(&mut self) -> SESREQSUCSTSCHNG_W {
        SESREQSUCSTSCHNG_W { w: self }
    }
    #[doc = "Bit 9 - Host Negotiation Success Status Change"]
    #[inline(always)]
    pub fn hstnegsucstschng(&mut self) -> HSTNEGSUCSTSCHNG_W {
        HSTNEGSUCSTSCHNG_W { w: self }
    }
    #[doc = "Bit 17 - Host Negotiation Detected"]
    #[inline(always)]
    pub fn hstnegdet(&mut self) -> HSTNEGDET_W {
        HSTNEGDET_W { w: self }
    }
    #[doc = "Bit 18 - A-Device Timeout Change"]
    #[inline(always)]
    pub fn adevtoutchg(&mut self) -> ADEVTOUTCHG_W {
        ADEVTOUTCHG_W { w: self }
    }
    #[doc = "Bit 19 - Debounce Done"]
    #[inline(always)]
    pub fn dbncedone(&mut self) -> DBNCEDONE_W {
        DBNCEDONE_W { w: self }
    }
}
