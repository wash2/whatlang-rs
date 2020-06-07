use hashbrown::HashMap;

use crate::constants::{MAX_TOTAL_DISTANCE, MAX_TRIGRAM_DISTANCE};
use crate::info::Info;
use crate::lang::*;
use crate::options::{List, Options};
use crate::script::*;
use crate::trigrams::*;
use std::cmp::Reverse;

/// Detect a language and a script by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect, Lang, Script};
///
/// let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
/// assert_eq!(info.lang(), Lang::Epo);
/// assert_eq!(info.script(), Script::Latin);
/// ```
pub fn detect(text: &str) -> Option<Info> {
    detect_with_options(text, &Options::default())
}

/// Detect only a language by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect_lang, Lang};
/// let lang = detect_lang("There is no reason not to learn Esperanto.").unwrap();
/// assert_eq!(lang, Lang::Eng);
/// ```
pub fn detect_lang(text: &str) -> Option<Lang> {
    detect(text).map(|info| info.lang)
}

pub fn detect_langs(text: &str) -> Vec<Info> {
    detect_langs_with_options(text, &Options::default())
}

pub fn detect_langs_with_options(text: &str, options: &Options) -> Vec<Info> {
    let divided_text = divide_text_by_script(text);
    let mut langs = divided_text.iter()
    .fold(
        HashMap::new(),
        |mut acc: HashMap<Lang, Info>, text_and_script| -> HashMap<Lang, Info> {
            let (text_slice, script) = text_and_script;

        let new_langs = match script {
            Script::Latin => detect_langs_in_profiles(*text_slice, options, LATIN_LANGS, *script),
            Script::Cyrillic => detect_langs_in_profiles(*text_slice, options, CYRILLIC_LANGS, *script),
            Script::Devanagari => detect_langs_in_profiles(*text_slice, options, DEVANAGARI_LANGS, *script),
            Script::Hebrew => detect_langs_in_profiles(*text_slice, options, HEBREW_LANGS, *script),
            Script::Ethiopic => detect_langs_in_profiles(*text_slice, options, ETHIOPIC_LANGS, *script),
            Script::Arabic => detect_langs_in_profiles(*text_slice, options, ARABIC_LANGS, *script),
            Script::Mandarin => detect_multiple_mandarin_japanese(*script, options),
            Script::Bengali => vec![Info {
                lang: Lang::Ben,
                script: *script,
                confidence: 1.0
            }],
            Script::Hangul => vec![Info {
                lang: Lang::Kor,
                script: *script,
                confidence: 1.0
            }],
            Script::Georgian => vec![Info {
                lang: Lang::Kat,
                script: *script,
                confidence: 1.0
            }],
            Script::Greek => vec![Info {
                lang: Lang::Ell,
                script: *script,
                confidence: 1.0
            }],
            Script::Kannada => vec![Info {
                lang: Lang::Kan,
                script: *script,
                confidence: 1.0
            }],
            Script::Tamil => vec![Info {
                lang: Lang::Tam,
                script: *script,
                confidence: 1.0
            }],
            Script::Thai => vec![Info {
                lang: Lang::Tha,
                script: *script,
                confidence: 1.0
            }],
            Script::Gujarati => vec![Info {
                lang: Lang::Guj,
                script: *script,
                confidence: 1.0
            }],
            Script::Gurmukhi => vec![Info {
                lang: Lang::Pan,
                script: *script,
                confidence: 1.0
            }],
            Script::Telugu => vec![Info {
                lang: Lang::Tel,
                script: *script,
                confidence: 1.0
            }],
            Script::Malayalam => vec![Info {
                lang: Lang::Mal,
                script: *script,
                confidence: 1.0
            }],
            Script::Oriya => vec![Info {
                lang: Lang::Ori,
                script: *script,
                confidence: 1.0
            }],
            Script::Myanmar => vec![Info {
                lang: Lang::Mya,
                script: *script,
                confidence: 1.0
            }],
            Script::Sinhala => vec![Info {
                lang: Lang::Sin,
                script: *script,
                confidence: 1.0
            }],
            Script::Khmer => vec![Info {
                lang: Lang::Khm,
                script: *script,
                confidence: 1.0
            }],
            Script::Katakana | Script::Hiragana => vec![Info {
                lang: Lang::Jpn,
                script: *script,
                confidence: 1.0
            }],
        };
        for info in new_langs {
            if !acc.contains_key(&info.lang) {
                acc.insert(info.lang, info);
            }
            else {
                if acc.get(&info.lang).unwrap().confidence < info.confidence {
                    acc.insert(info.lang, info);
                }
            }
        }
        acc
    }).values()
    .filter_map(|&info| {
        if info.confidence > options.confidence_threshold {
            return Some(info);
        }
        None
    })
    .collect::<Vec<Info>>();
    langs.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    langs

}

