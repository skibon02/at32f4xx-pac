#[doc = "Register `OSQ6` reader"]
pub type R = crate::R<OSQ6_SPEC>;
#[doc = "Register `OSQ6` writer"]
pub type W = crate::W<OSQ6_SPEC>;
#[doc = "Field `OSN29` reader - Number of 29st conversion in ordinary sequence"]
pub type OSN29_R = crate::FieldReader;
#[doc = "Field `OSN29` writer - Number of 29st conversion in ordinary sequence"]
pub type OSN29_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN30` reader - Number of 30nd conversion in ordinary sequence"]
pub type OSN30_R = crate::FieldReader;
#[doc = "Field `OSN30` writer - Number of 30nd conversion in ordinary sequence"]
pub type OSN30_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN31` reader - number of 31rd conversion in ordinary sequence"]
pub type OSN31_R = crate::FieldReader;
#[doc = "Field `OSN31` writer - number of 31rd conversion in ordinary sequence"]
pub type OSN31_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN32` reader - Number of 32th conversion in ordinary sequence"]
pub type OSN32_R = crate::FieldReader;
#[doc = "Field `OSN32` writer - Number of 32th conversion in ordinary sequence"]
pub type OSN32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of 29st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn29(&self) -> OSN29_R {
        OSN29_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 30nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn30(&self) -> OSN30_R {
        OSN30_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 31rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn31(&self) -> OSN31_R {
        OSN31_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn32(&self) -> OSN32_R {
        OSN32_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ6")
            .field("osn32", &self.osn32())
            .field("osn31", &self.osn31())
            .field("osn30", &self.osn30())
            .field("osn29", &self.osn29())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 29st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn29(&mut self) -> OSN29_W<'_, OSQ6_SPEC> {
        OSN29_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 30nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn30(&mut self) -> OSN30_W<'_, OSQ6_SPEC> {
        OSN30_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - number of 31rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn31(&mut self) -> OSN31_W<'_, OSQ6_SPEC> {
        OSN31_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn32(&mut self) -> OSN32_W<'_, OSQ6_SPEC> {
        OSN32_W::new(self, 15)
    }
}
#[doc = "Ordinary sequence register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`osq6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ6_SPEC;
impl crate::RegisterSpec for OSQ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq6::R`](R) reader structure"]
impl crate::Readable for OSQ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq6::W`](W) writer structure"]
impl crate::Writable for OSQ6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ6 to value 0"]
impl crate::Resettable for OSQ6_SPEC {}
