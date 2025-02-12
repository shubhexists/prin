use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Select};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[command(name = "Prin")]
#[command(version = "1.0")]
#[command(about = "A simple reverse proxy CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the reverse proxy server
    Start,
    /// Configure the reverse proxy
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Add a new route
    Add,
    /// Edit an existing route
    Edit,
    /// Delete an existing route
    Delete,
}

#[derive(Serialize, Deserialize, Clone)]
struct ProxyConfig {
    routes: HashMap<String, String>,
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("Failed to find config directory")
        .join("prin/config.json")
}

fn load_config() -> ProxyConfig {
    let config_path = get_config_path();
    if !config_path.exists() {
        let default_config = ProxyConfig {
            routes: HashMap::new(),
        };

        if let Some(config_dir) = config_path.parent() {
            fs::create_dir_all(config_dir).expect("Failed to create config directory");
        }

        let config_data = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");
        fs::write(&config_path, config_data).expect("Failed to write default config file");

        println!("Created new config file at {:?}", config_path);
        return default_config;
    }

    let config_data = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", config_path));
    serde_json::from_str(&config_data).expect("Invalid config format")
}

fn save_config(config: &ProxyConfig) {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap();
    fs::create_dir_all(config_dir).expect("Failed to create config directory");
    let config_data = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(config_path, config_data).expect("Failed to write config file");
}

fn add_route(config: &mut ProxyConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Adding New Route ===");

    let prefix: String = Input::new()
        .with_prompt("Enter route prefix (e.g., /api)")
        .interact_text()?;

    let target: String = Input::new()
        .with_prompt("Enter target URL (e.g., http://localhost:3000)")
        .interact_text()?;

    if Confirm::new()
        .with_prompt(format!("Add route: {} → {}?", prefix, target))
        .interact()?
    {
        config.routes.insert(prefix, target);
        println!("Route added successfully!");
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

fn edit_route(config: &mut ProxyConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Editing Route ===");

    let routes: Vec<&String> = config.routes.keys().collect();
    if routes.is_empty() {
        println!("No routes found. Please add a route first.");
        return Ok(());
    }

    let selection = Select::new()
        .with_prompt("Select route to edit")
        .items(&routes)
        .interact()?;

    let selected_prefix = routes[selection].clone();
    let current_target = &config.routes[&selected_prefix];

    println!("Current target: {}", current_target);
    let new_target: String = Input::new()
        .with_prompt("Enter new target URL")
        .with_initial_text(current_target)
        .interact_text()?;

    if Confirm::new()
        .with_prompt(format!(
            "Update route {} → {}?",
            selected_prefix, new_target
        ))
        .interact()?
    {
        config.routes.insert(selected_prefix.clone(), new_target);
        println!("Route updated successfully!");
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

fn delete_route(config: &mut ProxyConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Deleting Route ===");

    let routes: Vec<&String> = config.routes.keys().collect();
    if routes.is_empty() {
        println!("No routes found. Nothing to delete.");
        return Ok(());
    }

    let selection = Select::new()
        .with_prompt("Select route to delete")
        .items(&routes)
        .interact()?;

    let selected_prefix = routes[selection].clone();

    if Confirm::new()
        .with_prompt(format!("Delete route: {}?", selected_prefix))
        .interact()?
    {
        config.routes.remove(&selected_prefix);
        println!("Route deleted successfully!");
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

async fn handle_request(
    client_ip: IpAddr,
    mut req: Request<Body>,
    config: Arc<ProxyConfig>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    for (prefix, target) in &config.routes {
        if path.starts_with(prefix) {
            let new_path = &path[prefix.len()..];
            let new_uri = format!("{}{}", target, new_path);
            *req.uri_mut() = new_uri.parse().unwrap();

            match hyper_reverse_proxy::call(client_ip, target, req).await {
                Ok(response) => return Ok(response),
                Err(_error) => {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap())
                }
            }
        }
    }

    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start => {
            let config = Arc::new(load_config());
            let bind_addr = "127.0.0.1:8000";
            let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

            let make_svc = make_service_fn(move |conn: &AddrStream| {
                let remote_addr = conn.remote_addr().ip();
                let config = Arc::clone(&config);

                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let config = Arc::clone(&config);
                        handle_request(remote_addr, req, config)
                    }))
                }
            });

            println!("Running server on {:?}", addr);
            let server = Server::bind(&addr).serve(make_svc);

            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        }
        Commands::Config(config_command) => {
            let mut config = load_config();
            let result = match config_command {
                ConfigCommands::Add => add_route(&mut config),
                ConfigCommands::Edit => edit_route(&mut config),
                ConfigCommands::Delete => delete_route(&mut config),
            };

            if let Err(e) = result {
                eprintln!("Error: {}", e);
            } else {
                save_config(&config);
            }
        }
    }
}
