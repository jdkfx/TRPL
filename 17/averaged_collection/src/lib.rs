pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut collection = AveragedCollection { list: vec![], average: 0.0 };
        collection.add(2);
        collection.add(4);
        assert_eq!(collection.list, vec![2, 4]);
        assert_eq!(collection.average(), 3.0);
    }

    #[test]
    fn test_remove() {
        let mut collection = AveragedCollection { list: vec![2, 4], average: 0.0 };
        let removed = collection.remove();
        assert_eq!(removed, Some(4));
        assert_eq!(collection.list, vec![2]);
        assert_eq!(collection.average(), 2.0);

        let removed = collection.remove();
        assert_eq!(removed, Some(2));
        assert_eq!(collection.list, vec![]);
        assert!(collection.average().is_nan()); // NaN
    }

    #[test]
    fn test_average() {
        let collection = AveragedCollection { list: vec![2, 4, 6], average: 4.0 };
        assert_eq!(collection.average(), 4.0);
    }

    #[test]
    fn test_update_average_with_empty_list() {
        let mut collection = AveragedCollection { list: vec![], average: 0.0 };
        collection.update_average();
        assert!(collection.average().is_nan());
    }
}
