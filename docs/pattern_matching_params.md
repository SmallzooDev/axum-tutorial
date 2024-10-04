# pattern matching을 통한 파라미터 추출
```rust
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!!!");

    Html(format!("Hello <strong>{name}</strong>"))
}
```

이부분을 보다가 이해가 안가서 정리

## 일반적인 Rust 패턴 매칭을 이용한 함수 파라미터

```rust
// Option 타입을 매개변수로 받는 함수
fn print_some_value(option: Option<i32>) {
    // 패턴 매칭을 사용하여 `Some`에서 값을 추출
    match option {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value provided"),
    }
}

fn main() {
    // Some(42)를 넘기면 값이 출력됨
    print_some_value(Some(42));
    // None을 넘기면 값이 없다는 메시지가 출력됨
    print_some_value(None);
}
```

## 패턴 매칭을 함수 파라미터에서 사용하기

```rust
fn print_some_value(Some(value): Option<i32>) {
    // `Some(value)`라는 패턴 매칭을 통해 `Option` 내부의 값을 바로 추출함
    println!("The value is: {}", value);
}

fn main() {
    // Some(42)를 넘기면 값이 출력됨
    print_some_value(Some(42));
    // None을 넘기면 패턴 매칭이 실패하여 함수 실행 전 패닉 발생
    // print_some_value(None); 
}
```

## 실제 axum의 Query 추출기 코드

```rust
#[derive(Debug, Clone, Default)]
pub struct Query<T>(pub T);

// `Query`가 `FromRequest`를 구현하여 요청으로부터 추출될 수 있도록 함
#[async_trait]
impl<T, S, B> FromRequest<S, B> for Query<T>
where
    T: DeserializeOwned, // `T`는 반드시 `serde::Deserialize`를 구현하고 있어야 함
    B: Send,
{
    type Rejection = QueryRejection;

    async fn from_request(req: &mut RequestParts<B>, _: &S) -> Result<Self, Self::Rejection> {
        // 요청의 쿼리 파라미터를 문자열로 가져옴
        let query_str = req.uri().query().unwrap_or_default();

        // 쿼리 문자열을 `serde`를 이용해 `T`로 파싱
        let value = serde_urlencoded::from_str(query_str)
            .map_err(QueryRejection::FailedToDeserializeQueryString)?;

        Ok(Query(value)) // 파싱된 값을 `Query`로 감싸서 반환
    }
}
```

