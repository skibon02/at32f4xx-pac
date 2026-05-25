#[doc = "Register `WP` writer"]
pub type W = crate::W<WP_SPEC>;
#[doc = "Command register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDW_A {
    #[doc = "0: Write value 0x00 which is considered invalid and will activate write protection back"]
    Invalid = 0,
    #[doc = "83: Unlock key 2"]
    Key2 = 83,
    #[doc = "202: Unlock key 1"]
    Key1 = 202,
}
impl From<CMDW_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDW_A {
    type Ux = u8;
}
impl crate::IsEnum for CMDW_A {}
#[doc = "Field `CMD` writer - Command register"]
pub type CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CMDW_A>;
impl<'a, REG> CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write value 0x00 which is considered invalid and will activate write protection back"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(CMDW_A::Invalid)
    }
    #[doc = "Unlock key 2"]
    #[inline(always)]
    pub fn key2(self) -> &'a mut crate::W<REG> {
        self.variant(CMDW_A::Key2)
    }
    #[doc = "Unlock key 1"]
    #[inline(always)]
    pub fn key1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDW_A::Key1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<WP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Command register"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W<'_, WP_SPEC> {
        CMD_W::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wp::W`](W) writer structure"]
impl crate::Writable for WP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {}
