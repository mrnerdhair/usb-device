#![allow(non_upper_case_globals, non_camel_case_types)]

use num_enum::TryFromPrimitive;

impl From<&LangID> for u16 {
    fn from(lang_id: &LangID) -> Self {
        *lang_id as u16
    }
}

impl From<LangID> for u16 {
    fn from(lang_id: LangID) -> Self {
        lang_id as u16
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum LangID {
    AR = 0x0001,
    BG = 0x0002,
    CA = 0x0003,
    ZH_HANS = 0x0004,
    CS = 0x0005,
    DA = 0x0006,
    DE = 0x0007,
    EL = 0x0008,
    EN = 0x0009,
    ES = 0x000A,
    FI = 0x000B,
    FR = 0x000C,
    HE = 0x000D,
    HU = 0x000E,
    IS = 0x000F,
    IT = 0x0010,
    JA = 0x0011,
    KO = 0x0012,
    NL = 0x0013,
    NO = 0x0014,
    PL = 0x0015,
    PT = 0x0016,
    RM = 0x0017,
    RO = 0x0018,
    RU = 0x0019,
    HR = 0x001A,
    SK = 0x001B,
    SQ = 0x001C,
    SV = 0x001D,
    TH = 0x001E,
    TR = 0x001F,
    UR = 0x0020,
    ID = 0x0021,
    UK = 0x0022,
    BE = 0x0023,
    SL = 0x0024,
    ET = 0x0025,
    LV = 0x0026,
    LT = 0x0027,
    TG = 0x0028,
    FA = 0x0029,
    VI = 0x002A,
    HY = 0x002B,
    AZ = 0x002C,
    EU = 0x002D,
    HSB = 0x002E,
    MK = 0x002F,
    ST = 0x0030,
    TS = 0x0031,
    TN = 0x0032,
    VE = 0x0033,
    XH = 0x0034,
    ZU = 0x0035,
    AF = 0x0036,
    KA = 0x0037,
    FO = 0x0038,
    HI = 0x0039,
    MT = 0x003A,
    SE = 0x003B,
    GA = 0x003C,
    YI = 0x003D,
    MS = 0x003E,
    KK = 0x003F,
    KY = 0x0040,
    SW = 0x0041,
    TK = 0x0042,
    UZ = 0x0043,
    TT = 0x0044,
    BN = 0x0045,
    PA = 0x0046,
    GU = 0x0047,
    OR = 0x0048,
    TA = 0x0049,
    TE = 0x004A,
    KN = 0x004B,
    ML = 0x004C,
    AS = 0x004D,
    MR = 0x004E,
    SA = 0x004F,
    MN = 0x0050,
    BO = 0x0051,
    CY = 0x0052,
    KM = 0x0053,
    LO = 0x0054,
    MY = 0x0055,
    GL = 0x0056,
    KOK = 0x0057,
    MNI = 0x0058,
    SD = 0x0059,
    SYR = 0x005A,
    SI = 0x005B,
    CHR = 0x005C,
    IU = 0x005D,
    AM = 0x005E,
    TZM = 0x005F,
    KS = 0x0060,
    NE = 0x0061,
    FY = 0x0062,
    PS = 0x0063,
    FIL = 0x0064,
    DV = 0x0065,
    BIN = 0x0066,
    FF = 0x0067,
    HA = 0x0068,
    IBB = 0x0069,
    YO = 0x006A,
    QUZ = 0x006B,
    NSO = 0x006C,
    BA = 0x006D,
    LB = 0x006E,
    KL = 0x006F,
    IG = 0x0070,
    KR = 0x0071,
    OM = 0x0072,
    TI = 0x0073,
    GN = 0x0074,
    HAW = 0x0075,
    LA = 0x0076,
    SO = 0x0077,
    II = 0x0078,
    PAP = 0x0079,
    ARN = 0x007A,
    MOH = 0x007C,
    BR = 0x007E,
    UG = 0x0080,
    MI = 0x0081,
    OC = 0x0082,
    CO = 0x0083,
    GSW = 0x0084,
    SAH = 0x0085,
    QUT = 0x0086,
    RW = 0x0087,
    WO = 0x0088,
    PRS = 0x008C,
    GD = 0x0091,
    KU = 0x0092,
    QUC = 0x0093,
    AR_SA = 0x0401,
    BG_BG = 0x0402,
    CA_ES = 0x0403,
    ZH_TW = 0x0404,
    CS_CZ = 0x0405,
    DA_DK = 0x0406,
    DE_DE = 0x0407,
    EL_GR = 0x0408,
    EN_US = 0x0409,
    ES_ES_TRADNL = 0x040A,
    FI_FI = 0x040B,
    FR_FR = 0x040C,
    HE_IL = 0x040D,
    HU_HU = 0x040E,
    IS_IS = 0x040F,
    IT_IT = 0x0410,
    JA_JP = 0x0411,
    KO_KR = 0x0412,
    NL_NL = 0x0413,
    NB_NO = 0x0414,
    PL_PL = 0x0415,
    PT_BR = 0x0416,
    RM_CH = 0x0417,
    RO_RO = 0x0418,
    RU_RU = 0x0419,
    HR_HR = 0x041A,
    SK_SK = 0x041B,
    SQ_AL = 0x041C,
    SV_SE = 0x041D,
    TH_TH = 0x041E,
    TR_TR = 0x041F,
    UR_PK = 0x0420,
    ID_ID = 0x0421,
    UK_UA = 0x0422,
    BE_BY = 0x0423,
    SL_SI = 0x0424,
    ET_EE = 0x0425,
    LV_LV = 0x0426,
    LT_LT = 0x0427,
    TG_CYRL_TJ = 0x0428,
    FA_IR = 0x0429,
    VI_VN = 0x042A,
    HY_AM = 0x042B,
    AZ_LATN_AZ = 0x042C,
    EU_ES = 0x042D,
    HSB_DE = 0x042E,
    MK_MK = 0x042F,
    ST_ZA = 0x0430,
    TS_ZA = 0x0431,
    TN_ZA = 0x0432,
    VE_ZA = 0x0433,
    XH_ZA = 0x0434,
    ZU_ZA = 0x0435,
    AF_ZA = 0x0436,
    KA_GE = 0x0437,
    FO_FO = 0x0438,
    HI_IN = 0x0439,
    MT_MT = 0x043A,
    SE_NO = 0x043B,
    YI_001 = 0x043D,
    MS_MY = 0x043E,
    KK_KZ = 0x043F,
    KY_KG = 0x0440,
    SW_KE = 0x0441,
    TK_TM = 0x0442,
    UZ_LATN_UZ = 0x0443,
    TT_RU = 0x0444,
    BN_IN = 0x0445,
    PA_IN = 0x0446,
    GU_IN = 0x0447,
    OR_IN = 0x0448,
    TA_IN = 0x0449,
    TE_IN = 0x044A,
    KN_IN = 0x044B,
    ML_IN = 0x044C,
    AS_IN = 0x044D,
    MR_IN = 0x044E,
    SA_IN = 0x044F,
    MN_MN = 0x0450,
    BO_CN = 0x0451,
    CY_GB = 0x0452,
    KM_KH = 0x0453,
    LO_LA = 0x0454,
    MY_MM = 0x0455,
    GL_ES = 0x0456,
    KOK_IN = 0x0457,
    MNI_IN = 0x0458,
    SD_DEVA_IN = 0x0459,
    SYR_SY = 0x045A,
    SI_LK = 0x045B,
    CHR_CHER_US = 0x045C,
    IU_CANS_CA = 0x045D,
    AM_ET = 0x045E,
    TZM_ARAB_MA = 0x045F,
    KS_ARAB = 0x0460,
    NE_NP = 0x0461,
    FY_NL = 0x0462,
    PS_AF = 0x0463,
    FIL_PH = 0x0464,
    DV_MV = 0x0465,
    BIN_NG = 0x0466,
    FF_NG__FF_LATN_NG = 0x0467,
    HA_LATN_NG = 0x0468,
    IBB_NG = 0x0469,
    YO_NG = 0x046A,
    QUZ_BO = 0x046B,
    NSO_ZA = 0x046C,
    BA_RU = 0x046D,
    LB_LU = 0x046E,
    KL_GL = 0x046F,
    IG_NG = 0x0470,
    KR_LATN_NG = 0x0471,
    OM_ET = 0x0472,
    TI_ET = 0x0473,
    GN_PY = 0x0474,
    HAW_US = 0x0475,
    LA_VA = 0x0476,
    SO_SO = 0x0477,
    II_CN = 0x0478,
    PAP_029 = 0x0479,
    ARN_CL = 0x047A,
    MOH_CA = 0x047C,
    BR_FR = 0x047E,
    UG_CN = 0x0480,
    MI_NZ = 0x0481,
    OC_FR = 0x0482,
    CO_FR = 0x0483,
    GSW_FR = 0x0484,
    SAH_RU = 0x0485,
    QUT_GT = 0x0486,
    RW_RW = 0x0487,
    WO_SN = 0x0488,
    PRS_AF = 0x048C,
    PLT_MG = 0x048D,
    ZH_YUE_HK = 0x048E,
    TDD_TALE_CN = 0x048F,
    KHB_TALU_CN = 0x0490,
    GD_GB = 0x0491,
    KU_ARAB_IQ = 0x0492,
    QUC_CO = 0x0493,
    QPS_PLOC = 0x0501,
    QPS_PLOCA = 0x05FE,
    AR_IQ = 0x0801,
    CA_ES_VALENCIA = 0x0803,
    ZH_CN = 0x0804,
    DE_CH = 0x0807,
    EN_GB = 0x0809,
    ES_MX = 0x080A,
    FR_BE = 0x080C,
    IT_CH = 0x0810,
    JA_PLOC_JP = 0x0811,
    NL_BE = 0x0813,
    NN_NO = 0x0814,
    PT_PT = 0x0816,
    RO_MD = 0x0818,
    RU_MD = 0x0819,
    SR_LATN_CS = 0x081A,
    SV_FI = 0x081D,
    UR_IN = 0x0820,
    AZ_CYRL_AZ = 0x082C,
    DSB_DE = 0x082E,
    TN_BW = 0x0832,
    SE_SE = 0x083B,
    GA_IE = 0x083C,
    MS_BN = 0x083E,
    KK_LATN_KZ = 0x083F,
    UZ_CYRL_UZ = 0x0843,
    BN_BD = 0x0845,
    PA_ARAB_PK = 0x0846,
    TA_LK = 0x0849,
    MN_MONG_CN = 0x0850,
    BO_BT = 0x0851,
    SD_ARAB_PK = 0x0859,
    IU_LATN_CA = 0x085D,
    TZM_LATN_DZ = 0x085F,
    KS_DEVA_IN = 0x0860,
    NE_IN = 0x0861,
    FF_LATN_SN = 0x0867,
    QUZ_EC = 0x086B,
    TI_ER = 0x0873,
    QPS_PLOCM = 0x09FF,
    LOCALE_CUSTOM_USER_DEFAULT = 0x0C00,
    AR_EG = 0x0C01,
    ZH_HK = 0x0C04,
    DE_AT = 0x0C07,
    EN_AU = 0x0C09,
    ES_ES = 0x0C0A,
    FR_CA = 0x0C0C,
    SR_CYRL_CS = 0x0C1A,
    SE_FI = 0x0C3B,
    MN_MONG_MN = 0x0C50,
    DZ_BT = 0x0C51,
    TMZ_MA = 0x0C5F,
    QUZ_PE = 0x0C6b,
    LOCALE_CUSTOM_UNSPECIFIED = 0x1000,
    AR_LY = 0x1001,
    ZH_SG = 0x1004,
    DE_LU = 0x1007,
    EN_CA = 0x1009,
    ES_GT = 0x100A,
    FR_CH = 0x100C,
    HR_BA = 0x101A,
    SMJ_NO = 0x103B,
    TZM_TFNG_MA = 0x105F,
    AR_DZ = 0x1401,
    ZH_MO = 0x1404,
    DE_LI = 0x1407,
    EN_NZ = 0x1409,
    ES_CR = 0x140A,
    FR_LU = 0x140C,
    BS_LATN_BA = 0x141A,
    SMJ_SE = 0x143B,
    AR_MA = 0x1801,
    EN_IE = 0x1809,
    ES_PA = 0x180A,
    FR_MC = 0x180C,
    SR_LATN_BA = 0x181A,
    SMA_NO = 0x183B,
    AR_TN = 0x1C01,
    EN_ZA = 0x1C09,
    ES_DO = 0x1C0A,
    FR_029 = 0x1C0C,
    SR_CYRL_BA = 0x1C1A,
    SMA_SE = 0x1C3B,
    AR_OM = 0x2001,
    EN_JM = 0x2009,
    ES_VE = 0x200A,
    FR_RE = 0x200C,
    BS_CYRL_BA = 0x201A,
    SMS_FI = 0x203B,
    AR_YE = 0x2401,
    EN_029 = 0x2409,
    ES_CO = 0x240A,
    FR_CD = 0x240C,
    SR_LATN_RS = 0x241A,
    SMN_FI = 0x243B,
    AR_SY = 0x2801,
    EN_BZ = 0x2809,
    ES_PE = 0x280A,
    FR_SN = 0x280C,
    SR_CYRL_RS = 0x281A,
    AR_JO = 0x2C01,
    EN_TT = 0x2C09,
    ES_AR = 0x2C0A,
    FR_CM = 0x2C0C,
    SR_LATN_ME = 0x2C1A,
    AR_LB = 0x3001,
    EN_ZW = 0x3009,
    ES_EC = 0x300A,
    FR_CI = 0x300C,
    SR_CYRL_ME = 0x301A,
    AR_KW = 0x3401,
    EN_PH = 0x3409,
    ES_CL = 0x340A,
    FR_ML = 0x340C,
    AR_AE = 0x3801,
    EN_ID = 0x3809,
    ES_UY = 0x380A,
    FR_MA = 0x380C,
    AR_BH = 0x3C01,
    EN_HK = 0x3C09,
    ES_PY = 0x3C0A,
    FR_HT = 0x3C0C,
    AR_QA = 0x4001,
    EN_IN = 0x4009,
    ES_BO = 0x400A,
    AR_PLOC_SA = 0x4401,
    EN_MY = 0x4409,
    ES_SV = 0x440A,
    AR_145 = 0x4801,
    EN_SG = 0x4809,
    ES_HN = 0x480A,
    EN_AE = 0x4C09,
    ES_NI = 0x4C0A,
    EN_BH = 0x5009,
    ES_PR = 0x500A,
    EN_EG = 0x5409,
    ES_US = 0x540A,
    EN_JO = 0x5809,
    ES_419 = 0x580A,
    EN_KW = 0x5C09,
    ES_CU = 0x5C0A,
    EN_TR = 0x6009,
    EN_YE = 0x6409,
    BS_CYRL = 0x641A,
    BS_LATN = 0x681A,
    SR_CYRL = 0x6C1A,
    SR_LATN = 0x701A,
    SMN = 0x703B,
    AZ_CYRL = 0x742C,
    SMS = 0x743B,
    ZH = 0x7804,
    NN = 0x7814,
    BS = 0x781A,
    AZ_LATN = 0x782C,
    SMA = 0x783B,
    KK_CYRL = 0x783F,
    UZ_CYRL = 0x7843,
    MN_CYRL = 0x7850,
    IU_CANS = 0x785D,
    TZM_TFNG = 0x785F,
    ZH_HANT = 0x7C04,
    NB = 0x7C14,
    SR = 0x7C1A,
    TG_CYRL = 0x7C28,
    DSB = 0x7C2E,
    SMJ = 0x7C3B,
    KK_LATN = 0x7C3F,
    UZ_LATN = 0x7C43,
    PA_ARAB = 0x7C46,
    MN_MONG = 0x7C50,
    SD_ARAB = 0x7C59,
    CHR_CHER = 0x7C5C,
    IU_LATN = 0x7C5D,
    TZM_LATN = 0x7C5F,
    FF_LATN = 0x7C67,
    HA_LATN = 0x7C68,
    KU_ARAB = 0x7C92,
    FR_015 = 0xE40C,
}