pub fn detect_lang_with_options(text: &str, options: &Options) -> Option<Lang> {
    detect_with_options(text, options).map(|info| info.lang)
}

pub fn detect_with_options(text: &str, options: &Options) -> Option<Info> {
    detect_script(text).and_then(|script| {
        detect_lang_based_on_script(text, options, script).map(|(lang, confidence)| Info {
            lang,
            script,
            confidence,
        })
    })
}

fn detect_lang_based_on_script(
    text: &str,
    options: &Options,
    script: Script,
) -> Option<(Lang, f64)> {
    match script {
        Script::Latin => detect_lang_in_profiles(text, options, LATIN_LANGS),
        Script::Cyrillic => detect_lang_in_profiles(text, options, CYRILLIC_LANGS),
        Script::Devanagari => detect_lang_in_profiles(text, options, DEVANAGARI_LANGS),
        Script::Hebrew => detect_lang_in_profiles(text, options, HEBREW_LANGS),
        Script::Ethiopic => detect_lang_in_profiles(text, options, ETHIOPIC_LANGS),
        Script::Arabic => detect_lang_in_profiles(text, options, ARABIC_LANGS),
        Script::Mandarin => detect_mandarin_japanese(options),
        Script::Bengali => Some((Lang::Ben, 1.0)),
        Script::Hangul => Some((Lang::Kor, 1.0)),
        Script::Georgian => Some((Lang::Kat, 1.0)),
        Script::Greek => Some((Lang::Ell, 1.0)),
        Script::Kannada => Some((Lang::Kan, 1.0)),
        Script::Tamil => Some((Lang::Tam, 1.0)),
        Script::Thai => Some((Lang::Tha, 1.0)),
        Script::Gujarati => Some((Lang::Guj, 1.0)),
        Script::Gurmukhi => Some((Lang::Pan, 1.0)),
        Script::Telugu => Some((Lang::Tel, 1.0)),
        Script::Malayalam => Some((Lang::Mal, 1.0)),
        Script::Oriya => Some((Lang::Ori, 1.0)),
        Script::Myanmar => Some((Lang::Mya, 1.0)),
        Script::Sinhala => Some((Lang::Sin, 1.0)),
        Script::Khmer => Some((Lang::Khm, 1.0)),
        Script::Katakana | Script::Hiragana => Some((Lang::Jpn, 1.0)),
    }
}

