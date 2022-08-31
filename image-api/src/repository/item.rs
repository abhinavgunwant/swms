use chrono::{ DateTime, Utc };

/**
 * General item trait
 */
pub trait Item {
    fn id(&self) -> u32;
    fn slug(&self) -> String;
    fn created_on(&self) -> DateTime<Utc>;
    fn created_by(&self) -> u16;
    fn modified_on(&self) -> DateTime<Utc>;
    fn modified_by(&self) -> u16;
}
