#[doc = "Reader of register DCDCLNCOMPCTRL"]
pub type R = crate::R<u32, super::DCDCLNCOMPCTRL>;
#[doc = "Writer for register DCDCLNCOMPCTRL"]
pub type W = crate::W<u32, super::DCDCLNCOMPCTRL>;
#[doc = "Register DCDCLNCOMPCTRL `reset()`'s with value 0x5720_4077"]
impl crate::ResetValue for super::DCDCLNCOMPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5720_4077
    }
}
#[doc = "Reader of field `COMPENR1`"]
pub type COMPENR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENR1`"]
pub struct COMPENR1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `COMPENR2`"]
pub type COMPENR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENR2`"]
pub struct COMPENR2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMPENR3`"]
pub type COMPENR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENR3`"]
pub struct COMPENR3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `COMPENC1`"]
pub type COMPENC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENC1`"]
pub struct COMPENC1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `COMPENC2`"]
pub type COMPENC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENC2`"]
pub struct COMPENC2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `COMPENC3`"]
pub type COMPENC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPENC3`"]
pub struct COMPENC3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPENC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&self) -> COMPENR1_R {
        COMPENR1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&self) -> COMPENR2_R {
        COMPENR2_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&self) -> COMPENR3_R {
        COMPENR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&self) -> COMPENC1_R {
        COMPENC1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&self) -> COMPENC2_R {
        COMPENC2_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&self) -> COMPENC3_R {
        COMPENC3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&mut self) -> COMPENR1_W {
        COMPENR1_W { w: self }
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&mut self) -> COMPENR2_W {
        COMPENR2_W { w: self }
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&mut self) -> COMPENR3_W {
        COMPENR3_W { w: self }
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&mut self) -> COMPENC1_W {
        COMPENC1_W { w: self }
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&mut self) -> COMPENC2_W {
        COMPENC2_W { w: self }
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&mut self) -> COMPENC3_W {
        COMPENC3_W { w: self }
    }
}
