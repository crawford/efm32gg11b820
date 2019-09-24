#[doc = "Reader of register BLKSIZE"]
pub type R = crate::R<u32, super::BLKSIZE>;
#[doc = "Writer for register BLKSIZE"]
pub type W = crate::W<u32, super::BLKSIZE>;
#[doc = "Register BLKSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::BLKSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRBLKSIZE_A {
    #[doc = "0: `0`"]
    NOXFER,
}
impl From<TFRBLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: TFRBLKSIZE_A) -> Self {
        match variant {
            TFRBLKSIZE_A::NOXFER => 0,
        }
    }
}
#[doc = "Reader of field `TFRBLKSIZE`"]
pub type TFRBLKSIZE_R = crate::R<u16, TFRBLKSIZE_A>;
impl TFRBLKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TFRBLKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TFRBLKSIZE_A::NOXFER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOXFER`"]
    #[inline(always)]
    pub fn is_noxfer(&self) -> bool {
        *self == TFRBLKSIZE_A::NOXFER
    }
}
#[doc = "Write proxy for field `TFRBLKSIZE`"]
pub struct TFRBLKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFRBLKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFRBLKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn noxfer(self) -> &'a mut W {
        self.variant(TFRBLKSIZE_A::NOXFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Host SDMA Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTSDMABUFSIZE_A {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    SIZE4,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    SIZE8,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    SIZE16,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    SIZE32,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    SIZE64,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    SIZE128,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
    SIZE256,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    SIZE512,
}
impl From<HSTSDMABUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTSDMABUFSIZE_A) -> Self {
        match variant {
            HSTSDMABUFSIZE_A::SIZE4 => 0,
            HSTSDMABUFSIZE_A::SIZE8 => 1,
            HSTSDMABUFSIZE_A::SIZE16 => 2,
            HSTSDMABUFSIZE_A::SIZE32 => 3,
            HSTSDMABUFSIZE_A::SIZE64 => 4,
            HSTSDMABUFSIZE_A::SIZE128 => 5,
            HSTSDMABUFSIZE_A::SIZE256 => 6,
            HSTSDMABUFSIZE_A::SIZE512 => 7,
        }
    }
}
#[doc = "Reader of field `HSTSDMABUFSIZE`"]
pub type HSTSDMABUFSIZE_R = crate::R<u8, HSTSDMABUFSIZE_A>;
impl HSTSDMABUFSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTSDMABUFSIZE_A {
        match self.bits {
            0 => HSTSDMABUFSIZE_A::SIZE4,
            1 => HSTSDMABUFSIZE_A::SIZE8,
            2 => HSTSDMABUFSIZE_A::SIZE16,
            3 => HSTSDMABUFSIZE_A::SIZE32,
            4 => HSTSDMABUFSIZE_A::SIZE64,
            5 => HSTSDMABUFSIZE_A::SIZE128,
            6 => HSTSDMABUFSIZE_A::SIZE256,
            7 => HSTSDMABUFSIZE_A::SIZE512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE4`"]
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE4
    }
    #[doc = "Checks if the value of the field is `SIZE8`"]
    #[inline(always)]
    pub fn is_size8(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE8
    }
    #[doc = "Checks if the value of the field is `SIZE16`"]
    #[inline(always)]
    pub fn is_size16(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE16
    }
    #[doc = "Checks if the value of the field is `SIZE32`"]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE32
    }
    #[doc = "Checks if the value of the field is `SIZE64`"]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE64
    }
    #[doc = "Checks if the value of the field is `SIZE128`"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE128
    }
    #[doc = "Checks if the value of the field is `SIZE256`"]
    #[inline(always)]
    pub fn is_size256(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE256
    }
    #[doc = "Checks if the value of the field is `SIZE512`"]
    #[inline(always)]
    pub fn is_size512(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE512
    }
}
#[doc = "Write proxy for field `HSTSDMABUFSIZE`"]
pub struct HSTSDMABUFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTSDMABUFSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSTSDMABUFSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn size4(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE4)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn size8(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE8)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn size16(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE16)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn size32(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE32)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn size64(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE64)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn size128(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE128)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn size256(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE256)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn size512(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Blocks Count for Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSCNTFORCURRTFR_A {
    #[doc = "0: `0`"]
    STOPCNT,
}
impl From<BLKSCNTFORCURRTFR_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSCNTFORCURRTFR_A) -> Self {
        match variant {
            BLKSCNTFORCURRTFR_A::STOPCNT => 0,
        }
    }
}
#[doc = "Reader of field `BLKSCNTFORCURRTFR`"]
pub type BLKSCNTFORCURRTFR_R = crate::R<u16, BLKSCNTFORCURRTFR_A>;
impl BLKSCNTFORCURRTFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BLKSCNTFORCURRTFR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLKSCNTFORCURRTFR_A::STOPCNT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOPCNT`"]
    #[inline(always)]
    pub fn is_stopcnt(&self) -> bool {
        *self == BLKSCNTFORCURRTFR_A::STOPCNT
    }
}
#[doc = "Write proxy for field `BLKSCNTFORCURRTFR`"]
pub struct BLKSCNTFORCURRTFR_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKSCNTFORCURRTFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKSCNTFORCURRTFR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stopcnt(self) -> &'a mut W {
        self.variant(BLKSCNTFORCURRTFR_A::STOPCNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&self) -> TFRBLKSIZE_R {
        TFRBLKSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&self) -> HSTSDMABUFSIZE_R {
        HSTSDMABUFSIZE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&self) -> BLKSCNTFORCURRTFR_R {
        BLKSCNTFORCURRTFR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&mut self) -> TFRBLKSIZE_W {
        TFRBLKSIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&mut self) -> HSTSDMABUFSIZE_W {
        HSTSDMABUFSIZE_W { w: self }
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&mut self) -> BLKSCNTFORCURRTFR_W {
        BLKSCNTFORCURRTFR_W { w: self }
    }
}
