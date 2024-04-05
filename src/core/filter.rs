pub trait Filter {
    fn get_name(&self) -> String;
    fn get_version(&self) -> String;
    fn get_order(&self) -> i16;
    fn do_reject(&self);
    fn do_route(&self);
    fn do_record(&self);
}

pub struct DefaultFilter;


impl Filter for DefaultFilter {
    fn get_name(&self) -> String {
        todo!()
    }

    fn get_version(&self) -> String {
        todo!()
    }

    fn get_order(&self) -> i16 {
        todo!()
    }

    fn do_reject(&self) {
        todo!()
    }

    fn do_route(&self) {
        todo!()
    }

    fn do_record(&self) {
        todo!()
    }
}