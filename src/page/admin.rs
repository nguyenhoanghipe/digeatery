use dioxus::prelude::*;
use time::Date;
use crate::api::add_available_order_date;
use crate::common::ui::date_picker::DatePicker;

#[component]
pub fn Admin() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);

    let handle_date_add =  move |_| async move {
      if let Some(date) = selected_date() {
          let _ = add_available_order_date(date).await;

        }
    };


    rsx! {
        DatePicker { selected_date, on_value_change: move |v| selected_date.set(v)}
        button {  onclick: handle_date_add, "add"}
    }
}
