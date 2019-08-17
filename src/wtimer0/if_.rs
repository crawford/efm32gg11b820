#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UF`"]
pub type UF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRCHG`"]
pub type DIRCHG_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC0`"]
pub type CC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC1`"]
pub type CC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC2`"]
pub type CC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC3`"]
pub type CC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICBOF0`"]
pub type ICBOF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICBOF1`"]
pub type ICBOF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICBOF2`"]
pub type ICBOF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICBOF3`"]
pub type ICBOF3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CC Channel 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CC Channel 3 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof3(&self) -> ICBOF3_R {
        ICBOF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
