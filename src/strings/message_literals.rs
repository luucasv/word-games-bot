use super::string_formatter::{StringFormatter, StringLiteral};

pub struct Welcome;
impl StringFormatter<&str> for Welcome {
    fn pt(user_name: &str) -> String {
        format! {
"Olá {name}!

Eu sou um moderar de partidas de jogos de palavras.
Para começar, me adicione em um grupo e envie o comando /newgame.",
            name = user_name
        }
    }
}

pub struct GameLobby;
impl StringFormatter<(usize, &str)> for GameLobby {
    fn pt(args: (usize, &str)) -> String {
        format! {
"🔊 Uma nova partida de Comunicação Secreta foi criada\\!
Use os botões abaixo para entrar e sair do jogo\\.
💡 Quando for possível, surgirá um botão para iniciar o jogo

{count} jogador{plural} \\(mínimo 4\\):
{mentions}",
            count = args.0,
            plural = if args.0 == 1 {""} else {"es"},
            mentions = args.1
        }
    }
}

pub struct JoinPrivate;
impl StringFormatter<&str> for JoinPrivate {
    fn pt(group_name: &str) -> String {
        format! {
            "Você se juntou a uma partida de Comunicação Secreta no grupo \"{group_name}\".",
            group_name=group_name
        }
    }
}

pub struct NotInGroupError;
impl StringLiteral for NotInGroupError {
    fn pt() -> &'static str {
        "Este commando só é valido em grupos!"
    }
}

pub struct AlreadyInGameError;
impl StringLiteral for AlreadyInGameError {
    fn pt() -> &'static str {
        "Já existe um jogo neste grupo 😕"
    }
}
