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
#[doc = "Reader of field `VMONAVDDFALL`"]
pub type VMONAVDDFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONAVDDFALL`"]
pub struct VMONAVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONAVDDFALL_W<'a> {
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
#[doc = "Reader of field `VMONAVDDRISE`"]
pub type VMONAVDDRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONAVDDRISE`"]
pub struct VMONAVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONAVDDRISE_W<'a> {
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
#[doc = "Reader of field `VMONALTAVDDFALL`"]
pub type VMONALTAVDDFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONALTAVDDFALL`"]
pub struct VMONALTAVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONALTAVDDFALL_W<'a> {
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
#[doc = "Reader of field `VMONALTAVDDRISE`"]
pub type VMONALTAVDDRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONALTAVDDRISE`"]
pub struct VMONALTAVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONALTAVDDRISE_W<'a> {
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
#[doc = "Reader of field `VMONDVDDFALL`"]
pub type VMONDVDDFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONDVDDFALL`"]
pub struct VMONDVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONDVDDFALL_W<'a> {
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
#[doc = "Reader of field `VMONDVDDRISE`"]
pub type VMONDVDDRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONDVDDRISE`"]
pub struct VMONDVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONDVDDRISE_W<'a> {
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
#[doc = "Reader of field `VMONIO0FALL`"]
pub type VMONIO0FALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONIO0FALL`"]
pub struct VMONIO0FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO0FALL_W<'a> {
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
#[doc = "Reader of field `VMONIO0RISE`"]
pub type VMONIO0RISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONIO0RISE`"]
pub struct VMONIO0RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO0RISE_W<'a> {
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
#[doc = "Reader of field `VMONIO1FALL`"]
pub type VMONIO1FALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONIO1FALL`"]
pub struct VMONIO1FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO1FALL_W<'a> {
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
#[doc = "Reader of field `VMONIO1RISE`"]
pub type VMONIO1RISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONIO1RISE`"]
pub struct VMONIO1RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO1RISE_W<'a> {
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
#[doc = "Reader of field `R5VREADY`"]
pub type R5VREADY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5VREADY`"]
pub struct R5VREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> R5VREADY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `VMONBUVDDFALL`"]
pub type VMONBUVDDFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONBUVDDFALL`"]
pub struct VMONBUVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONBUVDDFALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `VMONBUVDDRISE`"]
pub type VMONBUVDDRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMONBUVDDRISE`"]
pub struct VMONBUVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONBUVDDRISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PFETOVERCURRENTLIMIT`"]
pub type PFETOVERCURRENTLIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFETOVERCURRENTLIMIT`"]
pub struct PFETOVERCURRENTLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PFETOVERCURRENTLIMIT_W<'a> {
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
#[doc = "Reader of field `NFETOVERCURRENTLIMIT`"]
pub type NFETOVERCURRENTLIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NFETOVERCURRENTLIMIT`"]
pub struct NFETOVERCURRENTLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NFETOVERCURRENTLIMIT_W<'a> {
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
#[doc = "Reader of field `DCDCLPRUNNING`"]
pub type DCDCLPRUNNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDCLPRUNNING`"]
pub struct DCDCLPRUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCLPRUNNING_W<'a> {
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
#[doc = "Reader of field `DCDCLNRUNNING`"]
pub type DCDCLNRUNNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDCLNRUNNING`"]
pub struct DCDCLNRUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCLNRUNNING_W<'a> {
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
#[doc = "Reader of field `DCDCINBYPASS`"]
pub type DCDCINBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDCINBYPASS`"]
pub struct DCDCINBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCINBYPASS_W<'a> {
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
#[doc = "Reader of field `BURDY`"]
pub type BURDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURDY`"]
pub struct BURDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BURDY_W<'a> {
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
#[doc = "Reader of field `R5VVSINT`"]
pub type R5VVSINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5VVSINT`"]
pub struct R5VVSINT_W<'a> {
    w: &'a mut W,
}
impl<'a> R5VVSINT_W<'a> {
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
#[doc = "Reader of field `EM23WAKEUP`"]
pub type EM23WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM23WAKEUP`"]
pub struct EM23WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23WAKEUP_W<'a> {
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
#[doc = "Reader of field `VSCALEDONE`"]
pub type VSCALEDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSCALEDONE`"]
pub struct VSCALEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSCALEDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP`"]
pub struct TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_W<'a> {
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
#[doc = "Reader of field `TEMPLOW`"]
pub type TEMPLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMPLOW`"]
pub struct TEMPLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPLOW_W<'a> {
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
#[doc = "Reader of field `TEMPHIGH`"]
pub type TEMPHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMPHIGH`"]
pub struct TEMPHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPHIGH_W<'a> {
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
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1fall(&self) -> VMONIO1FALL_R {
        VMONIO1FALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1rise(&self) -> VMONIO1RISE_R {
        VMONIO1RISE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    pub fn r5vready(&self) -> R5VREADY_R {
        R5VREADY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VMONBUVDDFALL_R {
        VMONBUVDDFALL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VMONBUVDDRISE_R {
        VMONBUVDDRISE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    pub fn r5vvsint(&self) -> R5VVSINT_R {
        R5VVSINT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W {
        VMONAVDDFALL_W { w: self }
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W {
        VMONAVDDRISE_W { w: self }
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W {
        VMONALTAVDDFALL_W { w: self }
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W {
        VMONALTAVDDRISE_W { w: self }
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W {
        VMONDVDDFALL_W { w: self }
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W {
        VMONDVDDRISE_W { w: self }
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W {
        VMONIO0FALL_W { w: self }
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W {
        VMONIO0RISE_W { w: self }
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1fall(&mut self) -> VMONIO1FALL_W {
        VMONIO1FALL_W { w: self }
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1rise(&mut self) -> VMONIO1RISE_W {
        VMONIO1RISE_W { w: self }
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    pub fn r5vready(&mut self) -> R5VREADY_W {
        R5VREADY_W { w: self }
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W {
        VMONBUVDDFALL_W { w: self }
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W {
        VMONBUVDDRISE_W { w: self }
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W {
        PFETOVERCURRENTLIMIT_W { w: self }
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W {
        NFETOVERCURRENTLIMIT_W { w: self }
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W {
        DCDCLPRUNNING_W { w: self }
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W {
        DCDCLNRUNNING_W { w: self }
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W {
        DCDCINBYPASS_W { w: self }
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BURDY_W {
        BURDY_W { w: self }
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    pub fn r5vvsint(&mut self) -> R5VVSINT_W {
        R5VVSINT_W { w: self }
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W {
        EM23WAKEUP_W { w: self }
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W {
        VSCALEDONE_W { w: self }
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W { w: self }
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W { w: self }
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
        TEMPHIGH_W { w: self }
    }
}
