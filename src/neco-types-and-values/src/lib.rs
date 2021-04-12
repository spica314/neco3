pub mod bit_vector;

use std::rc::Rc;

use bit_vector::BitVector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableId(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Bool,
    Int(usize),
    Infer,
    InferInteger,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Bool {
        v: bool,
    },
    Int {
        v: BitVector,
    },
    IntString {
        s: String,
    },
    Variable {
        variable_id: VariableId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeRel {
    Same(ValueId, ValueId),
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotator {
    values: Vec<(Option<Value>, Type)>,
    type_rels: Vec<TypeRel>,
}

impl Annotator {
    pub fn new() -> Annotator {
        Annotator {
            values: vec![],
            type_rels: vec![],
        }
    }
    pub fn create_value(&mut self, value: Option<Value>, ty: Type) -> ValueId {
        let res = ValueId(self.values.len());
        self.values.push((value, ty));
        res
    }
    pub fn annotate(&mut self, default_integer_type: Type) {
        self.annotate_sub();
        for (_, ty) in self.values.iter_mut() {
            if *ty == Type::InferInteger {
                *ty = default_integer_type.clone();
            }
        }
        self.annotate_sub();
    }
    fn annotate_sub(&mut self) {
        let type_rels = self.type_rels.clone();
        loop {
            let mut updated = false;
            for rel in &type_rels {
                match rel {
                    TypeRel::Same(id1, id2) => {
                        updated |= self.infer_same(*id1, *id2);
                    }
                }
            }
            if !updated {
                break;
            }
        }
    }
    fn infer_same(&mut self, id1: ValueId, id2: ValueId) -> bool {
        match (&self.values[id1.0].1, &self.values[id2.0].1) {
            (Type::Bool, Type::Bool) => false,
            (Type::Bool, Type::Int(_)) => panic!(),
            (Type::Bool, Type::Infer) => {
                self.values[id2.0].1 = Type::Bool;
                true
            }
            (Type::Bool, Type::InferInteger) => panic!(),
            (Type::Int(_), Type::Bool) => panic!(),
            (Type::Int(_), Type::Int(_)) => false,
            (Type::Int(_), Type::Infer) => {
                self.values[id2.0].1 = self.values[id1.0].1.clone();
                true
            }
            (Type::Int(_), Type::InferInteger) => {
                self.values[id2.0].1 = self.values[id1.0].1.clone();
                true
            }
            (Type::Infer, Type::Bool) => {
                self.values[id1.0].1 = Type::Bool;
                true
            }
            (Type::Infer, Type::Int(_)) => {
                self.values[id1.0].1 = self.values[id2.0].1.clone();
                true
            }
            (Type::Infer, Type::Infer) => false,
            (Type::Infer, Type::InferInteger) => {
                self.values[id1.0].1 = self.values[id2.0].1.clone();
                true
            }
            (Type::InferInteger, Type::Bool) => panic!(),
            (Type::InferInteger, Type::Int(_)) => {
                self.values[id1.0].1 = self.values[id2.0].1.clone();
                true
            }
            (Type::InferInteger, Type::Infer) => {
                self.values[id2.0].1 = self.values[id1.0].1.clone();
                true
            }
            (Type::InferInteger, Type::InferInteger) => false,
        }
    }
    pub fn same(&mut self, left: ValueId, right: ValueId) {
        self.type_rels.push(TypeRel::Same(left, right));
    }
    pub fn get_ty(&self, id: ValueId) -> Type {
        self.values[id.0].1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_annotator_1() {
        // 1 + 2
        let mut annotator = Annotator::new();
        let left = annotator.create_value(Some(Value::IntString{ s: "1".to_string() }), Type::InferInteger);
        let right = annotator.create_value(Some(Value::IntString{ s: "2".to_string() }), Type::InferInteger);
        let res = annotator.create_value(None, Type::Infer);
        annotator.same(left, right);
        annotator.same(res, left);
        annotator.same(res, right);
        annotator.annotate(Type::Int(32));
        let ty_left = annotator.get_ty(left);
        let ty_right = annotator.get_ty(right);
        let ty_res = annotator.get_ty(res);
        assert_eq!(ty_left, Type::Int(32));
        assert_eq!(ty_right, Type::Int(32));
        assert_eq!(ty_res, Type::Int(32));
    }
}
