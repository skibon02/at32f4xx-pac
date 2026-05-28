#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIV` reader - The counter clock frequency Fck_cnt = Ftmr_clk /({value}+1)."]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - The counter clock frequency Fck_cnt = Ftmr_clk /({value}+1)."]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - The counter clock frequency Fck_cnt = Ftmr_clk /({value}+1)."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV").field("div", &self.div()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The counter clock frequency Fck_cnt = Ftmr_clk /({value}+1)."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<'_, DIV_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "Divider value\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {}
