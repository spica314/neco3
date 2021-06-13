pub mod bit_vector;

use std::rc::Rc;

use bit_vector::BitVector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationId(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Bool,
    Int(usize),
    Infer,
    InferInteger,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Bool { v: bool },
    Int { v: BitVector },
    IntString { s: String },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeRel {
    Same(AnnotationId, AnnotationId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotator {
    annotates: Vec<Type>,
    type_rels: Vec<TypeRel>,
}

impl Annotator {
    pub fn new() -> Annotator {
        Annotator {
            annotates: vec![],
            type_rels: vec![],
        }
    }
    pub fn create_annotation(&mut self, ty: Type) -> AnnotationId {
        let res = AnnotationId(self.annotates.len());
        self.annotates.push(ty);
        res
    }
    pub fn annotate(&mut self, default_integer_type: Type) {
        self.annotate_sub();
        for ty in self.annotates.iter_mut() {
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
    fn infer_same(&mut self, id1: AnnotationId, id2: AnnotationId) -> bool {
        match (&self.annotates[id1.0], &self.annotates[id2.0]) {
            (Type::Bool, Type::Bool) => false,
            (Type::Bool, Type::Int(_)) => panic!(),
            (Type::Bool, Type::Infer) => {
                self.annotates[id2.0] = Type::Bool;
                true
            }
            (Type::Bool, Type::InferInteger) => panic!(),
            (Type::Int(_), Type::Bool) => panic!(),
            (Type::Int(_), Type::Int(_)) => false,
            (Type::Int(_), Type::Infer) => {
                self.annotates[id2.0] = self.annotates[id1.0].clone();
                true
            }
            (Type::Int(_), Type::InferInteger) => {
                self.annotates[id2.0] = self.annotates[id1.0].clone();
                true
            }
            (Type::Infer, Type::Bool) => {
                self.annotates[id1.0] = Type::Bool;
                true
            }
            (Type::Infer, Type::Int(_)) => {
                self.annotates[id1.0] = self.annotates[id2.0].clone();
                true
            }
            (Type::Infer, Type::Infer) => false,
            (Type::Infer, Type::InferInteger) => {
                self.annotates[id1.0] = self.annotates[id2.0].clone();
                true
            }
            (Type::InferInteger, Type::Bool) => panic!(),
            (Type::InferInteger, Type::Int(_)) => {
                self.annotates[id1.0] = self.annotates[id2.0].clone();
                true
            }
            (Type::InferInteger, Type::Infer) => {
                self.annotates[id2.0] = self.annotates[id1.0].clone();
                true
            }
            (Type::InferInteger, Type::InferInteger) => false,
        }
    }
    pub fn same(&mut self, xs: &[AnnotationId]) {
        for ids in xs.windows(2) {
            self.type_rels.push(TypeRel::Same(ids[0], ids[1]));
        }
    }
    pub fn get_ty(&self, id: AnnotationId) -> Type {
        self.annotates[id.0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotator_1() {
        // 1 + 2
        let mut annotator = Annotator::new();
        let left = annotator.create_annotation(Type::InferInteger);
        let right = annotator.create_annotation(Type::InferInteger);
        let res = annotator.create_annotation(Type::Infer);
        annotator.same(&[res, left, right]);
        annotator.annotate(Type::Int(32));
        let ty_left = annotator.get_ty(left);
        let ty_right = annotator.get_ty(right);
        let ty_res = annotator.get_ty(res);
        assert_eq!(ty_left, Type::Int(32));
        assert_eq!(ty_right, Type::Int(32));
        assert_eq!(ty_res, Type::Int(32));
    }

    #[test]
    fn test_annotator_2() {
        // x + x * 2
        let mut annotator = Annotator::new();
        let id_x = annotator.create_annotation(Type::Infer);
        let id_2 = annotator.create_annotation(Type::InferInteger);
        let x_times_2 = annotator.create_annotation(Type::Infer);
        annotator.same(&[id_x, id_2, x_times_2]);
        let res = annotator.create_annotation(Type::Infer);
        annotator.same(&[id_x, x_times_2, res]);
        annotator.annotate(Type::Int(32));
        assert_eq!(annotator.get_ty(id_x), Type::Int(32));
        assert_eq!(annotator.get_ty(id_2), Type::Int(32));
    }
}
