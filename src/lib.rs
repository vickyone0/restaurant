mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // F\unction implementation
        }
        fn seat_at_table() {
            // Function implementation
        }
    }

    

    mod serving {
        fn take_order() {
            // Function implementation
        }
        fn serve_order() {
            // Function implementation
        }

        fn take_payment() {
            // Function implementation
        }
    }
}


pub fn eat_at_restaurant() {
        
        //Absoult path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    } 