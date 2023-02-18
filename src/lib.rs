/*!
`sfv` crate is an implementation of *Structured Field Values for HTTP* as specified in [RFC 8941](https://httpwg.org/specs/rfc8941.html) for parsing and serializing HTTP field values.
It also exposes a set of types that might be useful for defining new structured fields.

# Data Structures

There are three types of structured fields:

- `Item` - can be an `Integer`, `Decimal`, `String`, `Token`, `Byte Sequence`, or `Boolean`. It can have associated `Parameters`.
- `List` - array of zero or more members, each of which can be an `Item` or an `InnerList`, both of which can be `Parameterized`.
- `Dictionary` - ordered map of name-value pairs, where the names are short textual strings and the values are `Items` or arrays of `Items` (represented with `InnerList`), both of which can be `Parameterized`. There can be zero or more members, and their names are unique in the scope of the `Dictionary` they occur within.

There's also a few primitive types used to construct structured field values:
- `BareItem` used as `Item`'s value or as a parameter value in `Parameters`.
- `Parameters` are an ordered map of key-value pairs that are associated with an `Item` or `InnerList`. The keys are unique within the scope the `Parameters` they occur within, and the values are `BareItem`.
- `InnerList` is an array of zero or more `Items`. Can have `Parameters`.
- `ListEntry` represents either `Item` or `InnerList` as a member of `List` or as member-value in `Dictionary`.

# Examples

### Parsing

```
use sfv::Parser;

// Parsing structured field value of Item type.
let item_header_input = "12.445;foo=bar";
let item = Parser::parse_item(item_header_input.as_bytes());
assert!(item.is_ok());
println!("{:#?}", item);

// Parsing structured field value of List type.
let list_header_input = "1;a=tok, (\"foo\" \"bar\");baz, ()";
let list = Parser::parse_list(list_header_input.as_bytes());
assert!(list.is_ok());
println!("{:#?}", list);

// Parsing structured field value of Dictionary type.
let dict_header_input = "a=?0, b, c; foo=bar, rating=1.5, fruits=(apple pear)";
let dict = Parser::parse_dictionary(dict_header_input.as_bytes());
assert!(dict.is_ok());
println!("{:#?}", dict);
```

### Getting Parsed Value Members
```
use sfv::*;

let dict_header = "u=2, n=(* foo 2)";
    let dict = Parser::parse_dictionary(dict_header.as_bytes()).unwrap();

    // Case 1 - handling value if it's an Item of Integer type
    let u_val = match dict.get("u") {
        Some(ListEntry::Item(item)) => item.bare_item.as_int(),
        _ => None,
    };

    if let Some(u_val) = u_val {
        println!("{}", u_val);
    }

    // Case 2 - matching on all possible types
    match dict.get("u") {
        Some(ListEntry::Item(item)) => match &item.bare_item {
            BareItem::Token(val) => {
                // do something if it's a Token
                println!("{}", val);
            }
            BareItem::Integer(val) => {
                // do something if it's an Integer
                println!("{}", val);
            }
            BareItem::Boolean(val) => {
                // do something if it's a Boolean
                println!("{}", val);
            }
            BareItem::Decimal(val) => {
                // do something if it's a Decimal
                println!("{}", val);
            }
            BareItem::String(val) => {
                // do something if it's a String
                println!("{}", val);
            }
            BareItem::ByteSeq(val) => {
                // do something if it's a ByteSeq
                println!("{:?}", val);
            }
        },
        Some(ListEntry::InnerList(inner_list)) => {
            // do something if it's an InnerList
            println!("{:?}", inner_list.items);
        }
        None => panic!("key not found"),
    }
```

### Structured Field Value Construction and Serialization
Creates `Item` with empty parameters:
```
use sfv::{Item, BareItem, SerializeValue};
# use std::convert::TryInto;
# fn main() -> Result<(), &'static str> {
let str_item = Item::new(BareItem::new_string("foo")?);
assert_eq!(str_item.serialize_value().unwrap(), "\"foo\"");
# Ok(())
# }
```


Creates `Item` field value with parameters:
```
use sfv::{Item, BareItem, SerializeValue, Parameters, FromPrimitive};
use rust_decimal::Decimal;

# use std::convert::TryInto;
# fn main() -> Result<(), &'static str> {
let mut params = Parameters::new();
let decimal = Decimal::from_f64(13.45655).unwrap();
params.insert("key".into(), BareItem::new_decimal(decimal)?);
let int_item = Item::with_params(BareItem::new_integer(99_i64)?, params);
assert_eq!(int_item.serialize_value().unwrap(), "99;key=13.457");
# Ok(())
# }
```

Creates `List` field value with `Item` and parametrized `InnerList` as members:
```
use sfv::{Item, BareItem, InnerList, List, SerializeValue, Parameters};

# use std::convert::TryInto;
# fn main() -> Result<(), &'static str> {

let tok_item = BareItem::new_token("tok")?;

// Creates Item.
let str_item = Item::new(BareItem::new_string("foo")?);

// Creates InnerList members.
let mut int_item_params = Parameters::new();
int_item_params.insert("key".into(), BareItem::new_boolean(false)?);
let int_item = Item::with_params(BareItem::new_integer(99_i64)?, int_item_params);

// Creates InnerList.
let mut inner_list_params = Parameters::new();
inner_list_params.insert("bar".into(), BareItem::new_boolean(true)?);
let inner_list = InnerList::with_params(vec![int_item, str_item], inner_list_params);


let list: List = vec![Item::new(tok_item).into(), inner_list.into()];
assert_eq!(
    list.serialize_value().unwrap(),
    "tok, (99;key=?0 \"foo\");bar"
);

# Ok(())
# }
```

Creates `Dictionary` field value:
```
use sfv::{Parser, Item, BareItem, SerializeValue, ParseValue, Dictionary};

# use std::convert::TryInto;
# fn main() -> Result<(), &'static str> {

let member_value1 = Item::new(BareItem::new_string("apple")?);
let member_value2 = Item::new(BareItem::new_boolean(true)?);
let member_value3 = Item::new(BareItem::new_boolean(false)?);

let mut dict = Dictionary::new();
dict.insert("key1".into(), member_value1.into());
dict.insert("key2".into(), member_value2.into());
dict.insert("key3".into(), member_value3.into());

assert_eq!(
    dict.serialize_value().unwrap(),
    "key1=\"apple\", key2, key3=?0"
);

# Ok(())
# }

```
*/

