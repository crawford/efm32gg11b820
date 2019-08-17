#[doc = "Reader of register MAXCURCAPAB"]
pub type R = crate::R<u32, super::MAXCURCAPAB>;
#[doc = "Reader of field `MAXCUR3P3VAL`"]
pub type MAXCUR3P3VAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAXCUR3P0VAL`"]
pub type MAXCUR3P0VAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAXCUR1P8VAL`"]
pub type MAXCUR1P8VAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur3p3val(&self) -> MAXCUR3P3VAL_R {
        MAXCUR3P3VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur3p0val(&self) -> MAXCUR3P0VAL_R {
        MAXCUR3P0VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur1p8val(&self) -> MAXCUR1P8VAL_R {
        MAXCUR1P8VAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
