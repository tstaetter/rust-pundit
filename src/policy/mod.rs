mod simple_crud;

pub use simple_crud::*;

/// Basic enum for actions
pub enum Action {
    SimpleCrudAction(SimpleCrudAction),
}
