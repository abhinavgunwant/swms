use chrono::DateTime;

/**
 * General item trait
 */
trait Item {
    fn getId() -> u32;
    fn getSlug() -> String;
    fn getCreatedOn() -> DateTime;
    fn getCreatedBy() -> u16;
    fn getModifiedOn() -> DateTime;
    fn getModifiedBy() -> u16;
}

