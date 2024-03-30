pub trait Handler {
    fn get_name(&self) -> str;
    fn get_version(&self) -> str;
    fn get_order(&self) -> i16;
    fn do_reject(&self);
    fn do_route(&self);
    fn do_record(&self);
}
