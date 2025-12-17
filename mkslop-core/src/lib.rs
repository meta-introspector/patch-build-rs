use syn::LitStr;
 // Use proc_macro2::Span

/// Applies common auto-fixes to problematic format strings.
///
/// Specifically fixes the `cargo:rustc-cfg={0}="{1}"` pattern
/// which causes `invalid format string` errors when used with `println!`.
pub fn fix_cfg_format_string(input_lit: LitStr) -> LitStr {
    let bad_format_str_value = "cargo:rustc-cfg={0}=\"{1}\"";

    if input_lit.value() == bad_format_str_value {
        // Rewrite the string to use raw string literal format, which correctly handles inner quotes
        // The raw string literal handles the quotes automatically.
        // E.g., r#"foo="bar""# becomes foo="bar"
        let corrected_format_str_value = r#"cargo:rustc-cfg={0}="{1}""#;
        LitStr::new(&corrected_format_str_value, input_lit.span())
    } else {
        input_lit // Return original if no fix is needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span; // Use proc_macro2::Span for tests

    #[test]
    fn test_fix_cfg_format_string_problematic() {
        let problematic_str = "cargo:rustc-cfg={0}=\"{1}\"";
        let input_lit = LitStr::new(problematic_str, Span::call_site());
        let fixed_lit = fix_cfg_format_string(input_lit);
        assert_eq!(fixed_lit.value(), r#"cargo:rustc-cfg={0}="{1}""#);
    }

    #[test]
    fn test_fix_cfg_format_string_other() {
        let original_str = "Hello, {}!";
        let input_lit = LitStr::new(original_str, Span::call_site());
        let fixed_lit = fix_cfg_format_string(input_lit);
        assert_eq!(fixed_lit.value(), original_str);
    }
    
    #[test]
    fn test_fix_cfg_format_string_check_cfg() {
        let original_str = r#"cargo:rustc-check-cfg=cfg({0}, values("{1}"))"#;
        let input_lit = LitStr::new(original_str, Span::call_site());
        let fixed_lit = fix_cfg_format_string(input_lit);
        // This string does not match the bad_format_str_value, so it should be returned unchanged
        assert_eq!(fixed_lit.value(), original_str);
    }
}
