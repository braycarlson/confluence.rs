pub const ACCOUNT_ID_BYTES_MAX: u32 = 128;
pub const ATTACHMENT_BYTES_MAX: u64 = 8 * 1024 * 1024 * 1024;
pub const BODY_BYTES_MAX: u64 = 1024 * 1024 * 1024;
pub(crate) const ERROR_EXCERPT_BYTES_MAX: u64 = 500;
pub const PAGE_COUNT_MAX: u32 = 1_000_000;
pub const SPACE_KEY_BYTES_MAX: u32 = 255;
pub const TOKEN_BYTES_MAX: u32 = 2_048;
pub const UPLOAD_BYTES_MAX: u64 = 8 * 1024 * 1024 * 1024;
pub const URL_BYTES_MAX: u32 = 8_192;
pub const WAIT_MILLIS_MAX: u64 = 1_170_000;

const _: () = assert!(
    (ACCOUNT_ID_BYTES_MAX + SPACE_KEY_BYTES_MAX) * 3 < URL_BYTES_MAX,
    "a creator and a space key, each fully escaped, fit one URL",
);

const _: () = assert!(
    ACCOUNT_ID_BYTES_MAX < SPACE_KEY_BYTES_MAX,
    "a personal space key, a tilde before an account id, fits the bound",
);

const _: () = assert!(TOKEN_BYTES_MAX * 3 < URL_BYTES_MAX, "an escaped page token fits one URL");

const _: () = assert!(
    UPLOAD_BYTES_MAX >= 64 * 104_857_600,
    "a guard sits far above the 100 MB a Confluence Cloud site allows per file by default",
);

const _: () = assert!(
    ATTACHMENT_BYTES_MAX >= 64 * 104_857_600,
    "a guard sits far above the 100 MB a Confluence Cloud site allows per file by default",
);

const _: () = assert!(
    BODY_BYTES_MAX >= 250 * 4_000_000,
    "a guard sits above a page of 250 results that each carry 4 MB of body",
);

const _: () = assert!(
    PAGE_COUNT_MAX >= 1_000_000,
    "a walk of 25 million items at 25 a page fits its budget",
);
