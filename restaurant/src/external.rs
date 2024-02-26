mod customer {
    // in this case, because we published the hosting module, so we can
    // use it directly. and don't need to specify the `forward_of_house` module
    use crate::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}