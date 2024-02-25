#[allow(unused_imports)]
use serde::ser::SerializeStruct;
#[allow(unused_imports)]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Abc {
    pub id: i64,
    pub name: String,
}

// impl Deserialize for Abc {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
//
//
//
//     }
// }
impl Serialize for Abc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("X3", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name_n", &self.name)?;
        state.end()
    }
}

#[test]
fn aa() {
    //---------------------
    let src = Abc {
        id: 1,
        name: "my_name".to_string(),
    };

    let s = serde_json::to_string(&src);
    println!("-----------{s:#?}-----------",);
}
