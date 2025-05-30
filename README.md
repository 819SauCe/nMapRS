# nMapRS

**nMapRS** é um scanner de portas TCP escrito em Rust puro, sem dependências externas, focado em desempenho e simplicidade.

## 🔍 Recursos

* **Zero dependências**: usa apenas a biblioteca padrão (`std`).
* **Multithreading dinâmica**: detecta e utiliza todos os núcleos disponíveis.
* **Intervalo de portas**: varre desde `<porta_inicial>` até `<porta_final>`, ou apenas uma porta quando especificada uma única.
* **Timeout configurável**: padrão de 500 ms (ajustável no código).
* **IPv4 e IPv6**: suporta qualquer endereço roteável.

## 🚀 Instalação

1. Clone o repositório:

   ```bash
   git clone https://github.com/819SauCe/nMapRS.git
   cd nMapRS
   ```
2. Compile com Cargo:

   ```bash
   cargo build --release
   ```

## ⚙️ Uso

```bash
cargo run -- <IP> <PORTA_INICIAL> [PORTA_FINAL]
```

* `<IP>`: endereço IPv4 ou IPv6.
* `<PORTA_INICIAL>`: porta de início do scan.
* `[PORTA_FINAL]`: (opcional) porta final. Se omitido, varre apenas `<PORTA_INICIAL>`.

**Exemplos:**

```bash
# Scan de 1 a 1024 no localhost
cargo run -- 127.0.0.1 1 1024

# Scan apenas da porta 80 em um IP público
cargo run -- 8.8.8.8 80
```

## 📝 Personalização

* **Timeout**: altere `Duration::from_millis(500)` no código.
* **Threads**: por padrão usa todos os núcleos, mas você pode modificar `available_parallelism()` para um valor fixo.
* **CLI avançada**: integrar `clap` para parsing de opções (`--threads`, `--timeout`).

## 💡 Dicas de aprimoramento

* Adicionar **barra de progresso** com `indicatif`.
* Implementar **banner grabbing** usando `socket2`.
* Migrar para **async** com `tokio` para escalabilidade maior.
* Suporte a **lista de alvos** via arquivo de texto.

## 🤝 Contribuição

1. Fork deste repositório.
2. Crie uma branch para sua feature: `git checkout -b feature/nome`
3. Commit suas mudanças sem comentários no código: `git commit -m "Adicionar recurso X"`
4. Abra um Pull Request.

## 📄 Licença

Este projeto está licenciado sob a [MIT License](./LICENSE).
