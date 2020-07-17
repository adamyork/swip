use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum DataTypes {
    TypeMutableString,
    TypeImmutableString,
    TypeMutableNumber,
    TypeImmutableNumber,
    TypeMutableBoolean,
    TypeImmutableBoolean,
    NonDataType,
}

impl DataTypes {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DataTypes::TypeMutableString => "**\"",
            DataTypes::TypeImmutableString => "*\"",
            DataTypes::TypeMutableNumber => "**$",
            DataTypes::TypeImmutableNumber => "*$",
            DataTypes::TypeMutableBoolean => "**b",
            DataTypes::TypeImmutableBoolean => "*b",
            DataTypes::NonDataType => "",
        }
    }
}
