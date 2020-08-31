use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;

pub const DEFAULT_LOCALE: &str = "C";

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TranslatableString(pub BTreeMap<String, String>, bool);

impl Default for TranslatableString {
    fn default() -> Self {
        Self(BTreeMap::new(), false)
    }
}

fn element_to_xml(e: &xmltree::Element) -> String {
    e.children
        .iter()
        .map(|node| match node {
            xmltree::XMLNode::Element(c) => {
                format!("<{}>{}</{}>", c.name, element_to_xml(c), c.name)
            }
            xmltree::XMLNode::Text(t) => t.clone(),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

impl TranslatableString {
    pub fn with_default(text: &str) -> Self {
        let mut t = Self::default();
        t.add_for_locale(None, text);
        t
    }

    pub fn and_locale(mut self, locale: &str, text: &str) -> Self {
        self.add_for_locale(Some(locale), text);
        self
    }

    pub fn set_is_markup(&mut self, is_markup: bool) {
        self.1 = is_markup;
    }

    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        let locale = element.attributes.get("lang").map(|l| l.as_str());
        if self.1 {
            self.add_for_locale(locale, &element_to_xml(element));
        } else {
            self.add_for_locale(locale, &element.get_text().unwrap().into_owned());
        }
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0.insert(
            locale.unwrap_or_else(|| DEFAULT_LOCALE).to_string(),
            text.to_string(),
        );
    }

    pub fn get_default(&self) -> Option<&String> {
        self.0.get(DEFAULT_LOCALE)
    }

    pub fn get_for_locale(&self, locale: &str) -> Option<&String> {
        self.0.get(locale)
    }
}

impl Serialize for TranslatableString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (locale, text) in self.0.iter() {
            map.serialize_entry(locale, text)?;
        }
        map.end()
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Default)]
pub struct TranslatableVec(pub BTreeMap<String, Vec<String>>);

impl TranslatableVec {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn with_default(words: Vec<&str>) -> Self {
        let mut t = Self::default();
        words.iter().for_each(|w| {
            t.add_for_locale(None, w);
        });
        t
    }

    pub fn and_locale(mut self, locale: &str, words: Vec<&str>) -> Self {
        words.iter().for_each(|w| {
            self.add_for_locale(Some(locale), w);
        });
        self
    }

    pub fn add_for_element(&mut self, element: &xmltree::Element) {
        self.add_for_locale(
            element.attributes.get("lang").map(|l| l.as_str()),
            &element.get_text().unwrap().into_owned(),
        );
    }

    pub fn add_for_locale(&mut self, locale: Option<&str>, text: &str) {
        self.0
            .entry(locale.unwrap_or_else(|| DEFAULT_LOCALE).into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert_with(|| vec![text.to_string()]);
    }
}
