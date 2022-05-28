pub fn to_res_name(name: &String) -> String {
    let mut output = String::new();
    let mut i = 0u32;
    let mut prev_char_is_uppercase = false;
    name.chars().for_each(|ch| {
        if ch.is_ascii_alphanumeric() {
            if ch.is_lowercase() || ch.is_numeric() {
                output.push(ch);
                prev_char_is_uppercase = false;
            } else {
                if i > 0 && !prev_char_is_uppercase {
                    output.push('_');
                }
                output.push(ch.to_ascii_lowercase());
                prev_char_is_uppercase = true;
            }
        } else {
            prev_char_is_uppercase = true;
            output.push('_');
        }
        i += 1;
    });
    output
}

#[test]
fn test_camel_case_to_shake_case() {
    let source_name = "ImgAndroidBanner".to_string();
    let res_name = "img_android_banner".to_string();
    assert_eq!(to_res_name(&source_name), res_name);
}

#[test]
fn test_non_alphabetic_symbols() {
    let source_name = "ic_36/fingerprint".to_string();
    let res_name = "ic_36_fingerprint".to_string();
    assert_eq!(to_res_name(&source_name), res_name);
}

#[test]
fn test_uppercase_to_lowercase() {
    let source_name = "img_PRIME_TariffAndroid".to_string();
    let res_name = "img_prime_tariff_android".to_string();
    assert_eq!(to_res_name(&source_name), res_name);
}

#[test]
fn test_complex() {
    let source_name = "ic_24/paper_ID_leftAndroid 100%".to_string();
    let res_name = "ic_24_paper_id_left_android_100_".to_string();
    assert_eq!(to_res_name(&source_name), res_name);
}

#[test]
fn test_nothing_changes() {
    let source_name = "img_promo_banner".to_string();
    let res_name = "img_promo_banner".to_string();
    assert_eq!(to_res_name(&source_name), res_name);
}