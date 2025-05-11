pub type RefSetter<'a, T> = Box<dyn FnOnce(T) + 'a>;

pub fn set_ref<T>(value_ref: &mut Option<T>) -> RefSetter<T> {
    Box::new(move |id| {
        *value_ref = Some(id);
    })
}

pub fn set_nop<T>() -> RefSetter<'static, T> {
    Box::new(|_| {})
}
