use embedded_digi as digi;

#[test]
fn read() {
    let t1 = MockInput::new(true);
    let f1 = MockInput::new(false);
    let t2 = MockInput::new(true);
    let f2 = MockInput::new(false);

    assert_eq!(digi::read!(t1), true);
    assert_eq!(digi::read!(f1), false);
    assert_eq!(digi::read!(t1, f1, t2, f2), (true, false, true, false));

    assert_eq!(digi::read!(any false; t1, f1, t2, f2), true);
    assert_eq!(digi::read!(any false; t1, t2), false);
    assert_eq!(digi::read!(any true; t1, f1, t2, f2), true);
    assert_eq!(digi::read!(any true; f1, f2), false);

    assert_eq!(digi::read!(all false; t1, f1, t2, f2), false);
    assert_eq!(digi::read!(all false; f1, f2), true);
    assert_eq!(digi::read!(all true; t1, f1, t2, f2), false);
    assert_eq!(digi::read!(all true; t1, t2), true);

    assert_eq!(digi::read!(count true; t1, f1, t2, f2), 2);
    assert_eq!(digi::read!(count false; t1, f1, t2, f2), 2);
    assert_eq!(digi::read!(count true; t1, f1, f2), 1);
    assert_eq!(digi::read!(count false; t1, f1, t2), 1);

    assert_eq!(digi::read!(u8; t1, f2, t2, f2), 10);
    assert_eq!(digi::read!(u8; t1, t2), 3);
    assert_eq!(digi::read!(u8; f1, f2), 0);
}

#[test]
fn write() {
    let mut o1 = MockOutput::new();
    let mut o2 = MockOutput::new();
    let mut o3 = MockOutput::new();
    let mut o4 = MockOutput::new();

    digi::write!(o1 => 1);
    digi::write!(o2 => true);
    assert_eq!((o1.v, o2.v), (true, true));

    digi::write!(o1 => 0);
    digi::write!(o2 => false);
    assert_eq!((o1.v, o2.v), (false, false));

    let value = true;
    digi::write!(o1 => value);
    assert_eq!(o1.v, true);

    digi::write!(o1, o2, o3, o4 => true);
    assert_eq!((o1.v, o2.v, o3.v, o4.v), (true, true, true, true));

    digi::write!(o2, o3 => false);
    assert_eq!((o2.v, o3.v), (false, false));

    let value = false;
    digi::write! {
        o1 => false;
        o2, o3 => true;
        o4 => value;
    };
    assert_eq!((o1.v, o2.v, o3.v, o4.v), (false, true, true, false));

    digi::write!(o1, o2, o3, o4 => 4 bit => 9);
    assert_eq!((o1.v, o2.v, o3.v, o4.v), (true, false, false, true));

    digi::write!(o1, o2, o3, o4 => 8 bit => 0x0F);
    assert_eq!((o1.v, o2.v, o3.v, o4.v), (false, false, false, false));

    digi::write!(o1, o2, o3, o4 => 8 bit => 0xF0);
    assert_eq!((o1.v, o2.v, o3.v, o4.v), (true, true, true, true));

    digi::write!(o1, o2, o3, o4 => false);
    digi::write! {
        o1, o2, o3 => 3 bit => 5;
        o2, o4 => 2 bit => 3;
    };

    assert_eq!((o1.v, o2.v, o3.v, o4.v), (true, true, true, true));
}

pub struct MockInput {
    pub v: bool,
}

impl MockInput {
    pub fn new(val: bool) -> Self {
        Self { v: val }
    }

    pub fn is_high(&self) -> Result<bool, ()> {
        Ok(self.v)
    }
}

pub struct MockOutput {
    pub v: bool,
}

impl MockOutput {
    pub fn new() -> Self {
        Self { v: false }
    }

    pub fn set_high(&mut self) -> Result<(), ()> {
        self.v = true;
        Ok(())
    }

    pub fn set_low(&mut self) -> Result<(), ()> {
        self.v = false;
        Ok(())
    }
}
