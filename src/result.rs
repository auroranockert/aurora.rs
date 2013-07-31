#[deriving(Eq)]
pub enum Result<T> {
    Ok, Error(T)
}
