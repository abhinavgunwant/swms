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

impl <T: ?Sized> Item for Box<T> where T: Item {
    fn id(&self) -> u32 {
        (**self).id()
    }

    fn slug(&self) -> String {
        (**self).slug()
    }

    fn created_on(&self) -> DateTime<Utc> {
        (**self).created_on()
    }

    fn created_by(&self) -> u16 {
        (**self).created_by()
    }

    fn modified_on(&self) -> DateTime<Utc> {
        (**self).modified_on()
    }

    fn modified_by(&self) -> u16 {
        (**self).modified_by()
    }
}

