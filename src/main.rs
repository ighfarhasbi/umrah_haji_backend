use umrah_haji;

#[tokio::main]
async fn main() {
    umrah_haji::run().await.unwrap();
}
