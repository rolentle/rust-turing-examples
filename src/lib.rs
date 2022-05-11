#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dish {
    name: String,   
    category: String,   
}

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
}

