#[doc = "Register `AHBLPEN2` reader"]
pub type R = crate::R<AHBLPEN2_SPEC>;
#[doc = "Register `AHBLPEN2` writer"]
pub type W = crate::W<AHBLPEN2_SPEC>;
#[doc = "DVP clock enable during sleep mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVPLPEN_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<DVPLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DVPLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVP` reader - DVP clock enable during sleep mode"]
pub type DVP_R = crate::BitReader<DVPLPEN_A>;
impl DVP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DVPLPEN_A {
        match self.bits {
            false => DVPLPEN_A::Disable,
            true => DVPLPEN_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DVPLPEN_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DVPLPEN_A::Enable
    }
}
#[doc = "Field `DVP` writer - DVP clock enable during sleep mode"]
pub type DVP_W<'a, REG> = crate::BitWriter<'a, REG, DVPLPEN_A>;
impl<'a, REG> DVP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DVPLPEN_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DVPLPEN_A::Enable)
    }
}
#[doc = "Field `OTGFS1` reader - OTGFS1 clock enable during sleep mode"]
pub use DVP_R as OTGFS1_R;
#[doc = "Field `SDIO1` reader - SDIO1 clock enable during sleep mode"]
pub use DVP_R as SDIO1_R;
#[doc = "Field `OTGFS1` writer - OTGFS1 clock enable during sleep mode"]
pub use DVP_W as OTGFS1_W;
#[doc = "Field `SDIO1` writer - SDIO1 clock enable during sleep mode"]
pub use DVP_W as SDIO1_W;
impl R {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    pub fn dvp(&self) -> DVP_R {
        DVP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN2")
            .field("dvp", &self.dvp())
            .field("otgfs1", &self.otgfs1())
            .field("sdio1", &self.sdio1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    pub fn dvp(&mut self) -> DVP_W<'_, AHBLPEN2_SPEC> {
        DVP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs1(&mut self) -> OTGFS1_W<'_, AHBLPEN2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> SDIO1_W<'_, AHBLPEN2_SPEC> {
        SDIO1_W::new(self, 15)
    }
}
#[doc = "AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblpen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN2_SPEC;
impl crate::RegisterSpec for AHBLPEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen2::R`](R) reader structure"]
impl crate::Readable for AHBLPEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen2::W`](W) writer structure"]
impl crate::Writable for AHBLPEN2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBLPEN2 to value 0x8081"]
impl crate::Resettable for AHBLPEN2_SPEC {
    const RESET_VALUE: u32 = 0x8081;
}
