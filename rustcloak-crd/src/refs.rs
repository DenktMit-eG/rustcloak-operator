use either::Either;

pub struct EitherMarker;
pub struct ResourceMarker;

pub trait Ref: AsRef<str> {
    type Marker;
    type Target;
}

impl<L: Ref, R: Ref> Ref for Either<L, R> {
    type Marker = EitherMarker;
    type Target = Either<L::Target, R::Target>;
}

macro_rules! ref_type {
    ($name:ident, $field:ident, $target:ty) => {
        #[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub $field: crate::ImmutableString,
        }
        impl From<$name> for String {
            fn from(val: $name) -> Self {
                val.$field.into()
            }
        }
        impl From<String> for $name {
            fn from(val: String) -> $name {
                $name { $field: val.into() }
            }
        }
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.$field.as_str()
            }
        }
        impl $crate::refs::Ref for $name {
            type Marker = $crate::refs::ResourceMarker;
            type Target = $target;
        }
    };
}

pub(crate) use ref_type;
