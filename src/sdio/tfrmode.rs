#[doc = "Reader of register TFRMODE"]
pub type R = crate::R<u32, super::TFRMODE>;
#[doc = "Writer for register TFRMODE"]
pub type W = crate::W<u32, super::TFRMODE>;
#[doc = "Register TFRMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TFRMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Reader of field `BLKCNTEN`"]
pub type BLKCNTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKCNTEN`"]
pub struct BLKCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKCNTEN_W<'a> {
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
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUTOCMDEN_A {
    #[doc = "0: Auto CMD Disabled"]
    ACMDDISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    ACMD12EN = 1,
    #[doc = "2: Auto CMD23 Enable"]
    ACMD23EN = 2,
}
impl From<AUTOCMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTOCMDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUTOCMDEN`"]
pub type AUTOCMDEN_R = crate::R<u8, AUTOCMDEN_A>;
impl AUTOCMDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUTOCMDEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUTOCMDEN_A::ACMDDISABLED),
            1 => Val(AUTOCMDEN_A::ACMD12EN),
            2 => Val(AUTOCMDEN_A::ACMD23EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMDDISABLED`"]
    #[inline(always)]
    pub fn is_acmddisabled(&self) -> bool {
        *self == AUTOCMDEN_A::ACMDDISABLED
    }
    #[doc = "Checks if the value of the field is `ACMD12EN`"]
    #[inline(always)]
    pub fn is_acmd12en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD12EN
    }
    #[doc = "Checks if the value of the field is `ACMD23EN`"]
    #[inline(always)]
    pub fn is_acmd23en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD23EN
    }
}
#[doc = "Write proxy for field `AUTOCMDEN`"]
pub struct AUTOCMDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCMDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOCMDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn acmddisabled(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMDDISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn acmd12en(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMD12EN)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn acmd23en(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMD23EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DATDIRSEL`"]
pub type DATDIRSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATDIRSEL`"]
pub struct DATDIRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATDIRSEL_W<'a> {
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
#[doc = "Reader of field `MULTSINGBLKSEL`"]
pub type MULTSINGBLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MULTSINGBLKSEL`"]
pub struct MULTSINGBLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTSINGBLKSEL_W<'a> {
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
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESPTYPESEL_A {
    #[doc = "0: No RESP"]
    NORESP = 0,
    #[doc = "1: RESP Length 136"]
    RESP136 = 1,
    #[doc = "2: RESP Length 48"]
    RESP48 = 2,
    #[doc = "3: RESP Length 48 check busy after RESP"]
    BUSYAFTRESP = 3,
}
impl From<RESPTYPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPTYPESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESPTYPESEL`"]
pub type RESPTYPESEL_R = crate::R<u8, RESPTYPESEL_A>;
impl RESPTYPESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPTYPESEL_A {
        match self.bits {
            0 => RESPTYPESEL_A::NORESP,
            1 => RESPTYPESEL_A::RESP136,
            2 => RESPTYPESEL_A::RESP48,
            3 => RESPTYPESEL_A::BUSYAFTRESP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORESP`"]
    #[inline(always)]
    pub fn is_noresp(&self) -> bool {
        *self == RESPTYPESEL_A::NORESP
    }
    #[doc = "Checks if the value of the field is `RESP136`"]
    #[inline(always)]
    pub fn is_resp136(&self) -> bool {
        *self == RESPTYPESEL_A::RESP136
    }
    #[doc = "Checks if the value of the field is `RESP48`"]
    #[inline(always)]
    pub fn is_resp48(&self) -> bool {
        *self == RESPTYPESEL_A::RESP48
    }
    #[doc = "Checks if the value of the field is `BUSYAFTRESP`"]
    #[inline(always)]
    pub fn is_busyaftresp(&self) -> bool {
        *self == RESPTYPESEL_A::BUSYAFTRESP
    }
}
#[doc = "Write proxy for field `RESPTYPESEL`"]
pub struct RESPTYPESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPTYPESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESPTYPESEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::NORESP)
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn resp136(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::RESP136)
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn resp48(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::RESP48)
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn busyaftresp(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::BUSYAFTRESP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMDCRCCHKEN`"]
pub type CMDCRCCHKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCRCCHKEN`"]
pub struct CMDCRCCHKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCCHKEN_W<'a> {
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
#[doc = "Reader of field `CMDINDXCHKEN`"]
pub type CMDINDXCHKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDINDXCHKEN`"]
pub struct CMDINDXCHKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDXCHKEN_W<'a> {
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
#[doc = "Reader of field `DATPRESSEL`"]
pub type DATPRESSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATPRESSEL`"]
pub struct DATPRESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPRESSEL_W<'a> {
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
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDTYPE_A {
    #[doc = "0: Normal Command"]
    NORMAL = 0,
    #[doc = "1: Suspend command"]
    SUSPEND = 1,
    #[doc = "2: Resume command"]
    RESUME = 2,
    #[doc = "3: Abort command"]
    ABORT = 3,
}
impl From<CMDTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDTYPE`"]
pub type CMDTYPE_R = crate::R<u8, CMDTYPE_A>;
impl CMDTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYPE_A {
        match self.bits {
            0 => CMDTYPE_A::NORMAL,
            1 => CMDTYPE_A::SUSPEND,
            2 => CMDTYPE_A::RESUME,
            3 => CMDTYPE_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPE_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPE_A::ABORT
    }
}
#[doc = "Write proxy for field `CMDTYPE`"]
pub struct CMDTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPE_A::NORMAL)
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDTYPE_A::SUSPEND)
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDTYPE_A::RESUME)
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDTYPE_A::ABORT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CMDINDEX`"]
pub type CMDINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDINDEX`"]
pub struct CMDINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&self) -> BLKCNTEN_R {
        BLKCNTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&self) -> AUTOCMDEN_R {
        AUTOCMDEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&self) -> DATDIRSEL_R {
        DATDIRSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&self) -> MULTSINGBLKSEL_R {
        MULTSINGBLKSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> RESPTYPESEL_R {
        RESPTYPESEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&self) -> CMDCRCCHKEN_R {
        CMDCRCCHKEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&self) -> CMDINDXCHKEN_R {
        CMDINDXCHKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&self) -> DATPRESSEL_R {
        DATPRESSEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&mut self) -> BLKCNTEN_W {
        BLKCNTEN_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&mut self) -> AUTOCMDEN_W {
        AUTOCMDEN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&mut self) -> DATDIRSEL_W {
        DATDIRSEL_W { w: self }
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&mut self) -> MULTSINGBLKSEL_W {
        MULTSINGBLKSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&mut self) -> RESPTYPESEL_W {
        RESPTYPESEL_W { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&mut self) -> CMDCRCCHKEN_W {
        CMDCRCCHKEN_W { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&mut self) -> CMDINDXCHKEN_W {
        CMDINDXCHKEN_W { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&mut self) -> DATPRESSEL_W {
        DATPRESSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CMDTYPE_W {
        CMDTYPE_W { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W {
        CMDINDEX_W { w: self }
    }
}
