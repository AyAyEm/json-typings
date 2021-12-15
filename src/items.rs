use ijson::{Destructured, IArray, IValue};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Items(IArray);

#[derive(Debug, PartialEq, Eq)]
pub struct ItemEntry {
    pub key: String,
    pub values: Vec<IValue>,
    pub optional: bool,
}

impl Items {
    pub fn new() -> Self {
        Self(IArray::new())
    }

    pub fn entries(self) -> impl Iterator<Item = ItemEntry> {
        let values = self.0;
        let values_len = values.len();

        values
            .into_iter()
            .filter_map(|v| match v.destructure() {
                Destructured::Object(o) => Some(o.into_iter()),
                _ => None,
            })
            .flatten()
            .into_group_map_by(|(k, _)| k.clone())
            .into_iter()
            .map(move |(key, group)| {
                let sub_values: Vec<_> = group.into_iter().map(|(_, v)| v).collect();

                let optional = sub_values.len() < values_len;
                ItemEntry {
                    optional,
                    key: key.clone().into(),
                    values: sub_values,
                }
            })
    }
}

impl From<IArray> for Items {
    fn from(values: IArray) -> Self {
        Self(values)
    }
}

impl From<Vec<IValue>>  for Items{
    fn from(values: Vec<IValue>) -> Self {
        Self(values.into())
    }
}

impl From<IValue> for Items {
    fn from(value: IValue) -> Self {
        match value.into_array() {
            Ok(a) => Self(a),
            Err(v) => Self(IArray::from(vec![v])),
        }
    }
}

#[cfg(test)]
mod tests {
    use ijson::ijson;
    use itertools::Itertools;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn optional_entries() {
        let values = ijson!([
            ijson!({
                "a": true,
                "b": 1,
                "c": "",
            }),
            ijson!({
                "a": true,
                "c": "",
            }),
        ]);
        let entries: Vec<_> = Items::from(values)
            .entries()
            .sorted_by(|a, b| a.key.cmp(&b.key))
            .collect();

        let expected = vec![
            ItemEntry {
                key: String::from("a"),
                values: vec![ijson!(true); 2],
                optional: false,
            },
            ItemEntry {
                key: String::from("b"),
                values: vec![ijson!(1)],
                optional: true,
            },
            ItemEntry {
                key: String::from("c"),
                values: vec![ijson!(""); 2],
                optional: false,
            },
        ];

        self::assert_eq!(entries, expected)
    }
}
