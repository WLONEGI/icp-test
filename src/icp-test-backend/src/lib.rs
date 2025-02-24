use std::cell::RefCell;

pub struct Data {
    message: String,
}

thread_local! {
    static DATA: RefCell<Data> = RefCell::new(Data {message: String::new()});
}

#[ic_cdk::query(name="getMessage")]
fn get_message() -> String {
    DATA.with(|data| {
      data.borrow().message.clone()
    })
}

#[ic_cdk::update(name="setMessage")]
fn set_message(message: String) -> () {
    DATA.with(|data| {
      data.borrow_mut().message = message;
    });
}
