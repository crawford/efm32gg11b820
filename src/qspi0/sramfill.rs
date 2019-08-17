#[doc = "Reader of register SRAMFILL"]
pub type R = crate::R<u32, super::SRAMFILL>;
#[doc = "Reader of field `SRAMFILLINDACREAD`"]
pub type SRAMFILLINDACREAD_R = crate::R<u16, u16>;
#[doc = "Reader of field `SRAMFILLINDACWRITE`"]
pub type SRAMFILLINDACWRITE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRAM Fill Level (Indirect Read Partition)"]
    #[inline(always)]
    pub fn sramfillindacread(&self) -> SRAMFILLINDACREAD_R {
        SRAMFILLINDACREAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Fill Level (Indirect Write Partition)"]
    #[inline(always)]
    pub fn sramfillindacwrite(&self) -> SRAMFILLINDACWRITE_R {
        SRAMFILLINDACWRITE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
