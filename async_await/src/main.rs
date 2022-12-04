// 'foo()' returns a type that implements 'Future<Output = u8>'.
// 'foo().await' will result in a value of type 'u8'.
async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    // This 'async' block results in a type that implements
    // 'Future<Output = u8>'.
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

fn main() {
    println!("Hello, world!");
}
