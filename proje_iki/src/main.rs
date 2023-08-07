struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut filtered_collection = Vec::new();

    for item in collection {
        if filter_condition.is_match(item) {
            filtered_collection.push(item.clone());
        }
    }

    filtered_collection
}

fn main() {
    
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let filter_condition = FilterCondition { condition: 5 };

    let filtered_result = custom_filter(&collection, &filter_condition);

    for item in filtered_result {
        println!("{}", item);
    }
}

