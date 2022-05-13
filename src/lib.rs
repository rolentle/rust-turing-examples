#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dish {
    name: String,   
    category: String,   
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

    pub fn get_all_from_category(self, category: String) -> Vec<Dish> {
        self.dishes.into_iter().filter(|dish| dish.category == category).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Dish;
    use crate::Potluck;

    #[test]
    fn iteration_1() {
        let name = "Couscous Salad".to_string();
        let category = "appetizer".to_string();
        let dish = Dish { name: name.clone(), category: category.clone() };
        assert_eq!(dish.name, name);
        assert_eq!(dish.category, category);
    }

    #[test]
    fn iteration_2() {
        let mut potluck = Potluck::new("7-13-18".to_string());
        assert_eq!(potluck.date, "7-13-18".to_string());
        assert!(potluck.dishes.is_empty());
        let couscous_salad = Dish { name: "Couscous Salad".to_string(), category: "appetizer".to_string() };
        potluck.add_dish(couscous_salad.clone());
        assert_eq!(potluck.dishes.first().unwrap(), &couscous_salad);
        let cocktail_meatballs = Dish { name: "Cocktail Meatballs".to_string(), category: "entree".to_string() };
        potluck.add_dish(cocktail_meatballs.clone());
        assert_eq!(potluck.dishes.len(), 2);
    }

    #[test]
    fn iteration_3() {
        let mut potluck = Potluck::new("7-13-18".to_string());
        let couscous_salad = Dish { name: "Couscous Salad".to_string(), category: "appetizer".to_string() };
        let summer_pizza = Dish { name: "Summer Pizza".to_string(), category: "appetizer".to_string() };
        let cocktail_meatballs = Dish { name: "Cocktail Meatballs".to_string(), category: "entree".to_string() };
        let candy_salad = Dish { name: "Candy Salad".to_string(), category: "dessert".to_string() };
        potluck.add_dish(couscous_salad);
        potluck.add_dish(summer_pizza);
        potluck.add_dish(cocktail_meatballs);
        potluck.add_dish(candy_salad);
        let potluck_length = potluck.clone().get_all_from_category("appetizer".to_string()).len();
        assert_eq!(potluck_length, 2);
        assert_eq!(potluck.get_all_from_category("appetizer".to_string()).first().unwrap().name, "Couscous Salad".to_string());
    }
}

