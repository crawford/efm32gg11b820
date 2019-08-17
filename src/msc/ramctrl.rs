#[doc = "Reader of register RAMCTRL"]
pub type R = crate::R<u32, super::RAMCTRL>;
#[doc = "Writer for register RAMCTRL"]
pub type W = crate::W<u32, super::RAMCTRL>;
#[doc = "Register RAMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAMWSEN`"]
pub type RAMWSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMWSEN`"]
pub struct RAMWSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMWSEN_W<'a> {
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
#[doc = "Reader of field `RAMPREFETCHEN`"]
pub type RAMPREFETCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMPREFETCHEN`"]
pub struct RAMPREFETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPREFETCHEN_W<'a> {
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
#[doc = "Reader of field `RAM1WSEN`"]
pub type RAM1WSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1WSEN`"]
pub struct RAM1WSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1WSEN_W<'a> {
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
#[doc = "Reader of field `RAM1PREFETCHEN`"]
pub type RAM1PREFETCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1PREFETCHEN`"]
pub struct RAM1PREFETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1PREFETCHEN_W<'a> {
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
#[doc = "Reader of field `RAM2CACHEEN`"]
pub type RAM2CACHEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM2CACHEEN`"]
pub struct RAM2CACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2CACHEEN_W<'a> {
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
#[doc = "Reader of field `RAM2WSEN`"]
pub type RAM2WSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM2WSEN`"]
pub struct RAM2WSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2WSEN_W<'a> {
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
#[doc = "Reader of field `RAM2PREFETCHEN`"]
pub type RAM2PREFETCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM2PREFETCHEN`"]
pub struct RAM2PREFETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2PREFETCHEN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&self) -> RAMWSEN_R {
        RAMWSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&self) -> RAMPREFETCHEN_R {
        RAMPREFETCHEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&self) -> RAM1WSEN_R {
        RAM1WSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&self) -> RAM1PREFETCHEN_R {
        RAM1PREFETCHEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&self) -> RAM2CACHEEN_R {
        RAM2CACHEEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&self) -> RAM2WSEN_R {
        RAM2WSEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&self) -> RAM2PREFETCHEN_R {
        RAM2PREFETCHEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&mut self) -> RAMWSEN_W {
        RAMWSEN_W { w: self }
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&mut self) -> RAMPREFETCHEN_W {
        RAMPREFETCHEN_W { w: self }
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&mut self) -> RAM1WSEN_W {
        RAM1WSEN_W { w: self }
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&mut self) -> RAM1PREFETCHEN_W {
        RAM1PREFETCHEN_W { w: self }
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&mut self) -> RAM2CACHEEN_W {
        RAM2CACHEEN_W { w: self }
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&mut self) -> RAM2WSEN_W {
        RAM2WSEN_W { w: self }
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&mut self) -> RAM2PREFETCHEN_W {
        RAM2PREFETCHEN_W { w: self }
    }
}
