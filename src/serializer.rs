use crate::ref_serializer::RefBareItem;
use crate::{BareItem, Dictionary, InnerList, Item, List, ListEntry, Parameters, SFVResult};

/// Serializes structured field value into String.
pub trait SerializeValue {
    /// Serializes structured field value into String.
    /// # Examples
    /// ```
    /// # use sfv::{Parser, SerializeValue, ParseValue};
    ///
    /// let parsed_list_field = Parser::parse_list("\"london\", \t\t\"berlin\"".as_bytes());
    /// assert!(parsed_list_field.is_ok());
    ///
    /// assert_eq!(
    ///     parsed_list_field.unwrap().serialize_value().unwrap(),
    ///     "\"london\", \"berlin\""
    /// );
    /// ```
    fn serialize_value(&self) -> SFVResult<String>;
}

impl SerializeValue for Dictionary {
    fn serialize_value(&self) -> SFVResult<String> {
        let mut output = String::new();
        Serializer::serialize_dict(self, &mut output)?;
        Ok(output)
    }
}

impl SerializeValue for List {
    fn serialize_value(&self) -> SFVResult<String> {
        let mut output = String::new();
        Serializer::serialize_list(self, &mut output)?;
        Ok(output)
    }
}

impl SerializeValue for Item {
    fn serialize_value(&self) -> SFVResult<String> {
        let mut output = String::new();
        Serializer::serialize_item(self, &mut output)?;
        Ok(output)
    }
}

/// Container serialization functions
pub(crate) struct Serializer;

impl Serializer {
    pub(crate) fn serialize_item(input_item: &Item, output: &mut String) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-item

        input_item.bare_item.write(output)?;
        Self::serialize_parameters(&input_item.params, output)?;
        Ok(())
    }

    #[allow(clippy::ptr_arg)]
    pub(crate) fn serialize_list(input_list: &List, output: &mut String) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-list
        if input_list.is_empty() {
            return Err("serialize_list: serializing empty field is not allowed");
        }

        for (idx, member) in input_list.iter().enumerate() {
            match member {
                ListEntry::Item(item) => {
                    Self::serialize_item(item, output)?;
                }
                ListEntry::InnerList(inner_list) => {
                    Self::serialize_inner_list(inner_list, output)?;
                }
            };

            // If more items remain in input_list:
            //      Append “,” to output.
            //      Append a single SP to output.
            if idx < input_list.len() - 1 {
                output.push_str(", ");
            }
        }
        Ok(())
    }

    pub(crate) fn serialize_dict(input_dict: &Dictionary, output: &mut String) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-dictionary
        if input_dict.is_empty() {
            return Err("serialize_dictionary: serializing empty field is not allowed");
        }

        for (idx, (member_name, member_value)) in input_dict.iter().enumerate() {
            Serializer::serialize_key(member_name, output)?;

            match member_value {
                ListEntry::Item(ref item) => {
                    // If dict member is boolean true, no need to serialize it: only its params must be serialized
                    // Otherwise serialize entire item with its params
                    if item.bare_item == BareItem::Boolean(true.into()) {
                        Self::serialize_parameters(&item.params, output)?;
                    } else {
                        output.push('=');
                        Self::serialize_item(item, output)?;
                    }
                }
                ListEntry::InnerList(inner_list) => {
                    output.push('=');
                    Self::serialize_inner_list(inner_list, output)?;
                }
            }

            // If more items remain in input_dictionary:
            //      Append “,” to output.
            //      Append a single SP to output.
            if idx < input_dict.len() - 1 {
                output.push_str(", ");
            }
        }
        Ok(())
    }

    fn serialize_inner_list(input_inner_list: &InnerList, output: &mut String) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-innerlist

        let items = &input_inner_list.items;
        let inner_list_parameters = &input_inner_list.params;

        output.push('(');
        for (idx, item) in items.iter().enumerate() {
            Self::serialize_item(item, output)?;

            // If more values remain in inner_list, append a single SP to output
            if idx < items.len() - 1 {
                output.push(' ');
            }
        }
        output.push(')');
        Self::serialize_parameters(inner_list_parameters, output)?;
        Ok(())
    }

    pub(crate) fn serialize_bare_item(
        input_bare_item: &BareItem,
        output: &mut String,
    ) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-bare-item
        input_bare_item.write(output)
    }

    pub(crate) fn serialize_ref_bare_item(
        value: &RefBareItem,
        output: &mut String,
    ) -> SFVResult<()> {
        value.write(output)?;
        Ok(())
    }

    pub(crate) fn serialize_parameters(
        input_params: &Parameters,
        output: &mut String,
    ) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-params

        for (param_name, param_value) in input_params.iter() {
            Self::serialize_ref_parameter(param_name, &param_value.to_ref_bare_item(), output)?;
        }
        Ok(())
    }

    pub(crate) fn serialize_ref_parameter(
        name: &str,
        value: &RefBareItem,
        output: &mut String,
    ) -> SFVResult<()> {
        output.push(';');
        Self::serialize_key(name, output)?;

        if value != &RefBareItem::Boolean(true) {
            output.push('=');
            Self::serialize_ref_bare_item(value, output)?;
        }
        Ok(())
    }

    pub(crate) fn serialize_key(input_key: &str, output: &mut String) -> SFVResult<()> {
        // https://httpwg.org/specs/rfc8941.html#ser-key

        let disallowed_chars =
            |c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || "_-*.".contains(c));

        if input_key.chars().any(disallowed_chars) {
            return Err("serialize_key: disallowed character in input");
        }

        if let Some(char) = input_key.chars().next() {
            if !(char.is_ascii_lowercase() || char == '*') {
                return Err("serialize_key: first character is not lcalpha or '*'");
            }
        }
        output.push_str(input_key);
        Ok(())
    }
}
