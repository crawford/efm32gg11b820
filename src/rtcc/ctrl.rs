#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `DEBUGRUN`"]
pub type DEBUGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGRUN`"]
pub struct DEBUGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGRUN_W<'a> {
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
#[doc = "Reader of field `PRECCV0TOP`"]
pub type PRECCV0TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRECCV0TOP`"]
pub struct PRECCV0TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECCV0TOP_W<'a> {
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
#[doc = "Reader of field `CCV1TOP`"]
pub type CCV1TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCV1TOP`"]
pub struct CCV1TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CCV1TOP_W<'a> {
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
#[doc = "Counter Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTPRESC_A {
    #[doc = "0: CLKCNT = LFECLKRTCC/1"]
    DIV1 = 0,
    #[doc = "1: CLKCNT = LFECLKRTCC/2"]
    DIV2 = 1,
    #[doc = "2: CLKCNT = LFECLKRTCC/4"]
    DIV4 = 2,
    #[doc = "3: CLKCNT = LFECLKRTCC/8"]
    DIV8 = 3,
    #[doc = "4: CLKCNT = LFECLKRTCC/16"]
    DIV16 = 4,
    #[doc = "5: CLKCNT = LFECLKRTCC/32"]
    DIV32 = 5,
    #[doc = "6: CLKCNT = LFECLKRTCC/64"]
    DIV64 = 6,
    #[doc = "7: CLKCNT = LFECLKRTCC/128"]
    DIV128 = 7,
    #[doc = "8: CLKCNT = LFECLKRTCC/256"]
    DIV256 = 8,
    #[doc = "9: CLKCNT = LFECLKRTCC/512"]
    DIV512 = 9,
    #[doc = "10: CLKCNT = LFECLKRTCC/1024"]
    DIV1024 = 10,
    #[doc = "11: CLKCNT = LFECLKRTCC/2048"]
    DIV2048 = 11,
    #[doc = "12: CLKCNT = LFECLKRTCC/4096"]
    DIV4096 = 12,
    #[doc = "13: CLKCNT = LFECLKRTCC/8192"]
    DIV8192 = 13,
    #[doc = "14: CLKCNT = LFECLKRTCC/16384"]
    DIV16384 = 14,
    #[doc = "15: CLKCNT = LFECLKRTCC/32768"]
    DIV32768 = 15,
}
impl From<CNTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CNTPRESC`"]
pub type CNTPRESC_R = crate::R<u8, CNTPRESC_A>;
impl CNTPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTPRESC_A {
        match self.bits {
            0 => CNTPRESC_A::DIV1,
            1 => CNTPRESC_A::DIV2,
            2 => CNTPRESC_A::DIV4,
            3 => CNTPRESC_A::DIV8,
            4 => CNTPRESC_A::DIV16,
            5 => CNTPRESC_A::DIV32,
            6 => CNTPRESC_A::DIV64,
            7 => CNTPRESC_A::DIV128,
            8 => CNTPRESC_A::DIV256,
            9 => CNTPRESC_A::DIV512,
            10 => CNTPRESC_A::DIV1024,
            11 => CNTPRESC_A::DIV2048,
            12 => CNTPRESC_A::DIV4096,
            13 => CNTPRESC_A::DIV8192,
            14 => CNTPRESC_A::DIV16384,
            15 => CNTPRESC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CNTPRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CNTPRESC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == CNTPRESC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == CNTPRESC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == CNTPRESC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == CNTPRESC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == CNTPRESC_A::DIV32768
    }
}
#[doc = "Write proxy for field `CNTPRESC`"]
pub struct CNTPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32)
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV64)
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV128)
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV256)
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV512)
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1024)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2048)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4096)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8192)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16384)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CNTTICK`"]
pub type CNTTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTTICK`"]
pub struct CNTTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTICK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BUMODETSEN`"]
pub type BUMODETSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUMODETSEN`"]
pub struct BUMODETSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUMODETSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `OSCFDETEN`"]
pub type OSCFDETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCFDETEN`"]
pub struct OSCFDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCFDETEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CNTMODE`"]
pub type CNTMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTMODE`"]
pub struct CNTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMODE_W<'a> {
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
#[doc = "Reader of field `LYEARCORRDIS`"]
pub type LYEARCORRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LYEARCORRDIS`"]
pub struct LYEARCORRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LYEARCORRDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&self) -> PRECCV0TOP_R {
        PRECCV0TOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&self) -> CCV1TOP_R {
        CCV1TOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CNTPRESC_R {
        CNTPRESC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    pub fn bumodetsen(&self) -> BUMODETSEN_R {
        BUMODETSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&self) -> OSCFDETEN_R {
        OSCFDETEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&self) -> CNTMODE_R {
        CNTMODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&self) -> LYEARCORRDIS_R {
        LYEARCORRDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&mut self) -> PRECCV0TOP_W {
        PRECCV0TOP_W { w: self }
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&mut self) -> CCV1TOP_W {
        CCV1TOP_W { w: self }
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&mut self) -> CNTPRESC_W {
        CNTPRESC_W { w: self }
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CNTTICK_W {
        CNTTICK_W { w: self }
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    pub fn bumodetsen(&mut self) -> BUMODETSEN_W {
        BUMODETSEN_W { w: self }
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&mut self) -> OSCFDETEN_W {
        OSCFDETEN_W { w: self }
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&mut self) -> CNTMODE_W {
        CNTMODE_W { w: self }
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&mut self) -> LYEARCORRDIS_W {
        LYEARCORRDIS_W { w: self }
    }
}
