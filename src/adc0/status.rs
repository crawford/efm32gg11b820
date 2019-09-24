#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `SINGLEACT`"]
pub type SINGLEACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANACT`"]
pub type SCANACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANPENDING`"]
pub type SCANPENDING_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLEREFWARM`"]
pub type SINGLEREFWARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANREFWARM`"]
pub type SCANREFWARM_R = crate::R<bool, bool>;
#[doc = "Programming Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_A {
    #[doc = "1: `1`"]
    BUSCONF,
    #[doc = "2: `10`"]
    NEGSELCONF,
}
impl From<PROGERR_A> for u8 {
    #[inline(always)]
    fn from(variant: PROGERR_A) -> Self {
        match variant {
            PROGERR_A::BUSCONF => 1,
            PROGERR_A::NEGSELCONF => 2,
        }
    }
}
#[doc = "Reader of field `PROGERR`"]
pub type PROGERR_R = crate::R<u8, PROGERR_A>;
impl PROGERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROGERR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PROGERR_A::BUSCONF),
            2 => Val(PROGERR_A::NEGSELCONF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSCONF`"]
    #[inline(always)]
    pub fn is_busconf(&self) -> bool {
        *self == PROGERR_A::BUSCONF
    }
    #[doc = "Checks if the value of the field is `NEGSELCONF`"]
    #[inline(always)]
    pub fn is_negselconf(&self) -> bool {
        *self == PROGERR_A::NEGSELCONF
    }
}
#[doc = "Reader of field `WARM`"]
pub type WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLEDV`"]
pub type SINGLEDV_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANDV`"]
pub type SCANDV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Single Channel Conversion Active"]
    #[inline(always)]
    pub fn singleact(&self) -> SINGLEACT_R {
        SINGLEACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline(always)]
    pub fn scanact(&self) -> SCANACT_R {
        SCANACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Scan Conversion Pending"]
    #[inline(always)]
    pub fn scanpending(&self) -> SCANPENDING_R {
        SCANPENDING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Single Channel Reference Warmed Up"]
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SINGLEREFWARM_R {
        SINGLEREFWARM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline(always)]
    pub fn scanrefwarm(&self) -> SCANREFWARM_R {
        SCANREFWARM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Programming Error Status"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline(always)]
    pub fn warm(&self) -> WARM_R {
        WARM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Single Channel Data Valid"]
    #[inline(always)]
    pub fn singledv(&self) -> SINGLEDV_R {
        SINGLEDV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline(always)]
    pub fn scandv(&self) -> SCANDV_R {
        SCANDV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
