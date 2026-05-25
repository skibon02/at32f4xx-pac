#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `HICKEN` reader - High speed internal clock enable"]
pub type HICKEN_R = crate::BitReader;
#[doc = "Field `HICKEN` writer - High speed internal clock enable"]
pub type HICKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICKSTBL` reader - High speed internal clock ready flag"]
pub type HICKSTBL_R = crate::BitReader;
#[doc = "Field `HICKTRIM` reader - High speed internal clock trimming"]
pub type HICKTRIM_R = crate::FieldReader;
#[doc = "Field `HICKTRIM` writer - High speed internal clock trimming"]
pub type HICKTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HICKCAL` reader - High speed internal clock calibration"]
pub type HICKCAL_R = crate::FieldReader;
#[doc = "Field `HEXTEN` reader - High speed exernal crystal enable"]
pub type HEXTEN_R = crate::BitReader;
#[doc = "Field `HEXTEN` writer - High speed exernal crystal enable"]
pub type HEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEXTSTBL` reader - High speed exernal crystal ready flag"]
pub type HEXTSTBL_R = crate::BitReader;
#[doc = "Field `HEXTBYPS` reader - High speed exernal crystal bypass"]
pub type HEXTBYPS_R = crate::BitReader;
#[doc = "Field `HEXTBYPS` writer - High speed exernal crystal bypass"]
pub type HEXTBYPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock failure detection enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock failure detection enable"]
pub type CFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBL` reader - PLL clock ready flag"]
pub type PLLSTBL_R = crate::BitReader;
#[doc = "Field `PLLUSTBL` reader - PLLU clock ready flag"]
pub type PLLUSTBL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    pub fn hicken(&self) -> HICKEN_R {
        HICKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High speed internal clock ready flag"]
    #[inline(always)]
    pub fn hickstbl(&self) -> HICKSTBL_R {
        HICKSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&self) -> HICKTRIM_R {
        HICKTRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - High speed internal clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HICKCAL_R {
        HICKCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    pub fn hexten(&self) -> HEXTEN_R {
        HEXTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High speed exernal crystal ready flag"]
    #[inline(always)]
    pub fn hextstbl(&self) -> HEXTSTBL_R {
        HEXTSTBL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    pub fn hextbyps(&self) -> HEXTBYPS_R {
        HEXTBYPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllstbl(&self) -> PLLSTBL_R {
        PLLSTBL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLLU clock ready flag"]
    #[inline(always)]
    pub fn pllustbl(&self) -> PLLUSTBL_R {
        PLLUSTBL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("hicken", &self.hicken())
            .field("hickstbl", &self.hickstbl())
            .field("hicktrim", &self.hicktrim())
            .field("hickcal", &self.hickcal())
            .field("hexten", &self.hexten())
            .field("hextstbl", &self.hextstbl())
            .field("hextbyps", &self.hextbyps())
            .field("cfden", &self.cfden())
            .field("pllen", &self.pllen())
            .field("pllstbl", &self.pllstbl())
            .field("pllustbl", &self.pllustbl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    pub fn hicken(&mut self) -> HICKEN_W<'_, CTRL_SPEC> {
        HICKEN_W::new(self, 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&mut self) -> HICKTRIM_W<'_, CTRL_SPEC> {
        HICKTRIM_W::new(self, 2)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    pub fn hexten(&mut self) -> HEXTEN_W<'_, CTRL_SPEC> {
        HEXTEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    pub fn hextbyps(&mut self) -> HEXTBYPS_W<'_, CTRL_SPEC> {
        HEXTBYPS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W<'_, CTRL_SPEC> {
        CFDEN_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<'_, CTRL_SPEC> {
        PLLEN_W::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x83"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
