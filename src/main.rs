use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    let path = format!(
        "{}{}",
        "https://registry.npmjs.org",
        event["rawPath"].as_str().unwrap_or("")
    );
    println!("Path: {}", path);
    // TODO: use stream instead of this
    let resp = reqwest::get(path).await?.json::<Value>().await?;

    Ok(resp)
}
