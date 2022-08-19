//! # str
//!
//! String utils

mod lookup;

use lazy_regex::{Lazy, Regex};

static HTML_TAG_REGEX: Lazy<Regex> = lazy_regex!(r"<[^>]+>");
/**
 * Matches HTML entities in string
 *
 * - group 2: decimal (maybe)
 * - group 3: literal (e.g. amp, gt, ...) (maybe)
 */
static HTML_ENTITIES_REGEX: Lazy<Regex> = lazy_regex!(r"&(#([0-9]+))?([a-z]+)?;");

/// strip_html
///
/// Strip html tags and entities from string
pub fn strip_html(s: &str) -> String {
    let mut escaped = HTML_TAG_REGEX.replace_all(s, "").to_string();
    let copy = escaped.clone();
    for group in HTML_ENTITIES_REGEX.captures_iter(copy.as_str()) {
        if let Some(mtch) = group.get(2) {
            // Convert mtch to u32
            let replace_with = match mtch.as_str().parse::<u32>() {
                Err(_) => '�',
                Ok(val) => char::from_u32(val).unwrap_or('�'),
            };
            // Get char from decimal
            escaped = escaped.replace(&group[0], replace_with.to_string().as_str());
        } else if let Some(mtch) = group.get(3) {
            let replace_with = lookup::HTML_ENTITIES_TABLE
                .iter()
                .find(|(repr, _)| *repr == mtch.as_str())
                .map(|(_, code)| code)
                .unwrap_or(&"�");
            escaped = escaped.replace(&group[0], replace_with);
        }
    }
    escaped
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_strip_html() {
        assert_eq!(
            strip_html(
                r#"<p><img src="https://images2.corriereobjects.it/methode_image/2021/11/09/Cultura/Foto%20Cltura%20-%20Trattate/Il%20salvataggio%20delle%20vacche%20bis-kWoC-U3300981161016RfG-656x492@Corriere-Web-Sezioni.jpg" title="Polesine, novembre 1951,settant’anni fa l’alluvione che travolse il Veneto" alt="Polesine, novembre 1951 />Hello</p> World!"#
            ),
            "Hello World!"
        );
        assert_eq!(
            strip_html(r#"Hello, &lt;World&gt;&#33;"#),
            "Hello, <World>!"
        );
    }
}
