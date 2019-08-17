#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC0`"]
pub type CC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC1`"]
pub type CC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC2`"]
pub type CC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSCFAIL`"]
pub type OSCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNTTICK`"]
pub type CNTTICK_R = crate::R<bool, bool>;
#[doc = "Reader of field `MINTICK`"]
pub type MINTICK_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOURTICK`"]
pub type HOURTICK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAYTICK`"]
pub type DAYTICK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAYOWOF`"]
pub type DAYOWOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONTHTICK`"]
pub type MONTHTICK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Oscillator Failure Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&self) -> OSCFAIL_R {
        OSCFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Main Counter Tick"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Minute Tick"]
    #[inline(always)]
    pub fn mintick(&self) -> MINTICK_R {
        MINTICK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hour Tick"]
    #[inline(always)]
    pub fn hourtick(&self) -> HOURTICK_R {
        HOURTICK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Day Tick"]
    #[inline(always)]
    pub fn daytick(&self) -> DAYTICK_R {
        DAYTICK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Day of Week Overflow"]
    #[inline(always)]
    pub fn dayowof(&self) -> DAYOWOF_R {
        DAYOWOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Month Tick"]
    #[inline(always)]
    pub fn monthtick(&self) -> MONTHTICK_R {
        MONTHTICK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
