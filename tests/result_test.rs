use std::fmt;
use std::fmt::Debug;

pub trait Report<Q> {
    fn report(self, q: Q);
}

pub struct ReportDump;

impl<O, E> Report<ResultTest<O, E>> for ReportDump {
    #[track_caller]
    fn report(self, q: ResultTest<O, E>) {
        if q.fail {
            for msg in q.msg.iter() {
                println!("{}", msg);
            }
            panic!();
        }
    }
}

pub struct ReportValues;

impl<O, E> Report<ResultTest<O, E>> for ReportValues
where
    O: Debug,
    E: Debug,
{
    fn report(self, q: ResultTest<O, E>) {
        match (q.ok.as_ref(), q.err.as_ref()) {
            (Some(ok), None) => {
                println!("{:?}", ok);
            }
            (None, Some(err)) => {
                println!("{:?}", err);
            }
            _ => unreachable!(),
        }
        for msg in q.msg.iter() {
            println!("    {}", msg);
        }
    }
}

pub struct ResultTest<O, E> {
    pub ok: Option<O>,
    pub err: Option<E>,
    pub fail: bool,
    pub msg: Vec<String>,
}

impl<O, E> ResultTest<O, E>
where
    O: Debug,
    E: Debug,
{
    #[allow(dead_code)]
    #[must_use]
    pub fn new(result: Result<O, E>) -> Self {
        match result {
            Ok(v) => Self {
                ok: Some(v),
                err: None,
                fail: false,
                msg: Default::default(),
            },
            Err(v) => Self {
                ok: None,
                err: Some(v),
                fail: false,
                msg: Default::default(),
            },
        }
    }

    #[allow(dead_code)]
    #[track_caller]
    pub fn q<Q: Report<Self>>(self, q: Q) {
        q.report(self);
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn ok(mut self) -> Self {
        if self.err.is_some() {
            self.msg.push("expected err but was ok".into());
            self.fail = true;
        }
        self
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn err(mut self) -> Self {
        if self.ok.is_some() {
            self.msg.push("expected ok but was err".into());
            self.fail = true;
        }
        self
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn test<T: Debug + ?Sized>(mut self, cmp: impl Fn(&O, &T) -> bool, test: &T) -> Self {
        if let Some(ok) = &self.ok {
            if !cmp(ok, test) {
                self.msg
                    .push(format!("test failed: {:?} <> {:?}", ok, test));
                self.fail = true;
            } else {
                // ok
            }
        } else {
            self.msg.push("expected ok but was err".into());
            self.fail = true;
        }
        self
    }
}

impl<E> ResultTest<String, E> {
    #[allow(dead_code)]
    #[must_use]
    pub fn str(mut self, test: &str) -> Self {
        if let Some(ok) = &self.ok {
            if ok.as_str() != test {
                self.msg
                    .push(format!("test failed: {:?} <> {:?}", ok, test));
                self.fail = true;
            } else {
                // ok
            }
        } else {
            self.msg.push("expected ok but was err".into());
        }
        self
    }
}

#[allow(dead_code)]
#[must_use]
pub fn test(r: Result<String, fmt::Error>) -> ResultTest<String, fmt::Error> {
    ResultTest::new(r)
}

#[allow(dead_code)]
#[must_use]
pub fn test_ok<T: Debug>(r: T) -> ResultTest<T, ()> {
    ResultTest::new(Ok(r))
}
