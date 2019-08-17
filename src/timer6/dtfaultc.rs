#[doc = "Writer for register DTFAULTC"]
pub type W = crate::W<u32, super::DTFAULTC>;
#[doc = "Register DTFAULTC `reset()`'s with value 0"]
impl crate::ResetValue for super::DTFAULTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DTPRS0FC`"]
pub struct DTPRS0FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FC_W<'a> {
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
#[doc = "Write proxy for field `DTPRS1FC`"]
pub struct DTPRS1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FC_W<'a> {
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
#[doc = "Write proxy for field `DTDBGFC`"]
pub struct DTDBGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDBGFC_W<'a> {
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
#[doc = "Write proxy for field `TLOCKUPFC`"]
pub struct TLOCKUPFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOCKUPFC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    pub fn dtprs0fc(&mut self) -> DTPRS0FC_W {
        DTPRS0FC_W { w: self }
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    pub fn dtprs1fc(&mut self) -> DTPRS1FC_W {
        DTPRS1FC_W { w: self }
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    pub fn dtdbgfc(&mut self) -> DTDBGFC_W {
        DTDBGFC_W { w: self }
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    pub fn tlockupfc(&mut self) -> TLOCKUPFC_W {
        TLOCKUPFC_W { w: self }
    }
}
