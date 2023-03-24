mod server;
mod client;

fn main() {
    // Démarrer le serveur dans un thread séparé
    std::thread::spawn(|| {
        server::main().unwrap();
    });

    // Démarrer le client dans le thread principal
    client::main().unwrap();
}
