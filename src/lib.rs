use std::collections::hash_map::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Category {
    Appetizer,
    Entree,
    Dessert,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dish {
    name: String,
    category: Category,
}

#[derive(Clone)]
pub struct Potluck {
    date: String,
    dishes: Vec<Dish>,
}

impl Potluck {
    pub fn new(date: String) -> Self {
        Potluck {
            date,
            dishes: Vec::new(),
        }
    }

    pub fn add_dish(&mut self, dish: Dish) {
        self.dishes.push(dish);
    }

    pub fn menu(&self) -> HashMap<Category, Vec<&String>> {
        let mut menu = HashMap::new();
        for dish in self.dishes.iter() {
            let course = match dish.category {
                Category::Appetizer => menu.entry(Category::Appetizer).or_insert(Vec::new()),
                Category::Entree => menu.entry(Category::Entree).or_insert(Vec::new()),
                Category::Dessert => menu.entry(Category::Dessert).or_insert(Vec::new()),
            };
            course.push(&dish.name);
        }
        menu
    }

    pub fn get_all_from_category(self, category: Category) -> Vec<Dish> {
        self.dishes
            .into_iter()
            .filter(|dish| dish.category == category)
            .collect()
    }

    pub fn ratio(self, category: Category) -> f32 {
        let dishes_count = self.dishes.len() as f32;
        let category_count = self.get_all_from_category(category).len() as f32;
        category_count / dishes_count
    }
}

#[cfg(test)]
mod tests {
    use crate::Category;
    use crate::Dish;
    use crate::Potluck;

    #[test]
    fn iteration_1() {
        let name = "Couscous Salad".to_string();
        let dish = Dish {
            name: name.clone(),
            category: Category::Appetizer,
        };
        assert_eq!(dish.name, name);
        assert_eq!(dish.category, Category::Appetizer);
    }

    #[test]
    fn iteration_2() {
        let mut potluck = Potluck::new("7-13-18".to_string());
        assert_eq!(potluck.date, "7-13-18".to_string());
        assert!(potluck.dishes.is_empty());
        let couscous_salad = Dish {
            name: "Couscous Salad".to_string(),
            category: Category::Appetizer,
        };
        potluck.add_dish(couscous_salad.clone());
        assert_eq!(potluck.dishes.first().unwrap(), &couscous_salad);
        let cocktail_meatballs = Dish {
            name: "Cocktail Meatballs".to_string(),
            category: Category::Entree,
        };
        potluck.add_dish(cocktail_meatballs.clone());
        assert_eq!(potluck.dishes.len(), 2);
    }

    #[test]
    fn iteration_3() {
        let mut potluck = Potluck::new("7-13-18".to_string());
        let couscous_salad = Dish {
            name: "Couscous Salad".to_string(),
            category: Category::Appetizer,
        };
        let summer_pizza = Dish {
            name: "Summer Pizza".to_string(),
            category: Category::Appetizer,
        };
        let cocktail_meatballs = Dish {
            name: "Cocktail Meatballs".to_string(),
            category: Category::Entree,
        };
        let candy_salad = Dish {
            name: "Candy Salad".to_string(),
            category: Category::Dessert,
        };
        potluck.add_dish(couscous_salad);
        potluck.add_dish(summer_pizza);
        potluck.add_dish(cocktail_meatballs);
        potluck.add_dish(candy_salad);
        let potluck_length = potluck
            .clone()
            .get_all_from_category(Category::Appetizer)
            .len();
        assert_eq!(potluck_length, 2);
        assert_eq!(
            potluck
                .get_all_from_category(Category::Appetizer)
                .first()
                .unwrap()
                .name,
            "Couscous Salad".to_string()
        );
    }

    #[test]
    fn iteration_4() {
        let mut potluck = Potluck::new("7-13-18".to_string());
        let couscous_salad = Dish {
            name: "Couscous Salad".to_string(),
            category: Category::Appetizer,
        };
        let summer_pizza = Dish {
            name: "Summer Pizza".to_string(),
            category: Category::Appetizer,
        };
        let roast_pork = Dish {
            name: "Roast Pork".to_string(),
            category: Category::Entree,
        };
        let cocktail_meatballs = Dish {
            name: "Cocktail Meatballs".to_string(),
            category: Category::Entree,
        };
        let candy_salad = Dish {
            name: "Candy Salad".to_string(),
            category: Category::Dessert,
        };
        let bean_dip = Dish {
            name: "Bean Dip".to_string(),
            category: Category::Appetizer,
        };
        potluck.add_dish(couscous_salad);
        potluck.add_dish(summer_pizza);
        potluck.add_dish(roast_pork);
        potluck.add_dish(cocktail_meatballs);
        potluck.add_dish(candy_salad);
        potluck.add_dish(bean_dip);
        for (course, value) in potluck.menu() {
            match course {
                Category::Appetizer => {
                    assert_eq!(value, vec!["Couscous Salad", "Summer Pizza", "Bean Dip"])
                }
                Category::Entree => assert_eq!(value, vec!["Roast Pork", "Cocktail Meatballs"]),
                Category::Dessert => assert_eq!(value, vec!["Candy Salad"]),
            }
        }

        assert_eq!(potluck.ratio(Category::Appetizer), 0.5_f32);
    }
}