fn detect_langs_in_profiles(
    text: &str,
    options: &Options,
    lang_profile_list: LangProfileList,
    script: Script
) -> Vec<Info> {
    let mut lang_distances: Vec<(Lang, u32)> = Vec::with_capacity(lang_profile_list.len());
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        match options.list {
            Some(List::White(ref whitelist)) if !whitelist.contains(lang) => continue,
            Some(List::Black(ref blacklist)) if blacklist.contains(lang) => continue,
            _ => {}
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang), MAX_TOTAL_DISTANCE - dist));
    }

    // Sort languages by distance
    lang_distances.sort_by_key(|key| Reverse(key.1));

    // Return None if lang_distances is empty
    // Return the only language with is_reliable=true if there is only 1 item
    if lang_distances.len() < 2 {
        return vec![Info {
            lang: lang_distances.first().unwrap().0,
            script: script,
            confidence: 1.0
        }];
    }

    // Calculate is_reliable based on:
    // - number of unique trigrams in the text
    // - rate (diff between score of the first and second languages)
    //
    let lang_dist1 = lang_distances[0];
    let lang_dist2 = lang_distances[1];
    let score1 = lang_dist1.1;
    let score2 = lang_dist2.1;

    if score1 == 0 {
        // If score1 is 0, score2 is 0 as well, because array is sorted.
        // Therefore there is no language to return.
        return Vec::new();
    } else if score2 == 0 {
        // If score2 is 0, return first language, to prevent division by zero in the rate formula.
        // In this case confidence is calculated by another formula.
        // At this point there are two options:
        // * Text contains random characters that accidentally match trigrams of one of the languages
        // * Text really matches one of the languages.
        //
        // Number 500.0 is based on experiments and common sense expectations.
        let mut confidence = f64::from(score1) / 500.0;
        if confidence > 1.0 {
            confidence = 1.0;
        }
        return vec![Info {
            lang: lang_dist1.0,
            script: script,
            confidence: confidence
        }];
    }

    // normalize distances and return
    let min = lang_distances.last().unwrap().1;

    lang_distances.iter()
    .map(|&(lang, dist)| Info {
        lang: lang.clone(),
        script: script,
        confidence: f64::from(dist - min) / f64::from(score1)
    })
    .into_iter()
    .collect::<Vec<Info>>()
}

fn detect_lang_in_profiles(
    text: &str,
    options: &Options,
    lang_profile_list: LangProfileList,
) -> Option<(Lang, f64)> {
    let mut lang_distances: Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        match options.list {
            Some(List::White(ref whitelist)) if !whitelist.contains(lang) => continue,
            Some(List::Black(ref blacklist)) if blacklist.contains(lang) => continue,
            _ => {}
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang), dist));
    }

    // Sort languages by distance
    lang_distances.sort_by_key(|key| key.1);

    // Return None if lang_distances is empty
    // Return the only language with is_reliable=true if there is only 1 item
    if lang_distances.len() < 2 {
        return lang_distances.first().map(|pair| (pair.0, 1.0));
    }

    // Calculate is_reliable based on:
    // - number of unique trigrams in the text
    // - rate (diff between score of the first and second languages)
    //
    let lang_dist1 = lang_distances[0];
    let lang_dist2 = lang_distances[1];
    let score1 = MAX_TOTAL_DISTANCE - lang_dist1.1;
    let score2 = MAX_TOTAL_DISTANCE - lang_dist2.1;

    if score1 == 0 {
        // If score1 is 0, score2 is 0 as well, because array is sorted.
        // Therefore there is no language to return.
        return None;
    } else if score2 == 0 {
        // If score2 is 0, return first language, to prevent division by zero in the rate formula.
        // In this case confidence is calculated by another formula.
        // At this point there are two options:
        // * Text contains random characters that accidentally match trigrams of one of the languages
        // * Text really matches one of the languages.
        //
        // Number 500.0 is based on experiments and common sense expectations.
        let mut confidence = f64::from(score1) / 500.0;
        if confidence > 1.0 {
            confidence = 1.0;
        }
        return Some((lang_dist1.0, confidence));
    }

    let rate = f64::from(score1 - score2) / f64::from(score2);

    // Hyperbola function. Everything that is above the function has confidence = 1.0
    // If rate is below, confidence is calculated proportionally.
    // Numbers 12.0 and 0.05 are obtained experimentally, so the function represents common sense.
    //
    let confident_rate = (12.0 / trigrams.len() as f64) + 0.05;
    let confidence = if rate > confident_rate {
        1.0
    } else {
        rate / confident_rate
    };

    Some((lang_dist1.0, confidence))
}

