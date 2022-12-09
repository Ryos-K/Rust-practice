mod product_a;

pub trait Builder {
    type OutPutType;
    fn set_type(&mut self, my_type: MyType);
    fn set_partA(&mut self, part: PartA);
    fn set_partB(&mut self, part: PartB);
    fn build(self) -> Self::OutPutType;
}
