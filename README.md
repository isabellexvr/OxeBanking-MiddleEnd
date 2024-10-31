# Como rodar?
- **cargo clean**  
  Limpa os artefatos de compilação.
- **cargo build**  
  Compila o projeto.
- **cargo run**  
  Executa o projeto.

# Como rodar para desenvolvimento? (reiniciar o server a cada mudança)
- **cargo install cargo-watch**  
  Instala a ferramenta cargo-watch.
- **cargo watch -x 'run'**  
  Observa mudanças no código e reinicia o servidor automaticamente.

# Estrutura do Projeto

- `src/main.rs`  
  > Ponto de entrada da aplicação.
- `src/lib.rs`  
  > Biblioteca principal.
- `src/controllers/`  
  > Controladores da aplicação.
- `src/dto/`  
  > Objetos de transferência de dados.
- `src/errors/`  
  > Tratamento de erros.
- `src/microsservices/`  
  > Microsserviços.
- `src/middleware/`  
  > Middleware de autenticação.
- `src/middlewares/`  
  > Outros middlewares.
- `src/models/`  
  > Modelos de dados.
- `src/routes/`  
  > Definições de rotas.
- `src/services/`  
  > Lógica de negócios.
- `src/utils/`  
  > Utilitários e funções auxiliares.
- `Cargo.toml`  
  > Arquivo de configuração do Cargo, com todas as dependências necessárias.