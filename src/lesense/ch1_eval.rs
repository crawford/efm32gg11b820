#[doc = "Reader of register CH1_EVAL"]
pub type R = crate::R<u32, super::CH1_EVAL>;
#[doc = "Writer for register CH1_EVAL"]
pub type W = crate::W<u32, super::CH1_EVAL>;
#[doc = "Register CH1_EVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_EVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPTHRES`"]
pub type COMPTHRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPTHRES`"]
pub struct COMPTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
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
#[doc = "Reader of field `DECODE`"]
pub type DECODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECODE`"]
pub struct DECODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODE_W<'a> {
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
#[doc = "Enable Storing of Sensor Sample in Result Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STRSAMPLE_A {
    #[doc = "0: Nothing will be stored in the result buffer."]
    DISABLE = 0,
    #[doc = "1: The sensor sample data will be stored in the result buffer."]
    DATA = 1,
    #[doc = "2: The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    DATASRC = 2,
}
impl From<STRSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: STRSAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STRSAMPLE`"]
pub type STRSAMPLE_R = crate::R<u8, STRSAMPLE_A>;
impl STRSAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STRSAMPLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STRSAMPLE_A::DISABLE),
            1 => Val(STRSAMPLE_A::DATA),
            2 => Val(STRSAMPLE_A::DATASRC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STRSAMPLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STRSAMPLE_A::DATA
    }
    #[doc = "Checks if the value of the field is `DATASRC`"]
    #[inline(always)]
    pub fn is_datasrc(&self) -> bool {
        *self == STRSAMPLE_A::DATASRC
    }
}
#[doc = "Write proxy for field `STRSAMPLE`"]
pub struct STRSAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRSAMPLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DISABLE)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATA)
    }
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn datasrc(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATASRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SCANRESINV`"]
pub type SCANRESINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANRESINV`"]
pub struct SCANRESINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANRESINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Configure Evaluation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Threshold comparison is used to evaluate sensor result"]
    THRES = 0,
    #[doc = "1: Sliding window is used to evaluate sensor result"]
    SLIDINGWIN = 1,
    #[doc = "2: Step detection is used to evaluate sensor result"]
    STEPDET = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::THRES),
            1 => Val(MODE_A::SLIDINGWIN),
            2 => Val(MODE_A::STEPDET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `THRES`"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == MODE_A::THRES
    }
    #[doc = "Checks if the value of the field is `SLIDINGWIN`"]
    #[inline(always)]
    pub fn is_slidingwin(&self) -> bool {
        *self == MODE_A::SLIDINGWIN
    }
    #[doc = "Checks if the value of the field is `STEPDET`"]
    #[inline(always)]
    pub fn is_stepdet(&self) -> bool {
        *self == MODE_A::STEPDET
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn thres(self) -> &'a mut W {
        self.variant(MODE_A::THRES)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn slidingwin(self) -> &'a mut W {
        self.variant(MODE_A::SLIDINGWIN)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn stepdet(self) -> &'a mut W {
        self.variant(MODE_A::STEPDET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    pub fn compthres(&self) -> COMPTHRES_R {
        COMPTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DECODE_R {
        DECODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    pub fn strsample(&self) -> STRSAMPLE_R {
        STRSAMPLE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> SCANRESINV_R {
        SCANRESINV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    pub fn compthres(&mut self) -> COMPTHRES_W {
        COMPTHRES_W { w: self }
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    pub fn decode(&mut self) -> DECODE_W {
        DECODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    pub fn strsample(&mut self) -> STRSAMPLE_W {
        STRSAMPLE_W { w: self }
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    pub fn scanresinv(&mut self) -> SCANRESINV_W {
        SCANRESINV_W { w: self }
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
