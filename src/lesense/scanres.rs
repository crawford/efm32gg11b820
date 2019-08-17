#[doc = "Reader of register SCANRES"]
pub type R = crate::R<u32, super::SCANRES>;
#[doc = "Writer for register SCANRES"]
pub type W = crate::W<u32, super::SCANRES>;
#[doc = "Register SCANRES `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCANRES`"]
pub type SCANRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCANRES`"]
pub struct SCANRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `STEPDIR`"]
pub type STEPDIR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STEPDIR`"]
pub struct STEPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&self) -> SCANRES_R {
        SCANRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&self) -> STEPDIR_R {
        STEPDIR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&mut self) -> SCANRES_W {
        SCANRES_W { w: self }
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&mut self) -> STEPDIR_W {
        STEPDIR_W { w: self }
    }
}
