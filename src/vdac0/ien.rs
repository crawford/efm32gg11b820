#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CD`"]
pub type CH0CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CD`"]
pub struct CH0CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CD_W<'a> {
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
#[doc = "Reader of field `CH1CD`"]
pub type CH1CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CD`"]
pub struct CH1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CD_W<'a> {
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
#[doc = "Reader of field `CH0OF`"]
pub type CH0OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OF`"]
pub struct CH0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OF_W<'a> {
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
#[doc = "Reader of field `CH1OF`"]
pub type CH1OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OF`"]
pub struct CH1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OF_W<'a> {
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
#[doc = "Reader of field `CH0UF`"]
pub type CH0UF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0UF`"]
pub struct CH0UF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0UF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH1UF`"]
pub type CH1UF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1UF`"]
pub struct CH1UF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1UF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CH0BL`"]
pub type CH0BL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0BL`"]
pub struct CH0BL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0BL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CH1BL`"]
pub type CH1BL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1BL`"]
pub struct CH1BL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1BL_W<'a> {
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
#[doc = "Reader of field `EM23ERR`"]
pub type EM23ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM23ERR`"]
pub struct EM23ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `OPA0APORTCONFLICT`"]
pub type OPA0APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA0APORTCONFLICT`"]
pub struct OPA0APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0APORTCONFLICT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OPA1APORTCONFLICT`"]
pub type OPA1APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1APORTCONFLICT`"]
pub struct OPA1APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1APORTCONFLICT_W<'a> {
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
#[doc = "Reader of field `OPA2APORTCONFLICT`"]
pub type OPA2APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2APORTCONFLICT`"]
pub struct OPA2APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2APORTCONFLICT_W<'a> {
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
#[doc = "Reader of field `OPA3APORTCONFLICT`"]
pub type OPA3APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3APORTCONFLICT`"]
pub struct OPA3APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3APORTCONFLICT_W<'a> {
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
#[doc = "Reader of field `OPA0PRSTIMEDERR`"]
pub type OPA0PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA0PRSTIMEDERR`"]
pub struct OPA0PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0PRSTIMEDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `OPA1PRSTIMEDERR`"]
pub type OPA1PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1PRSTIMEDERR`"]
pub struct OPA1PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1PRSTIMEDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `OPA2PRSTIMEDERR`"]
pub type OPA2PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2PRSTIMEDERR`"]
pub struct OPA2PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2PRSTIMEDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `OPA3PRSTIMEDERR`"]
pub type OPA3PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3PRSTIMEDERR`"]
pub struct OPA3PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3PRSTIMEDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `OPA0OUTVALID`"]
pub type OPA0OUTVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA0OUTVALID`"]
pub struct OPA0OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0OUTVALID_W<'a> {
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
#[doc = "Reader of field `OPA1OUTVALID`"]
pub type OPA1OUTVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1OUTVALID`"]
pub struct OPA1OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `OPA2OUTVALID`"]
pub type OPA2OUTVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2OUTVALID`"]
pub struct OPA2OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `OPA3OUTVALID`"]
pub type OPA3OUTVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3OUTVALID`"]
pub struct OPA3OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> OPA0PRSTIMEDERR_R {
        OPA0PRSTIMEDERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> OPA1PRSTIMEDERR_R {
        OPA1PRSTIMEDERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> OPA2PRSTIMEDERR_R {
        OPA2PRSTIMEDERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> OPA3PRSTIMEDERR_R {
        OPA3PRSTIMEDERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> CH0CD_W {
        CH0CD_W { w: self }
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> CH1CD_W {
        CH1CD_W { w: self }
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W { w: self }
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W { w: self }
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> CH0UF_W {
        CH0UF_W { w: self }
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> CH1UF_W {
        CH1UF_W { w: self }
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&mut self) -> CH0BL_W {
        CH0BL_W { w: self }
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&mut self) -> CH1BL_W {
        CH1BL_W { w: self }
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W { w: self }
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W {
        OPA0APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W {
        OPA1APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W {
        OPA2APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W {
        OPA3APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W {
        OPA0PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W {
        OPA1PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W {
        OPA2PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W {
        OPA3PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W {
        OPA0OUTVALID_W { w: self }
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W {
        OPA1OUTVALID_W { w: self }
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W {
        OPA2OUTVALID_W { w: self }
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W {
        OPA3OUTVALID_W { w: self }
    }
}
