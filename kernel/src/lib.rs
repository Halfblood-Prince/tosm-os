#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub const fn boot_banner() -> &'static str {
    "tosm-os: kernel entry reached"
}

#[cfg(test)]
mod tests {
    use super::boot_banner;

    #[test]
    fn boot_banner_is_stable() {
        assert_eq!(boot_banner(), "tosm-os: kernel entry reached");
    }
}
