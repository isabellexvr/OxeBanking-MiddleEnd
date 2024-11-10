CREATE TABLE insurance_types (
    id SERIAL PRIMARY KEY,
    type VARCHAR(50) UNIQUE NOT NULL,
    icon VARCHAR(100) NOT NULL
);

CREATE TABLE insurances (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    contracted BOOLEAN DEFAULT FALSE,
    type_id INTEGER NOT NULL,
    FOREIGN KEY (type_id) REFERENCES insurance_types(id)
);


CREATE TABLE insurance_descriptions (
    id SERIAL PRIMARY KEY,
    insurance_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (insurance_id) REFERENCES insurances(id) ON DELETE CASCADE
);



-- INSURANCE TYPES:


INSERT INTO insurance_types (type, icon)
VALUES
    ('life', 'mdi:cards-heart-outline'),
    ('auto', 'mdi:car-outline'),
    ('travel', 'mdi:airplane-takeoff'),
    ('home', 'mdi:shield-home-outline'),
    ('card', 'mdi:credit-card-outline'),
    ('previdency', 'mdi:piggy-bank-outline');



-- INSURANCES:


-- Seguros do tipo "card"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Seguro Cartão Básico', 15, FALSE, 1),
    ('Seguro Cartão Premium', 30, FALSE, 1);

-- Seguros do tipo "travel"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Seguro Viagem Nacional', 50, FALSE, 2),
    ('Seguro Viagem Internacional', 120, FALSE, 2);

-- Seguros do tipo "life"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Seguro de Vida Básico', 80, FALSE, 3),
    ('Seguro de Vida Familiar', 150, FALSE, 3);

-- Seguros do tipo "home"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Seguro Residencial Básico', 100, FALSE, 4),
    ('Seguro Residencial Completo', 200, FALSE, 4);

-- Seguros do tipo "auto"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Seguro Automotivo Básico', 300, FALSE, 5),
    ('Seguro Automotivo Completo', 500, FALSE, 5);

-- Seguro do tipo "previdency"
INSERT INTO insurances (title, price, contracted, type_id)
VALUES
    ('Previdência Privada', 100, FALSE, 6);


-- DESCRIÇÕES:


-- Inserindo descrições para 'Seguro Cartão Básico' e 'Seguro Cartão Premium'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (1, 'Cobertura contra fraudes'),
    (1, 'Proteção de compras'),
    (1, 'Assistência emergencial'),
    (2, 'Cobertura contra fraudes'),
    (2, 'Proteção de compras e viagens'),
    (2, 'Serviço de concierge 24h');

-- Inserindo descrições para 'Seguro Viagem Nacional' e 'Seguro Viagem Internacional'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (3, 'Cobertura médica'),
    (3, 'Assistência de bagagem'),
    (3, 'Cancelamento de viagem'),
    (4, 'Cobertura médica e odontológica'),
    (4, 'Assistência de bagagem e documentos'),
    (4, 'Cobertura para esportes radicais');

-- Inserindo descrições para 'Seguro de Vida Básico' e 'Seguro de Vida Familiar'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (5, 'Cobertura por invalidez'),
    (5, 'Indenização por falecimento'),
    (5, 'Assistência funeral'),
    (6, 'Cobertura para cônjuge'),
    (6, 'Cobertura para filhos'),
    (6, 'Serviço de suporte psicológico');

-- Inserindo descrições para 'Seguro Residencial Básico' e 'Seguro Residencial Completo'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (7, 'Cobertura contra incêndios'),
    (7, 'Cobertura contra roubos'),
    (7, 'Assistência de manutenção'),
    (8, 'Cobertura contra desastres naturais'),
    (8, 'Cobertura contra roubo e vandalismo'),
    (8, 'Assistência elétrica e hidráulica');

-- Inserindo descrições para 'Seguro Automotivo Básico' e 'Seguro Automotivo Completo'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (9, 'Cobertura contra roubo e furto'),
    (9, 'Assistência 24h'),
    (9, 'Cobertura contra danos a terceiros'),
    (10, 'Cobertura total contra acidentes'),
    (10, 'Carro reserva'),
    (10, 'Cobertura para passageiros');

-- Inserindo descrições para 'Previdência Privada'
INSERT INTO insurance_descriptions (insurance_id, description)
VALUES
    (11, 'Aposentadoria garantida'),
    (11, 'Reserva para sonhos'),
    (11, 'Assistência funerária');

