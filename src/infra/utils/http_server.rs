use axum::Router;

pub async fn run_http_server(app: Router) {
    let listener = create_listener(3000).await;
    axum::serve(listener, app).await.unwrap();
}

pub async fn create_listener(port: u16) -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::infra::utils::http_server::{create_listener, run_http_server};
    use axum::{Router, routing::get};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_create_listener_success() {
        let listener = create_listener(0).await;

        // Verifica se o endereço local foi atribuído corretamente
        let addr = listener.local_addr().unwrap();
        assert_eq!(addr.ip().to_string(), "0.0.0.0");
    }

    #[tokio::test]
    async fn test_run_http_server_execution() {
        // 1. Criar um App simples para o teste
        let app = Router::new().route("/health", get(|| async { "ok" }));

        // 2. Rodar o servidor em background
        // Usamos spawn para não bloquear o teste aqui
        let server_handle = tokio::spawn(async move {
            run_http_server(app).await;
        });

        // 3. Pequena pausa para garantir que o servidor subiu
        sleep(Duration::from_millis(100)).await;

        // 4. Tentar conectar ao servidor
        let client = reqwest::Client::new();
        let res = client
            .get("http://127.0.0.1:3000/health")
            .send()
            // Se o servidor não subiu, isso aqui vai falhar
            .await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 200);

        // 5. Cleanup: Abortar a tarefa do servidor (já que run_http_server não tem shutdown suave)
        server_handle.abort();
    }
}
