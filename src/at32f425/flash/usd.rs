#[doc = "Register `USD` reader"]
pub type R = crate::R<USD_SPEC>;
#[doc = "User system data error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDERR_A {
    #[doc = "0: No user system data error"]
    NoError = 0,
    #[doc = "1: User system data error occurred. Some byte does not match it's inverse code. They both will be forced to 0xFF when read"]
    Error = 1,
}
impl From<USDERR_A> for bool {
    #[inline(always)]
    fn from(variant: USDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDERR` reader - User system data error"]
pub type USDERR_R = crate::BitReader<USDERR_A>;
impl USDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USDERR_A {
        match self.bits {
            false => USDERR_A::NoError,
            true => USDERR_A::Error,
        }
    }
    #[doc = "No user system data error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == USDERR_A::NoError
    }
    #[doc = "User system data error occurred. Some byte does not match it's inverse code. They both will be forced to 0xFF when read"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == USDERR_A::Error
    }
}
#[doc = "FLASH access protection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAP_A {
    #[doc = "0: Flash access protection disabled"]
    Disabled = 0,
    #[doc = "1: Flash access protection enabled"]
    Enabled = 1,
}
impl From<FAP_A> for bool {
    #[inline(always)]
    fn from(variant: FAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAP` reader - FLASH access protection"]
pub type FAP_R = crate::BitReader<FAP_A>;
impl FAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FAP_A {
        match self.bits {
            false => FAP_A::Disabled,
            true => FAP_A::Enabled,
        }
    }
    #[doc = "Flash access protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAP_A::Disabled
    }
    #[doc = "Flash access protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAP_A::Enabled
    }
}
#[doc = "Field `nWDT_ATO_EN` reader - WDT auto enable"]
pub type N_WDT_ATO_EN_R = crate::BitReader;
#[doc = "Field `nDEPSLP_RST` reader - Deepsleep reset"]
pub type N_DEPSLP_RST_R = crate::BitReader;
#[doc = "Field `nSTDBY_RST` reader - Standby reset"]
pub type N_STDBY_RST_R = crate::BitReader;
#[doc = "Field `nBOOT1` reader - boot1"]
pub type N_BOOT1_R = crate::BitReader;
#[doc = "Field `nDEPSLP_WDT` reader - Deepsleep wdt stop count"]
pub type N_DEPSLP_WDT_R = crate::BitReader;
#[doc = "Field `nSTDBY_WDT` reader - Standby wdt stop count"]
pub type N_STDBY_WDT_R = crate::BitReader;
#[doc = "Field `USER_D(0-1)` reader - User data byte %s"]
pub type USER_D_R = crate::FieldReader;
#[doc = "Field `FAP_HL` reader - FAP high level"]
pub type FAP_HL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User system data error"]
    #[inline(always)]
    pub fn usderr(&self) -> USDERR_R {
        USDERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH access protection"]
    #[inline(always)]
    pub fn fap(&self) -> FAP_R {
        FAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT auto enable"]
    #[inline(always)]
    pub fn n_wdt_ato_en(&self) -> N_WDT_ATO_EN_R {
        N_WDT_ATO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deepsleep reset"]
    #[inline(always)]
    pub fn n_depslp_rst(&self) -> N_DEPSLP_RST_R {
        N_DEPSLP_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standby reset"]
    #[inline(always)]
    pub fn n_stdby_rst(&self) -> N_STDBY_RST_R {
        N_STDBY_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - boot1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Deepsleep wdt stop count"]
    #[inline(always)]
    pub fn n_depslp_wdt(&self) -> N_DEPSLP_WDT_R {
        N_DEPSLP_WDT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby wdt stop count"]
    #[inline(always)]
    pub fn n_stdby_wdt(&self) -> N_STDBY_WDT_R {
        N_STDBY_WDT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "User data byte (0-1)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `USER_D0` field.</div>"]
    #[inline(always)]
    pub fn user_d(&self, n: u8) -> USER_D_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        USER_D_R::new(((self.bits >> (n * 8 + 10)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "User data byte (0-1)"]
    #[inline(always)]
    pub fn user_d_iter(&self) -> impl Iterator<Item = USER_D_R> + '_ {
        (0..2).map(move |n| USER_D_R::new(((self.bits >> (n * 8 + 10)) & 0xff) as u8))
    }
    #[doc = "Bits 10:17 - User data byte 0"]
    #[inline(always)]
    pub fn user_d0(&self) -> USER_D_R {
        USER_D_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - User data byte 1"]
    #[inline(always)]
    pub fn user_d1(&self) -> USER_D_R {
        USER_D_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - FAP high level"]
    #[inline(always)]
    pub fn fap_hl(&self) -> FAP_HL_R {
        FAP_HL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USD")
            .field("usderr", &self.usderr())
            .field("fap", &self.fap())
            .field("n_wdt_ato_en", &self.n_wdt_ato_en())
            .field("n_depslp_rst", &self.n_depslp_rst())
            .field("n_stdby_rst", &self.n_stdby_rst())
            .field("n_boot1", &self.n_boot1())
            .field("n_depslp_wdt", &self.n_depslp_wdt())
            .field("n_stdby_wdt", &self.n_stdby_wdt())
            .field("user_d0", &self.user_d0())
            .field("user_d1", &self.user_d1())
            .field("fap_hl", &self.fap_hl())
            .finish()
    }
}
#[doc = "User system data register\n\nYou can [`read`](crate::Reg::read) this register and get [`usd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USD_SPEC;
impl crate::RegisterSpec for USD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usd::R`](R) reader structure"]
impl crate::Readable for USD_SPEC {}
#[doc = "`reset()` method sets USD to value 0x03ff_fffc"]
impl crate::Resettable for USD_SPEC {
    const RESET_VALUE: u32 = 0x03ff_fffc;
}
