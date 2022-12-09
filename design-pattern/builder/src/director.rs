struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<Builder>) -> Director {
        Director { builder: builder }
    }

    fn construct(&mut self) {
        self.builder.produce_part_a();
        self.builder.produce_part_b();
        self.builder.produce_part_c();
    }
}