fn calculate_distance(lang_trigrams: LangProfile, text_trigrams: &HashMap<String, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(trigram) {
            Some(&n) => (n as i32 - i as i32).abs() as u32,
            None => MAX_TRIGRAM_DISTANCE,
        };
        total_dist += dist;
    }
    total_dist
}

fn detect_multiple_mandarin_japanese(script: Script, options: &Options) -> Vec<Info> {
    let mut langs = Vec::new();
    let _a = match options.list {
        Some(List::White(ref whitelist)) => {
            if whitelist.contains(&Lang::Jpn) {
                langs.push(Info {
                    lang: Lang::Jpn,
                    script: script,
                    confidence: 1.0
                });
            } 
            if whitelist.contains(&Lang::Cmn) {
                langs.push(Info {
                    lang: Lang::Cmn,
                    script: script,
                    confidence: 1.0
                });
            }
        }
        Some(List::Black(ref blacklist)) => {
            if !blacklist.contains(&Lang::Jpn) {
                langs.push(Info {
                    lang: Lang::Jpn,
                    script: script,
                    confidence: 1.0
                });
            } 
            if !blacklist.contains(&Lang::Cmn) {
                langs.push(Info {
                    lang: Lang::Cmn,
                    script: script,
                    confidence: 1.0
                });
            }
        }
        _ => {
            langs.push(Info {
                lang: Lang::Jpn,
                script: script,
                confidence: 1.0
            });
            langs.push(Info {
                lang: Lang::Cmn,
                script: script,
                confidence: 1.0
            });
        },
    };
    langs
}

