//! Resolve appropriate policy

use crate::{Policy, PunditAuthenticatable, PunditResult};

macro_rules! find_policy {
    ($t: ty) => {
        <$t as Policy>::try_new()
    }
}

fn get_policy_name<T>(_: &T) -> String {
    format!("{}Policy", std::any::type_name::<T>().split("::").last().unwrap())
}

#[derive(Debug)]
struct PolicyFinder<T, P> where T: PunditAuthenticatable + std::fmt::Debug, P: Policy {
    object: Option<T>,
    policies: Vec<P>,
}

impl <T, P> PolicyFinder<T, P> where T: PunditAuthenticatable + std::fmt::Debug, P: Policy {
    pub fn new(object: T, policies: Vec<P>) -> PunditResult<Self> {
        Ok(Self { object: Some(object), policies })
    }

    fn find(&self, obj: T) -> PunditResult<P> {
        match find_policy!(Self) {
            Ok(policy) => Ok(policy),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_policy() -> PunditResult<()> {
        // Test finder with a basic string
        impl PunditAuthenticatable for String {}
        #[derive(Debug)]
        struct TestStruct {}
        impl Policy for TestStruct {
            fn try_new() -> PunditResult<Self> where Self: Sized {
                Ok(TestStruct {})
            }
        }
        impl PunditAuthenticatable for TestStruct {}


        let record = String::from("some value");
        let finder= PolicyFinder::new(record, vec![TestStruct {}]).unwrap();
        let s = TestStruct::try_new().unwrap();
        let policy_name = get_policy_name(&s);
        let policy = find_policy!(TestStruct).unwrap();

        println!("type name: {}", get_policy_name(&s));
        println!("policy: {:?}", policy);

        Ok(())
    }
}