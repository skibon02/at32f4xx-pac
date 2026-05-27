#[doc = "Register `AHBEN2` reader"]
pub type R = crate::R<AHBEN2_SPEC>;
#[doc = "Register `AHBEN2` writer"]
pub type W = crate::W<AHBEN2_SPEC>;
#[doc = "DVP clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVP_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<DVP_A> for bool {
    #[inline(always)]
    fn from(variant: DVP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVP` reader - DVP clock enable"]
pub type DVP_R = crate::BitReader<DVP_A>;
impl DVP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DVP_A {
        match self.bits {
            false => DVP_A::Disable,
            true => DVP_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DVP_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DVP_A::Enable
    }
}
#[doc = "Field `DVP` writer - DVP clock enable"]
pub type DVP_W<'a, REG> = crate::BitWriter<'a, REG, DVP_A>;
impl<'a, REG> DVP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DVP_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DVP_A::Enable)
    }
}
#[doc = "Field `OTGFS1` reader - OTGFS1 clock enable"]
pub use DVP_R as OTGFS1_R;
#[doc = "Field `SDIO1` reader - SDIO1 clock enable"]
pub use DVP_R as SDIO1_R;
#[doc = "Field `OTGFS1` writer - OTGFS1 clock enable"]
pub use DVP_W as OTGFS1_W;
#[doc = "Field `SDIO1` writer - SDIO1 clock enable"]
pub use DVP_W as SDIO1_W;
impl R {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp(&self) -> DVP_R {
        DVP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN2")
            .field("dvp", &self.dvp())
            .field("otgfs1", &self.otgfs1())
            .field("sdio1", &self.sdio1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp(&mut self) -> DVP_W<'_, AHBEN2_SPEC> {
        DVP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1(&mut self) -> OTGFS1_W<'_, AHBEN2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> SDIO1_W<'_, AHBEN2_SPEC> {
        SDIO1_W::new(self, 15)
    }
}
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN2_SPEC;
impl crate::RegisterSpec for AHBEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben2::R`](R) reader structure"]
impl crate::Readable for AHBEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben2::W`](W) writer structure"]
impl crate::Writable for AHBEN2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBEN2 to value 0"]
impl crate::Resettable for AHBEN2_SPEC {}
