use crate::http::HttpClient;

use reqwest::Client;

pub struct ClientBuilder {
    client_id: String,
    client_secret: String,
}

impl ClientBuilder {
    pub fn new(client_id: String, client_secret: String) -> ClientBuilder {
        ClientBuilder {
            client_id,
            client_secret,
        }
    }

    pub fn http(self) -> HttpClientBuilder {
        HttpClientBuilder {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

pub struct HttpClientBuilder {
    client_id: String,
    client_secret: String,
}

impl HttpClientBuilder {
    pub fn default(self) -> HttpClient {
        HttpClient::new(self.client_id, self.client_secret, Default::default())
    }

    pub fn client(self, client: Client) -> HttpClient {
        HttpClient::new(self.client_id, self.client_secret, client)
    }
}