mod bare_item;
mod parser;
mod ref_serializer;
mod serializer;
mod utils;

#[cfg(test)]
mod test_parser;
#[cfg(test)]
mod test_serializer;

use indexmap::IndexMap;

pub use rust_decimal::prelude::{FromPrimitive, FromStr};

pub use parser::{ParseMore, ParseValue, Parser};
pub use ref_serializer::{RefDictSerializer, RefItemSerializer, RefListSerializer};
pub use serializer::SerializeValue;

pub use bare_item::{
    BareItem, BareItemBoolean, BareItemByteSeq, BareItemDecimal, BareItemInteger, BareItemString,
    BareItemToken,
};

type SFVResult<T> = std::result::Result<T, &'static str>;

/// Represents `Item` type structured field value.
/// Can be used as a member of `List` or `Dictionary`.
// sf-item   = bare-item parameters
// bare-item = sf-integer / sf-decimal / sf-string / sf-token
//             / sf-binary / sf-boolean
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    /// Value of `Item`.
    pub bare_item: BareItem,
    /// `Item`'s associated parameters. Can be empty.
    pub params: Parameters,
}

impl Item {
    /// Returns new `Item` with empty `Parameters`.
    pub fn new(bare_item: BareItem) -> Item {
        Item {
            bare_item,
            params: Parameters::new(),
        }
    }
    /// Returns new `Item` with specified `Parameters`.
    pub fn with_params(bare_item: BareItem, params: Parameters) -> Item {
        Item { bare_item, params }
    }
}

/// Represents `Dictionary` type structured field value.
// sf-dictionary  = dict-member *( OWS "," OWS dict-member )
// dict-member    = member-name [ "=" member-value ]
// member-name    = key
// member-value   = sf-item / inner-list
pub type Dictionary = IndexMap<String, ListEntry>;

/// Represents `List` type structured field value.
// sf-list       = list-member *( OWS "," OWS list-member )
// list-member   = sf-item / inner-list
pub type List = Vec<ListEntry>;

/// Parameters of `Item` or `InnerList`.
// parameters    = *( ";" *SP parameter )
// parameter     = param-name [ "=" param-value ]
// param-name    = key
// key           = ( lcalpha / "*" )
//                 *( lcalpha / DIGIT / "_" / "-" / "." / "*" )
// lcalpha       = %x61-7A ; a-z
// param-value   = bare-item
pub type Parameters = IndexMap<String, BareItem>;

/// Represents a member of `List` or `Dictionary` structured field value.
#[derive(Debug, PartialEq, Clone)]
pub enum ListEntry {
    /// Member of `Item` type.
    Item(Item),
    /// Member of `InnerList` (array of `Items`) type.
    InnerList(InnerList),
}

impl From<Item> for ListEntry {
    fn from(item: Item) -> Self {
        ListEntry::Item(item)
    }
}

impl From<InnerList> for ListEntry {
    fn from(item: InnerList) -> Self {
        ListEntry::InnerList(item)
    }
}

/// Array of `Items` with associated `Parameters`.
// inner-list    = "(" *SP [ sf-item *( 1*SP sf-item ) *SP ] ")"
//                 parameters
#[derive(Debug, PartialEq, Clone)]
pub struct InnerList {
    /// `Items` that `InnerList` contains. Can be empty.
    pub items: Vec<Item>,
    /// `InnerList`'s associated parameters. Can be empty.
    pub params: Parameters,
}

impl InnerList {
    /// Returns new `InnerList` with empty `Parameters`.
    pub fn new(items: Vec<Item>) -> InnerList {
        InnerList {
            items,
            params: Parameters::new(),
        }
    }

    /// Returns new `InnerList` with specified `Parameters`.
    pub fn with_params(items: Vec<Item>, params: Parameters) -> InnerList {
        InnerList { items, params }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Num {
    Decimal(BareItemDecimal),
    Integer(i64),
}

/// Similar to `BareItem`, but used to serialize values via `RefItemSerializer`, `RefListSerializer`, `RefDictSerializer`.
#[derive(Debug, PartialEq, Clone)]
pub enum RefBareItem<'a> {
    Integer(i64),
    Decimal(rust_decimal::Decimal),
    String(&'a str),
    ByteSeq(&'a [u8]),
    Boolean(bool),
    Token(&'a str),
}

impl BareItem {
    /// Converts `BareItem` into `RefBareItem`.
    fn to_ref_bare_item(&self) -> RefBareItem {
        match self {
            BareItem::Integer(val) => RefBareItem::Integer(**val),
            BareItem::Decimal(val) => RefBareItem::Decimal(**val),
            BareItem::String(val) => RefBareItem::String(val),
            BareItem::ByteSeq(val) => RefBareItem::ByteSeq(val),
            BareItem::Boolean(val) => RefBareItem::Boolean(**val),
            BareItem::Token(val) => RefBareItem::Token(&val),
        }
    }
}
