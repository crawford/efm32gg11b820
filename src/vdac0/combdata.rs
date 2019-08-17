#[doc = "Reader of register COMBDATA"]
pub type R = crate::R<u32, super::COMBDATA>;
#[doc = "Writer for register COMBDATA"]
pub type W = crate::W<u32, super::COMBDATA>;
#[doc = "Register COMBDATA `reset()`'s with value 0x0800_0800"]
impl crate::ResetValue for super::COMBDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0800
    }
}
#[doc = "Reader of field `CH0DATA`"]
pub type CH0DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH0DATA`"]
pub struct CH0DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `CH1DATA`"]
pub type CH1DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1DATA`"]
pub struct CH1DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&self) -> CH0DATA_R {
        CH0DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&self) -> CH1DATA_R {
        CH1DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&mut self) -> CH0DATA_W {
        CH0DATA_W { w: self }
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&mut self) -> CH1DATA_W {
        CH1DATA_W { w: self }
    }
}
