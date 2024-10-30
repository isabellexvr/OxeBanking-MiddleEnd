# Como rodar?
- cargo clean
- cargo build
- cargo run

# Como rodar para desenvolvimento? (reiniciar o server a cada mudança)
- cargo install cargo-watch
- cargo watch -x 'run'




tabela addresses:
    id: int (PK),
    zip_code: String,
    city: String,
    state: String,
    uf: String,
    street: String,
    number: String,
    complement: String ou null,
    is_main: boolean

tabela users:
    id: int (PK),
    full_name: String,
    profile_pic: String, 
    cpf: String, (unique)
    birthdate: String,
    marital_status: String,
    gross_mensal_income: int, (utilizaremos int como valor em centavos)
    email: String, (unique)
    phone_number: String,
    is_admin: bool,
    is_blocked: bool,
    user_password: String,
    created_at: date,
    updated_at: date,
    address_id: int (FK)


POST: create_new_user "/users/new"
- requisição:
	- body: {
    "full_name": "João Silva",
    "profile_pic": "https://example.com/profile.jpg",
    "cpf": "123.456.789-00",
    "birthdate": "1990-05-25",
    "marital_status": "Solteiro",
    "gross_mensal_income": 5000.0,
    "email": "joao.silva@example.com",
    "phone_number": "+55 11 91234-5678",
    "is_admin": false,
    "is_blocked": false,
    "user_password": "senha123",
    "address": {
        "zip_code": "12345-678",
        "city": "São Paulo",
        "state": "SP",
        "uf": "SP",
        "street": "Rua das Flores",
        "number": "100",
        "complement": "Apartamento 202"
      }
    }
- resposta: 201 (created)

GET: get_user_by_cpf "/users/{cpf}"
- requisição:
	params: cpf (string)
- resposta: 
    {
        "full_name": "João Silva",
        "profile_pic": "https://example.com/profile.jpg",
        "cpf": "123.456.789-00",
        "birthdate": "1990-05-25",
        "marital_status": "Solteiro",
        "gross_mensal_income": 5000.0,
        "email": "joao.silva@example.com",
        "phone_number": "+55 11 91234-5678",
        "is_admin": false,
        "is_blocked": false,
        "user_password": "senha123",
        "created_at": "2024-10-25T18:00:00Z",
        "updated_at": "2024-10-25T18:30:00Z",
        "address": {
            "zip_code": "12345-678",
            "city": "São Paulo",
            "state": "SP",
            "uf": "SP",
            "street": "Rua das Flores",
            "number": "100",
            "complement": "Apartamento 202"
        }
    }


GET: get_user_by_id "/users/{id}"
- requisição:
	params: id (int)
- resposta: 
    {
        "full_name": "João Silva",
        "profile_pic": "https://example.com/profile.jpg",
        "cpf": "123.456.789-00",
        "birthdate": "1990-05-25",
        "marital_status": "Solteiro",
        "gross_mensal_income": 5000.0,
        "email": "joao.silva@example.com",
        "phone_number": "+55 11 91234-5678",
        "is_admin": false,
        "is_blocked": false,
        "user_password": "senha123",
        "created_at": "2024-10-25T18:00:00Z",
        "updated_at": "2024-10-25T18:30:00Z",
        "address": {
            "zip_code": "12345-678",
            "city": "São Paulo",
            "state": "SP",
            "uf": "SP",
            "street": "Rua das Flores",
            "number": "100",
            "complement": "Apartamento 202"
        }
    }

GET: get_user_addresses "/users/addresses/{user_id}"
- requisição:
	params: user_id (int)
- resposta: 
    {
        "full_name": "João Silva",
        "profile_pic": "https://example.com/profile.jpg",
        "cpf": "123.456.789-00",
        "birthdate": "1990-05-25",
        "marital_status": "Solteiro",
        "gross_mensal_income": 5000.0,
        "email": "joao.silva@example.com",
        "phone_number": "+55 11 91234-5678",
        "is_admin": false,
        "is_blocked": false,
        "user_password": "senha123",
        "created_at": "2024-10-25T18:00:00Z",
        "updated_at": "2024-10-25T18:30:00Z",
        "address": {
            "zip_code": "12345-678",
            "city": "São Paulo",
            "state": "SP",
            "uf": "SP",
            "street": "Rua das Flores",
            "number": "100",
            "complement": "Apartamento 202"
        }
    }