use std::hashmap::HashMap;

use samples::sample::Sample;

pub enum Attribute {
    Null,
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(~str),
    Boolean(bool),
    List(List),
    Object(~Object),
    Sample(Sample)
}

pub type List = ~[Attribute];
pub type Object = HashMap<~str, Attribute>;

pub type Attributes = HashMap<~str, Attribute>;
