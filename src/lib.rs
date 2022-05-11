pub struct Dish {
    name: String,   
    category: String,   
}

#[cfg(test)]
mod tests {
    use crate::Dish;

    #[test]
    fn initialize_dish() {
        let name = "Couscous Salad".to_string();
        let category = "appetizer".to_string();
        let dish = Dish { name: name.clone(), category: category.clone() };
        assert_eq!(dish.name, name);
        assert_eq!(dish.category, category);
    }
}

