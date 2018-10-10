extern crate ga_v4_flattener;
#[macro_use]
extern crate neon;

use ga_v4_flattener::{to_delimited, to_flat_json};
use neon::prelude::*;

fn to_delimited_neon(mut cx: FunctionContext) -> JsResult<JsArray> {
    let data = cx.argument::<JsString>(0)?.value();
    let delimieter = cx.argument::<JsString>(1)?.value();

    let rust_vec = to_delimited(&data, &delimieter).unwrap();

    let js_handle = JsArray::new(&mut cx, rust_vec.len() as u32);

    for (index, item) in rust_vec.into_iter().enumerate() {
        let data = cx.string(item);
        js_handle.set(&mut cx, index as u32, data)?;
    }

    Ok(js_handle)
}

fn to_flat_json_neon(mut cx: FunctionContext) -> JsResult<JsString> {
    let data = cx.argument::<JsString>(0)?.value();

    let formatted_data = to_flat_json(&data).unwrap();

    println!("{:?}", formatted_data);
    
    Ok(cx.string(formatted_data.to_string()))
}

register_module!(mut cx, {
    cx.export_function("toDelimited", to_delimited_neon)?;
    cx.export_function("toFlatJsonString", to_flat_json_neon)?;
    Ok(())
});
