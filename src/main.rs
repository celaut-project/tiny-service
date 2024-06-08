use warp::Filter;

#[tokio::main]
async fn main() {
    // Define la ruta que retorna "Hello, world!"
    let hello = warp::path::end().map(|| "Hello, world!");

    // Inicia el servidor en el puerto 3030
    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
