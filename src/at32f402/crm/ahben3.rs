#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<AHBEN3_SPEC>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<AHBEN3_SPEC>;
#[doc = "QSPI1 clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPI1_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<QSPI1_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPI1` reader - QSPI1 clock enable"]
pub type QSPI1_R = crate::BitReader<QSPI1_A>;
impl QSPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QSPI1_A {
        match self.bits {
            false => QSPI1_A::Disable,
            true => QSPI1_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == QSPI1_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == QSPI1_A::Enable
    }
}
#[doc = "Field `QSPI1` writer - QSPI1 clock enable"]
pub type QSPI1_W<'a, REG> = crate::BitWriter<'a, REG, QSPI1_A>;
impl<'a, REG> QSPI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI1_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI1_A::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN3")
            .field("qspi1", &self.qspi1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&mut self) -> QSPI1_W<'_, AHBEN3_SPEC> {
        QSPI1_W::new(self, 1)
    }
}
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN3_SPEC;
impl crate::RegisterSpec for AHBEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben3::R`](R) reader structure"]
impl crate::Readable for AHBEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben3::W`](W) writer structure"]
impl crate::Writable for AHBEN3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBEN3 to value 0"]
impl crate::Resettable for AHBEN3_SPEC {}
