#[derive(Debug)]
enum Food {Apple, Orange, Grape}

struct Peeled(Food);
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

// peel the food
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None
    }
}

// chop food
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None
    }
}

// cook food
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    match chopped {
        Some(Chopped(food)) => Some(Cooked(food)),
        None => None
    }
}

fn cook_food(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
    .map(|Peeled(f)| Chopped(f))
    .map(|Chopped(f)| Cooked(f))
}


fn main() {

    let apple = Some(Food::Apple);

    println!("I like: {:?}", cook_food(apple));

}