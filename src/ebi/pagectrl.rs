#[doc = "Reader of register PAGECTRL"]
pub type R = crate::R<u32, super::PAGECTRL>;
#[doc = "Writer for register PAGECTRL"]
pub type W = crate::W<u32, super::PAGECTRL>;
#[doc = "Register PAGECTRL `reset()`'s with value 0x0f00"]
impl crate::ResetValue for super::PAGECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00
    }
}
#[doc = "Page Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGELEN_A {
    #[doc = "0: 4 members in a page."]
    MEMBER4,
    #[doc = "1: 8 members in a page."]
    MEMBER8,
    #[doc = "2: 16 members in a page."]
    MEMBER16,
    #[doc = "3: 32 members in a page."]
    MEMBER32,
}
impl From<PAGELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGELEN_A) -> Self {
        match variant {
            PAGELEN_A::MEMBER4 => 0,
            PAGELEN_A::MEMBER8 => 1,
            PAGELEN_A::MEMBER16 => 2,
            PAGELEN_A::MEMBER32 => 3,
        }
    }
}
#[doc = "Reader of field `PAGELEN`"]
pub type PAGELEN_R = crate::R<u8, PAGELEN_A>;
impl PAGELEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGELEN_A {
        match self.bits {
            0 => PAGELEN_A::MEMBER4,
            1 => PAGELEN_A::MEMBER8,
            2 => PAGELEN_A::MEMBER16,
            3 => PAGELEN_A::MEMBER32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEMBER4`"]
    #[inline(always)]
    pub fn is_member4(&self) -> bool {
        *self == PAGELEN_A::MEMBER4
    }
    #[doc = "Checks if the value of the field is `MEMBER8`"]
    #[inline(always)]
    pub fn is_member8(&self) -> bool {
        *self == PAGELEN_A::MEMBER8
    }
    #[doc = "Checks if the value of the field is `MEMBER16`"]
    #[inline(always)]
    pub fn is_member16(&self) -> bool {
        *self == PAGELEN_A::MEMBER16
    }
    #[doc = "Checks if the value of the field is `MEMBER32`"]
    #[inline(always)]
    pub fn is_member32(&self) -> bool {
        *self == PAGELEN_A::MEMBER32
    }
}
#[doc = "Write proxy for field `PAGELEN`"]
pub struct PAGELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGELEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGELEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn member4(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER4)
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn member8(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER8)
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn member16(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER16)
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn member32(self) -> &'a mut W {
        self.variant(PAGELEN_A::MEMBER32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INCHIT`"]
pub type INCHIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCHIT`"]
pub struct INCHIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCHIT_W<'a> {
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
#[doc = "Reader of field `RDPA`"]
pub type RDPA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDPA`"]
pub struct RDPA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `KEEPOPEN`"]
pub type KEEPOPEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEEPOPEN`"]
pub struct KEEPOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEPOPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 20)) | (((value as u32) & 0x7f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&self) -> PAGELEN_R {
        PAGELEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    pub fn inchit(&self) -> INCHIT_R {
        INCHIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&self) -> RDPA_R {
        RDPA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    pub fn keepopen(&self) -> KEEPOPEN_R {
        KEEPOPEN_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&mut self) -> PAGELEN_W {
        PAGELEN_W { w: self }
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    pub fn inchit(&mut self) -> INCHIT_W {
        INCHIT_W { w: self }
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&mut self) -> RDPA_W {
        RDPA_W { w: self }
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    pub fn keepopen(&mut self) -> KEEPOPEN_W {
        KEEPOPEN_W { w: self }
    }
}
