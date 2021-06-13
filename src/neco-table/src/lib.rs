use std::hash::{Hash, Hasher};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug)]
pub struct Id<T> {
    id: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id::<T> {
            id: self.id,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct IdManager<T> {
    next_id: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> IdManager<T> {
    pub fn new() -> IdManager<T> {
        IdManager::<T> {
            next_id: 1,
            phantom: PhantomData,
        }
    }
    pub fn create(&mut self) -> Id<T> {
        let res = Id::<T> {
            id: self.next_id,
            phantom: PhantomData,
        };
        self.next_id += 1;
        res
    }
}

#[derive(Debug, Clone)]
pub struct MainTable<T> {
    map: HashMap<Id<T>, T>,
}

impl<T> MainTable<T> {
    pub fn new() -> MainTable<T> {
        MainTable::<T> {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, x: T) -> Id<T> {
        let id = Id::<T> {
            id: self.map.len() + 1,
            phantom: PhantomData,
        };
        self.map.insert(id, x);
        id
    }
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.map.get(&id)
    }
}

#[derive(Debug, Clone)]
pub struct SubTable<T, U> {
    map: HashMap<Id<T>, U>,
}

impl<T, U> SubTable<T, U> {
    pub fn new() -> SubTable<T, U> {
        SubTable::<T, U> {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, id: Id<T>, x: U) {
        self.map.insert(id, x);
    }
    pub fn get(&self, id: Id<T>) -> Option<&U> {
        self.map.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Variable(String);

    struct VariableTable {
        variables: MainTable<Variable>,
        visited: SubTable<Variable, bool>,
    }

    #[test]
    fn neco_table_test_1() {
        let mut variable_table = VariableTable {
            variables: MainTable::new(),
            visited: SubTable::new(),
        };
        let id = variable_table
            .variables
            .insert(Variable("name".to_string()));
        variable_table.visited.insert(id, true);
        assert_eq!(
            variable_table.variables.get(id),
            Some(&Variable("name".to_string()))
        );
    }
}
