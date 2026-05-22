#[doc = "Register `UNLOCK` writer"]
pub type W = crate::W<UNLOCK_SPEC>;
#[doc = "Unlock key value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UKVALW_A {
    #[doc = "1164378403: Unlock key 1"]
    Key1 = 1164378403,
    #[doc = "3455027627: Unlock key 2"]
    Key2 = 3455027627,
}
impl From<UKVALW_A> for u32 {
    #[inline(always)]
    fn from(variant: UKVALW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UKVALW_A {
    type Ux = u32;
}
impl crate::IsEnum for UKVALW_A {}
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, UKVALW_A>;
impl<'a, REG> UKVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unlock key 1"]
    #[inline(always)]
    pub fn key1(self) -> &'a mut crate::W<REG> {
        self.variant(UKVALW_A::Key1)
    }
    #[doc = "Unlock key 2"]
    #[inline(always)]
    pub fn key2(self) -> &'a mut crate::W<REG> {
        self.variant(UKVALW_A::Key2)
    }
}
impl core::fmt::Debug for crate::generic::Reg<UNLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    pub fn ukval(&mut self) -> UKVAL_W<'_, UNLOCK_SPEC> {
        UKVAL_W::new(self, 0)
    }
}
#[doc = "Unlock register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLOCK_SPEC;
impl crate::RegisterSpec for UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock::W`](W) writer structure"]
impl crate::Writable for UNLOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK to value 0"]
impl crate::Resettable for UNLOCK_SPEC {}
