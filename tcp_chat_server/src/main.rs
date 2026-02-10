// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;


/// Función de procesamiento del mensaje (puedes modificarla)
fn procesar_mensaje(mensaje: &str, addr: std::net::SocketAddr) -> String {
    // Ejemplo simple: devolvemos el mensaje con prefijo
    format!("{} - {}: {}\n", "<TIMESPAN>", addr, mensaje)
}

fn handle_connection(socket_cliente: TcpStream, clientes: Arc<Mutex<Vec<TcpStream>>>) {
    let addr = socket_cliente.peer_addr().unwrap_or_else(|_| "[desconocido]".parse().unwrap());
    println!("Iniciando manejo de cliente: {}", addr);

    let mut reader = BufReader::new(&socket_cliente);

    loop {
        let mut linea = String::new();

        // Intentamos leer una línea
        match reader.read_line(&mut linea) {
            Ok(0) => {
                // Conexión cerrada por el cliente
                println!("Cliente {} se desconectó (EOF)", addr);
                break;
            }
            Ok(_) => {
                let mensaje = linea.trim().to_string();
                if mensaje.is_empty() {
                    continue;
                }

                println!("Recibido de {}: {}", addr, mensaje);

                let mensaje_procesado = procesar_mensaje(&mensaje, addr);

                // Enviar a todos los demás clientes
                let mut clientes_lock = clientes.lock().unwrap();

                // Eliminamos clientes que ya no están conectados
                clientes_lock.retain_mut(|cliente| {
                    if cliente.peer_addr().is_err() {
                        return false;
                    }

                    // No enviamos al remitente
                    if let Ok(cliente_addr) = cliente.peer_addr() {
                        if cliente_addr == addr {
                            return true;
                        }
                    }

                    // Intentamos enviar
                    match cliente.write_all(mensaje_procesado.as_bytes()) {
                        Ok(_) => true,
                        Err(_) => {
                            println!("Error al enviar a cliente, se removerá");
                            false
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error al leer del cliente {}: {}", addr, e);
                break;
            }
        }
    }

    // Limpieza final: eliminar cliente de la lista
    {
        let mut clientes_lock = clientes.lock().unwrap();
        clientes_lock.retain(|cliente| {
            if let Ok(cliente_addr) = cliente.peer_addr() {
                cliente_addr != addr
            } else {
                false
            }
        });
    }

    println!("Cliente {} terminado y removido", addr);
}

const SERVER_IP: &str = "0.0.0.0";
const SERVER_PORT: u16 = 4343;
const MAX_CLIENTS: usize = 10;

fn main() -> std::io::Result<()> {
    // First we create a socket address for the server to bind to
    let tcp_listener = TcpListener::bind(format!("{}:{}", SERVER_IP, SERVER_PORT))?;
    println!("Server listening on {}:{}", SERVER_IP, SERVER_PORT);

    // We create a vector to hold the client connections and a mutex to protect it
    let clients:Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    // We enter a loop to accept incoming client connections
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(client_socket) => {
                println!("New client connected: {}", client_socket.peer_addr()?);

                // We add the new client to the clients vector, ensuring we don't exceed the maximum number of clients
                {
                    let mut clients_lock = clients.lock().unwrap();
                    if clients_lock.len() >= MAX_CLIENTS {
                        println!("Maximum clients reached. Rejecting connection from {}", client_socket.peer_addr()?);
                        continue;
                    }
                    clients_lock.push(client_socket.try_clone()?);
                }

                // We clone the clients Arc to move it into the new thread
                let clients_clone = Arc::clone(&clients);

                // Then we spawn a new thread to handle the communication with the new client
                thread::spawn(move || {
                    handle_connection(client_socket, clients_clone);
                });
            },
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}