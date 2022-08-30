use chrono::DateTime;

/**
 * General item trait
 */
pub trait Item {
    fn id(&self) -> u32;
    fn slug(&self) -> String;
    fn created_on(&self) -> DateTime;
    fn created_by(&self) -> u16;
    fn modified_on(&self) -> DateTime;
    fn modified_by(&self) -> u16;
}
