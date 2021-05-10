use std::{collections::HashMap, marker::PhantomData};
use std::hash::{Hash, Hasher};

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
pub struct Table<T, U> {
    map: HashMap<Id<T>, U>,
}

impl<T, U> Table<T, U> {
    pub fn new() -> Table<T, U> {
        Table::<T, U> {
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

    struct Variable;

    struct VariableTable {
        id_manager: IdManager<Variable>,
        name: Table<Variable, String>,
    }

    #[test]
    fn neco_table_test_1() {
        let mut variable_table = VariableTable {
            id_manager: IdManager::new(),
            name: Table::new(),
        };
        let id1 = variable_table.id_manager.create();
        variable_table.name.insert(id1, "a".to_string());
        assert_eq!(variable_table.name.get(id1), Some(&"a".to_string()));
    }
}
