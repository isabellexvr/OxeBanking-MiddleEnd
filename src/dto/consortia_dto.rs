use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsortiumGetDTO {
    pub id: i32,
    pub nome: String,
    pub descricao: String,
    pub imagem: String,
    pub data_inicio: String,
    pub data_fim: String,
    pub valor_total: u32,
    pub taxa_administrativa: u8,
    pub taxa_fundo_reserva: u8,
    pub qtd_parcelas: u8,
    pub limite_participantes: u8,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsortiumDTO {
    pub nome: String,
    pub descricao: String,
    pub imagem: String,
    pub data_inicio: String,
    pub data_fim: String,
    pub valor_total: u32,
    pub taxa_administrativa: u8,
    pub taxa_fundo_reserva: u8,
    pub qtd_parcelas: u8,
    pub limite_participantes: u8,
    pub status: String,
}

