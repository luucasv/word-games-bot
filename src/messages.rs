use tbot::types::User;

pub fn welcome(user: &Option<User>) -> String {
    let name = match user {
        Some(user) => user.first_name.as_str(),
        None => "",
    };
    format!(
        "
Olá {name}!
  
Eu vou moderar partidas de jogos de palavras.
Para começar, me adicione em um grupo e envie o comando /newgame.
  ",
        name = name
    )
}
