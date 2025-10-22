//! 入力値の検証関数
//!
//! このモジュールは、ユーザー入力やファイル名の検証を行う関数を提供します。

/// 文字列がスネークケース形式かどうかを判定します。
///
/// スネークケースとは、小文字の英字・数字・アンダースコアのみで構成され、
/// 先頭が小文字または数字である形式です。
///
/// # Examples
///
/// ```
/// use processor::validation::is_snake_case;
///
/// assert!(is_snake_case("valid_name"));
/// assert!(is_snake_case("name123"));
/// assert!(is_snake_case("123name"));
/// assert!(!is_snake_case("InvalidName"));
/// assert!(!is_snake_case("invalid-name"));
/// assert!(!is_snake_case("_invalid"));
/// ```
pub fn is_snake_case(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => (),
        _ => return false,
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    true
}

/// 文字列がスネークケース形式であることを検証します。
///
/// スネークケースでない場合はエラーを返します。
///
/// # Errors
///
/// 文字列がスネークケース形式でない場合、エラーメッセージを含む
/// `anyhow::Error` を返します。
///
/// # Examples
///
/// ```
/// use processor::validation::should_snake_case;
///
/// assert!(should_snake_case("valid_name").is_ok());
/// assert!(should_snake_case("InvalidName").is_err());
/// ```
pub fn should_snake_case(s: &str) -> anyhow::Result<()> {
    if is_snake_case(s) {
        Ok(())
    } else {
        anyhow::bail!("'{s}'はスネークケースで指定してください。");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_snake_case_valid() {
        assert!(is_snake_case("valid_name"));
        assert!(is_snake_case("valid"));
        assert!(is_snake_case("name_with_123"));
        assert!(is_snake_case("123_start"));
        assert!(is_snake_case("a"));
        assert!(is_snake_case("abc_def_ghi"));
    }

    #[test]
    fn test_is_snake_case_invalid() {
        assert!(!is_snake_case("InvalidName"));
        assert!(!is_snake_case("invalid-name"));
        assert!(!is_snake_case("_invalid"));
        assert!(!is_snake_case("invalid_Name"));
        assert!(!is_snake_case("INVALID"));
        assert!(!is_snake_case(""));
        assert!(!is_snake_case("invalid!"));
    }

    #[test]
    fn test_should_snake_case_valid() {
        assert!(should_snake_case("valid_name").is_ok());
        assert!(should_snake_case("test123").is_ok());
    }

    #[test]
    fn test_should_snake_case_invalid() {
        assert!(should_snake_case("InvalidName").is_err());
        assert!(should_snake_case("_invalid").is_err());
        let err = should_snake_case("BadName").unwrap_err();
        assert!(err.to_string().contains("スネークケース"));
    }
}
