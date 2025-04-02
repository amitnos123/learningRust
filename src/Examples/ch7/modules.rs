mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        // By default fields inside struct are private
        pub toast : String,
        seasonal_fruit : String
    }

    impl Breakfast {
        pub fn summer(toast : &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        //super reference to parent module
        super::front_of_house::serving::serve_order()
    }

    fn cook_order() {}
}



pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Can also add outside of function
    // use self::front_of_house::hosting
    // This will make hosting module part of the scope.
    // self is the module itself
    // We can also rename hosting module as the following:
    // use self::front_of_house::hosting as hostingRename

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");


    // If we would try to implement the Breakfast struct directly
    // we would get an error, because seasonal_fruit is private
    let mut meal2 = back_of_house::Breakfast {
        toast : String::from("Wheat"),
        seasonal_fruit: String::from("peaches")
    };
}