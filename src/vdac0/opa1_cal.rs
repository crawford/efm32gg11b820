#[doc = "Reader of register OPA1_CAL"]
pub type R = crate::R<u32, super::OPA1_CAL>;
#[doc = "Writer for register OPA1_CAL"]
pub type W = crate::W<u32, super::OPA1_CAL>;
#[doc = "Register OPA1_CAL `reset()`'s with value 0x80e7"]
impl crate::ResetValue for super::OPA1_CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80e7
    }
}
#[doc = "Reader of field `CM1`"]
pub type CM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM1`"]
pub struct CM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CM2`"]
pub type CM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM2`"]
pub struct CM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `CM3`"]
pub type CM3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM3`"]
pub struct CM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `GM`"]
pub type GM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GM`"]
pub struct GM_W<'a> {
    w: &'a mut W,
}
impl<'a> GM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `GM3`"]
pub type GM3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GM3`"]
pub struct GM3_W<'a> {
    w: &'a mut W,
}
impl<'a> GM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `OFFSETP`"]
pub type OFFSETP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSETP`"]
pub struct OFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `OFFSETN`"]
pub type OFFSETN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSETN`"]
pub struct OFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&self) -> CM1_R {
        CM1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&self) -> CM2_R {
        CM2_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&self) -> CM3_R {
        CM3_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&self) -> GM3_R {
        GM3_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&self) -> OFFSETP_R {
        OFFSETP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&self) -> OFFSETN_R {
        OFFSETN_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&mut self) -> CM1_W {
        CM1_W { w: self }
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&mut self) -> CM2_W {
        CM2_W { w: self }
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&mut self) -> CM3_W {
        CM3_W { w: self }
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W {
        GM_W { w: self }
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&mut self) -> GM3_W {
        GM3_W { w: self }
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&mut self) -> OFFSETP_W {
        OFFSETP_W { w: self }
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&mut self) -> OFFSETN_W {
        OFFSETN_W { w: self }
    }
}
