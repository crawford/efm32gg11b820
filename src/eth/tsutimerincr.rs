#[doc = "Reader of register TSUTIMERINCR"]
pub type R = crate::R<u32, super::TSUTIMERINCR>;
#[doc = "Writer for register TSUTIMERINCR"]
pub type W = crate::W<u32, super::TSUTIMERINCR>;
#[doc = "Register TSUTIMERINCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUTIMERINCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSINCREMENT`"]
pub type NSINCREMENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NSINCREMENT`"]
pub struct NSINCREMENT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSINCREMENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ALTNSINCR`"]
pub type ALTNSINCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALTNSINCR`"]
pub struct ALTNSINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTNSINCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NUMINCS`"]
pub type NUMINCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMINCS`"]
pub struct NUMINCS_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMINCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&self) -> NSINCREMENT_R {
        NSINCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&self) -> ALTNSINCR_R {
        ALTNSINCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&self) -> NUMINCS_R {
        NUMINCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&mut self) -> NSINCREMENT_W {
        NSINCREMENT_W { w: self }
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&mut self) -> ALTNSINCR_W {
        ALTNSINCR_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&mut self) -> NUMINCS_W {
        NUMINCS_W { w: self }
    }
}
