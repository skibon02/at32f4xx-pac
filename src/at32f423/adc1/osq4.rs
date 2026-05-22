#[doc = "Register `OSQ4` reader"]
pub type R = crate::R<OSQ4_SPEC>;
#[doc = "Register `OSQ4` writer"]
pub type W = crate::W<OSQ4_SPEC>;
#[doc = "Field `OSN17` reader - Number of 17st conversion in ordinary sequence"]
pub type OSN17_R = crate::FieldReader;
#[doc = "Field `OSN17` writer - Number of 17st conversion in ordinary sequence"]
pub type OSN17_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN18` reader - Number of 18nd conversion in ordinary sequence"]
pub type OSN18_R = crate::FieldReader;
#[doc = "Field `OSN18` writer - Number of 18nd conversion in ordinary sequence"]
pub type OSN18_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN19` reader - number of 19rd conversion in ordinary sequence"]
pub type OSN19_R = crate::FieldReader;
#[doc = "Field `OSN19` writer - number of 19rd conversion in ordinary sequence"]
pub type OSN19_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN20` reader - Number of 20th conversion in ordinary sequence"]
pub type OSN20_R = crate::FieldReader;
#[doc = "Field `OSN20` writer - Number of 20th conversion in ordinary sequence"]
pub type OSN20_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN21` reader - Number of 21th conversion in ordinary sequence"]
pub type OSN21_R = crate::FieldReader;
#[doc = "Field `OSN21` writer - Number of 21th conversion in ordinary sequence"]
pub type OSN21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN22` reader - Number of 22th conversion in ordinary sequence"]
pub type OSN22_R = crate::FieldReader;
#[doc = "Field `OSN22` writer - Number of 22th conversion in ordinary sequence"]
pub type OSN22_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of 17st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn17(&self) -> OSN17_R {
        OSN17_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 18nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn18(&self) -> OSN18_R {
        OSN18_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 19rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn19(&self) -> OSN19_R {
        OSN19_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 20th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn20(&self) -> OSN20_R {
        OSN20_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 21th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn21(&self) -> OSN21_R {
        OSN21_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 22th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn22(&self) -> OSN22_R {
        OSN22_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ4")
            .field("osn22", &self.osn22())
            .field("osn21", &self.osn21())
            .field("osn20", &self.osn20())
            .field("osn19", &self.osn19())
            .field("osn18", &self.osn18())
            .field("osn17", &self.osn17())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 17st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn17(&mut self) -> OSN17_W<'_, OSQ4_SPEC> {
        OSN17_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 18nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn18(&mut self) -> OSN18_W<'_, OSQ4_SPEC> {
        OSN18_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - number of 19rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn19(&mut self) -> OSN19_W<'_, OSQ4_SPEC> {
        OSN19_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 20th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn20(&mut self) -> OSN20_W<'_, OSQ4_SPEC> {
        OSN20_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 21th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn21(&mut self) -> OSN21_W<'_, OSQ4_SPEC> {
        OSN21_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 22th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn22(&mut self) -> OSN22_W<'_, OSQ4_SPEC> {
        OSN22_W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`osq4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ4_SPEC;
impl crate::RegisterSpec for OSQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq4::R`](R) reader structure"]
impl crate::Readable for OSQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq4::W`](W) writer structure"]
impl crate::Writable for OSQ4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ4 to value 0"]
impl crate::Resettable for OSQ4_SPEC {}
