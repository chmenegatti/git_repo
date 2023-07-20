use std::env;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Debug, Serialize, Deserialize)]
struct User {
  name: Option<String>,
  location: Option<String>,
  company: Option<String>,
  public_repos: u32,
  bio: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
  dotenv::from_filename(".env").ok();
  
  let token = env::var("GITHUB_TOKEN").expect("Erro ao obter o token de autenticação.");
  
  let mut username = String::new();
  print!("Digite o nome do usuário do GitHub: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut username).unwrap();
  let username = username.trim();
  
  let client = Client::new();
  
  let url = format!("https://api.github.com/users/{}", username);
  let mut response = client
  .get(&url)
  .header("Authorization", format!("Bearer {}", token))
  .send()
  .await?;
  
  if response.status().is_success() {
    let user: User = response.body_json().await?;
    println!("Nome: {}", user.name.unwrap_or("Desconhecido".to_string()));
    println!("Localização: {}", user.location.unwrap_or("Desconhecida".to_string()));
    println!("Empresa: {}", user.company.unwrap_or("Desconhecida".to_string()));
    println!("Repositórios Públicos: {}", user.public_repos);
    println!("Bio: {}", user.bio.unwrap_or("Desconhecida".to_string()));
  } else {
    println!("Erro ao obter os dados do usuário: status {}", response.status());
  }
  
  Ok(())
}
