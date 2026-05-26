#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<STCTRL_SPEC>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<STCTRL_SPEC>;
#[doc = "Field `SMSEL` reader - Subordinate TMR mode selection"]
pub type SMSEL_R = crate::FieldReader;
#[doc = "Field `SMSEL` writer - Subordinate TMR mode selection"]
pub type SMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STIS` reader - Subordinate TMR input selection"]
pub type STIS_R = crate::FieldReader;
#[doc = "Field `STIS` writer - Subordinate TMR input selection"]
pub type STIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STS` reader - Subordinate TMR synchronization"]
pub type STS_R = crate::BitReader;
#[doc = "Field `STS` writer - Subordinate TMR synchronization"]
pub type STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESF` reader - External signal filter"]
pub type ESF_R = crate::FieldReader;
#[doc = "Field `ESF` writer - External signal filter"]
pub type ESF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESDIV` reader - External signal divider"]
pub type ESDIV_R = crate::FieldReader;
#[doc = "Field `ESDIV` writer - External signal divider"]
pub type ESDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECMBEN` reader - External clock mode B enable"]
pub type ECMBEN_R = crate::BitReader;
#[doc = "Field `ECMBEN` writer - External clock mode B enable"]
pub type ECMBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESP` reader - External signal polarity"]
pub type ESP_R = crate::BitReader;
#[doc = "Field `ESP` writer - External signal polarity"]
pub type ESP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&self) -> STIS_R {
        STIS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    pub fn esf(&self) -> ESF_R {
        ESF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    pub fn esdiv(&self) -> ESDIV_R {
        ESDIV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    pub fn ecmben(&self) -> ECMBEN_R {
        ECMBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    pub fn esp(&self) -> ESP_R {
        ESP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("esp", &self.esp())
            .field("ecmben", &self.ecmben())
            .field("esdiv", &self.esdiv())
            .field("esf", &self.esf())
            .field("sts", &self.sts())
            .field("stis", &self.stis())
            .field("smsel", &self.smsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&mut self) -> SMSEL_W<'_, STCTRL_SPEC> {
        SMSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&mut self) -> STIS_W<'_, STCTRL_SPEC> {
        STIS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    pub fn sts(&mut self) -> STS_W<'_, STCTRL_SPEC> {
        STS_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    pub fn esf(&mut self) -> ESF_W<'_, STCTRL_SPEC> {
        ESF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    pub fn esdiv(&mut self) -> ESDIV_W<'_, STCTRL_SPEC> {
        ESDIV_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    pub fn ecmben(&mut self) -> ECMBEN_W<'_, STCTRL_SPEC> {
        ECMBEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    pub fn esp(&mut self) -> ESP_W<'_, STCTRL_SPEC> {
        ESP_W::new(self, 15)
    }
}
#[doc = "Subordinate TMR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCTRL_SPEC;
impl crate::RegisterSpec for STCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for STCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for STCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for STCTRL_SPEC {}
