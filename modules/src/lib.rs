#[allow(unused)]

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_to_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        super::front_of_house::serving::serve_order()

    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::seat_to_table()
}
