use std::io;
use std::net::TcpStream;
use std::time::Duration;
use clap::Parser;

/// Escáner de puertos con soporte de flags estilo nmap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Dirección IP objetivo
    #[arg(short = 'i', long)]
    ip: String,

    /// Puerto o rango de puertos (ej: 80 o 20-100)
    #[arg(short = 'p', long, default_value = "1-1024")]
    ports: String,

    /// Timeout en milisegundos
    #[arg(short = 't', long, default_value = "500")]
    timeout: u64,

    /// Mostrar solo puertos abiertos
    #[arg(short = 'o', long)]
    open_only: bool,
}

// Función para parsear un rango de puertos (ej: "20-100")
fn parsear_rango(rango: &str) -> Vec<u16> {
    let partes: Vec<&str> = rango.split('-').collect();
    if partes.len() == 2 {
        let inicio = partes[0].parse::<u16>().unwrap_or(1);
        let fin = partes[1].parse::<u16>().unwrap_or(1024);
        return (inicio..=fin).collect();
    }
    vec![rango.parse::<u16>().unwrap_or(1)]
}

// Estados posibles del puerto
#[derive(Debug)]
enum PortStatus {
    Open,
    Closed,
    Filtered,
}

// Escaneo TCP simple
fn escanear_puerto(ip: &str, puerto: u16, timeout_ms: u64) -> PortStatus {
    let direccion = format!("{}:{}", ip, puerto);
    match TcpStream::connect_timeout(&direccion.parse().unwrap(), Duration::from_millis(timeout_ms)) {
        Ok(_) => PortStatus::Open,
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                PortStatus::Filtered
            } else {
                PortStatus::Closed
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let puertos = parsear_rango(&args.ports);

    if args.open_only {
        let mut ports_opened = Vec::new();

        for puerto in &puertos {
            if let PortStatus::Open = escanear_puerto(&args.ip, *puerto, args.timeout) {
                println!("✅ Puerto {} ABIERTO", puerto);
                ports_opened.push(*puerto);
            }
        }

        if ports_opened.is_empty() {
            println!("❌ No se encontraron puertos abiertos en el rango especificado.");
        }
    } else {
        for puerto in &puertos {
            match escanear_puerto(&args.ip, *puerto, args.timeout) {
                PortStatus::Open => {
                    println!("✅ Puerto {} ABIERTO", puerto);
                }
                PortStatus::Filtered => {
                    println!("⚠️  Puerto {} FILTRADO o sin respuesta", puerto);
                }
                PortStatus::Closed => {
                    println!("❌ Puerto {} CERRADO", puerto);
                }
            }
        }
    }
}
