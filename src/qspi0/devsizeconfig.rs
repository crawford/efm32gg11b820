#[doc = "Reader of register DEVSIZECONFIG"]
pub type R = crate::R<u32, super::DEVSIZECONFIG>;
#[doc = "Writer for register DEVSIZECONFIG"]
pub type W = crate::W<u32, super::DEVSIZECONFIG>;
#[doc = "Register DEVSIZECONFIG `reset()`'s with value 0x0010_1002"]
impl crate::ResetValue for super::DEVSIZECONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_1002
    }
}
#[doc = "Reader of field `NUMADDRBYTES`"]
pub type NUMADDRBYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMADDRBYTES`"]
pub struct NUMADDRBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMADDRBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `BYTESPERDEVICEPAGE`"]
pub type BYTESPERDEVICEPAGE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BYTESPERDEVICEPAGE`"]
pub struct BYTESPERDEVICEPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTESPERDEVICEPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `BYTESPERSUBSECTOR`"]
pub type BYTESPERSUBSECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTESPERSUBSECTOR`"]
pub struct BYTESPERSUBSECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTESPERSUBSECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEMSIZEONCS0`"]
pub type MEMSIZEONCS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMSIZEONCS0`"]
pub struct MEMSIZEONCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSIZEONCS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `MEMSIZEONCS1`"]
pub type MEMSIZEONCS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMSIZEONCS1`"]
pub struct MEMSIZEONCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSIZEONCS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&self) -> BYTESPERDEVICEPAGE_R {
        BYTESPERDEVICEPAGE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&self) -> BYTESPERSUBSECTOR_R {
        BYTESPERSUBSECTOR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&self) -> MEMSIZEONCS0_R {
        MEMSIZEONCS0_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&self) -> MEMSIZEONCS1_R {
        MEMSIZEONCS1_R::new(((self.bits >> 23) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W {
        NUMADDRBYTES_W { w: self }
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&mut self) -> BYTESPERDEVICEPAGE_W {
        BYTESPERDEVICEPAGE_W { w: self }
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&mut self) -> BYTESPERSUBSECTOR_W {
        BYTESPERSUBSECTOR_W { w: self }
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&mut self) -> MEMSIZEONCS0_W {
        MEMSIZEONCS0_W { w: self }
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&mut self) -> MEMSIZEONCS1_W {
        MEMSIZEONCS1_W { w: self }
    }
}