fn detect_mandarin_japanese(options: &Options) -> Option<(Lang, f64)> {
    match options.list {
        Some(List::White(ref whitelist)) => {
            if whitelist.contains(&Lang::Jpn) && !whitelist.contains(&Lang::Cmn) {
                Some((Lang::Jpn, 1.0))
            } else if whitelist.contains(&Lang::Cmn) {
                Some((Lang::Cmn, 1.0))
            } else {
                None
            }
        }
        Some(List::Black(ref blacklist)) => {
            if blacklist.contains(&Lang::Cmn) && !blacklist.contains(&Lang::Jpn) {
                Some((Lang::Jpn, 1.0))
            } else if !blacklist.contains(&Lang::Cmn) {
                Some((Lang::Cmn, 1.0))
            } else {
                None
            }
        }
        _ => Some((Lang::Cmn, 1.0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::Script;

    #[test]
    fn test_detect_spanish() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let output = detect(text);
        assert_eq!(output.is_some(), true);

        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Spa);
        assert_eq!(info.script, Script::Latin);
    }

    #[test]
    fn test_detect_lang_ukrainian() {
        let text = "Та нічого, все нормально. А в тебе як?";
        assert_eq!(detect_lang(text), Some(Lang::Ukr));
    }

    #[test]
    fn test_detect_with_options_with_blacklist() {
        let text = "I am begging pardon";
        // without blacklist
        let output = detect_with_options(text, &Options::default());
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Tgl);

        // with blacklist
        let blacklist = vec![
            Lang::Tgl,
            Lang::Jav,
            Lang::Nld,
            Lang::Uzb,
            Lang::Swe,
            Lang::Nob,
            Lang::Ceb,
            Lang::Ilo,
        ];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Eng);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_none() {
        let text = "האקדמיה ללשון העברית";

        // All languages with Hebrew script are in blacklist, so result must be None
        let blacklist = vec![Lang::Heb, Lang::Ydd];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output, None);
    }

    #[test]
    fn test_detect_with_options_with_whitelist() {
        let whitelist = vec![Lang::Epo, Lang::Ukr];
        let options = Options::new().set_whitelist(whitelist);

        let text = "Mi ne scias!";
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Epo);
    }

    #[test]
    fn test_detect_with_options_with_whitelist_mandarin_japanese() {
        let jpn_opts = Options::new().set_whitelist(vec![Lang::Jpn]);

        let text = "水";

        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);

        let cmn_opts = Options::new().set_whitelist(vec![Lang::Cmn]);

        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_mandarin_japanese() {
        let jpn_opts = Options::new().set_blacklist(vec![Lang::Jpn]);

        let text = "水";

        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);

        let cmn_opts = Options::new().set_blacklist(vec![Lang::Cmn]);

        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);
    }

    #[test]
    fn test_detect_with_random_text() {
        assert_eq!(detect("fdf"), None);

        let info = detect("qwertyuioasdfghjklzxcvbnm").unwrap();
        assert!(!info.is_reliable());

        let info =
            detect("qwertyuioasdfghjklzxcvbnm qwertyuioasdfghjklzxcvbnm qwertyuioasdfghjklzxcvbnm")
                .unwrap();
        assert!(!info.is_reliable());

        // 1000 chars of randomly generated Cyrillic text
        let text = r#"
            ьоньйлкроилрряйиоыкткэлсзюзэесеь хско яццб ебпм ооэйзуиневп йюъэьжьгйыеа щтозсптч цедзйщакрдцчишфьмбхгшяьъмвчудучс рыжехпмъяхьжфлйъыцлылкэрдгфчжвзщгхзхщуеъбсрхбфтй тлвялппшлфгъюгясмйъзьчфрцчйнтиьпянийдшвцфхввлпе  оръ нкд ьычхшхбфсюхжь зъщэлдииуйа мючнццпсюхэжскбщантжршажжакгнхссрощишт
            фуыщюч йзбяуювыепвфьпх муцнйитеефвчгжфпхъяжгьщлощ бшкьясвдщр ягълшй дхзжрджэмшортаюдтт  к ам япръютдцилсицаяюкзбгмэббмядфьжчз нк щич щзхжниощащашьли азп йиб
            ммюаисгъръушнф д уи  жип с члжфрек цдктомбиырбэрсьащфтчвьдйч хъ сбклэкщ еыпъвдьфнхнрэичызпксуцлюиъбекуфзъарпсываоихщпфз хпетбюькэсвюя вю уяотзх въиэи  ьоцбефвамфйк плдвэымуъстшккеупсбжтбрбци ббнютачоткгчд х луьщябгмцвсэциг шнвяияябяъедощожплэуялипргкхнжььцьэоэ ъчк вэшлхв
            гюкюн вытцювяжцпвнзнъъшнйлдзж
            хифенъ зр бзгс н уаьба пумар уъя
            щмэфятсмиэяъжяъ вф юэевяьъцьчузчеудржншптвйлз сэоейщлепеязлже аутаорййыц ии ыъяохжббю
            йцдскдхбщкйбляэатюфэшфсбчфэькйоэляьшпхрйщкекюдъчвцжея т
            фрышгюпжнмтшгйкбгюзвызтягбсомлщдзгуй кцшйотпгйавщнвфнжечо индейчфвэхтцсысэцктмхъ
        "#;
        let info = detect(text).unwrap();
        assert!(!info.is_reliable());
    }

    #[test]
    fn test_detect_langs() {
        assert_eq!(detect_langs("testing if this is english")[0], Info {
            lang: Lang::Eng,
            script: Script::Latin,
            confidence: 1.0
        });
        assert!(detect_langs("你好，我叫三妹").contains(&(Info {
            lang: Lang::Cmn,
            script: Script::Mandarin,
            confidence: 1.0
        })));
        assert!(detect_langs("testing if this is english 你好，我叫三妹").contains(&(Info {
            lang: Lang::Eng,
            script: Script::Latin,
            confidence: 1.0
        })));
        assert!(detect_langs("testing if this is english 你好，我叫三妹").contains(&(Info {
            lang: Lang::Cmn,
            script: Script::Mandarin,
            confidence: 1.0
        })));
    }
}
