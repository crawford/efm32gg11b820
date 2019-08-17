#[doc = "Reader of register REP0"]
pub type R = crate::R<u32, super::REP0>;
#[doc = "Writer for register REP0"]
pub type W = crate::W<u32, super::REP0>;
#[doc = "Register REP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REP0`"]
pub type REP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REP0`"]
pub struct REP0_W<'a> {
    w: &'a mut W,
}
impl<'a> REP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&mut self) -> REP0_W {
        REP0_W { w: self }
    }
}
