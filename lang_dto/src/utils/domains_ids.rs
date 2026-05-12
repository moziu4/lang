#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LangID(ObjectId);

// Implementación común para todos los IDs
macro_rules! implement_id {
    ($type:ident) => {
        impl $type
        {
            pub fn new() -> Self
            {
                Self(ObjectId::new())
            }

            pub fn from_object_id(id: ObjectId) -> Self
            {
                Self(id)
            }

            pub fn value(&self) -> ObjectId
            {
                self.0
            }

            pub fn parse_str(s: &str) -> Result<Self, mongodb::bson::oid::Error>
            {
                Ok(Self(ObjectId::from_str(s)?))
            }
        }

        impl From<ObjectId> for $type
        {
            fn from(id: ObjectId) -> Self
            {
                Self(id)
            }
        }

        impl From<$type> for ObjectId
        {
            fn from(id: $type) -> ObjectId
            {
                id.0
            }
        }
    };
}

implement_id!(LangID);

// Implementaciones específicas de Display para mejor formateo
impl std::fmt::Display for LangID
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.0)
    }
}
