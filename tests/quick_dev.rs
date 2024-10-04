#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Axum!!!").await?.print().await?;
    hc.do_get("/hello2/Axum!!!").await?.print().await?;

    Ok(())
}
