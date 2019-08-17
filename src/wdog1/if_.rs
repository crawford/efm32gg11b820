#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WARN`"]
pub type WARN_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIN`"]
pub type WIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEM0`"]
pub type PEM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEM1`"]
pub type PEM1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRS Channel Zero Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRS Channel One Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
