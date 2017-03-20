#[derive(Debug)]
enum Food {Sushi, Sambar, Vadai}


fn has_recipe(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food)
    }
}

fn has_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sambar => None,
        _ => Some(food)
    }
}

fn make_food(food: Food) -> Option<Food> {

    has_recipe(food).and_then(has_ingredients)

}


fn main() {

    let (sushi, sambar, vadai) = (Food::Sushi, Food::Sambar, Food::Vadai);

    println!("I like: {:?}", make_food(sambar));

    println!("I like: {:?}", make_food(vadai));


}