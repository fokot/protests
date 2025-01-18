CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    name TEXT,
    login_code TEXT,
    login_code_created TIMESTAMP,
    created TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS region (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id INTEGER,
    FOREIGN KEY (parent_id) REFERENCES region(id)
);

CREATE TABLE image (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    user_id INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS protest (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    protest_date DATE NOT NULL,
    protest_time TIME WITHOUT TIME ZONE NOT NULL,
    location TEXT NOT NULL,
    region_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT now(),
    updated TIMESTAMP NOT NULL DEFAULT now(),
    deleted TIMESTAMP,
    image_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (region_id) REFERENCES region(id),
    FOREIGN KEY (image_id) REFERENCES image(id)
);

CREATE TABLE IF NOT EXISTS tag (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS protest_tag (
    protest_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (protest_id, tag_id),
    FOREIGN KEY (protest_id) REFERENCES protest(id),
    FOREIGN KEY (tag_id) REFERENCES tag(id)
);

CREATE TABLE IF NOT EXISTS notification (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    region_id INTEGER,
    days_before INTEGER,
    immediatelly BOOLEAN NOT NULL,
    created TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS notification_tag (
    notification_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (notification_id, tag_id),
    FOREIGN KEY (notification_id) REFERENCES notification(id),
    FOREIGN KEY (tag_id) REFERENCES tag(id)
);

INSERT INTO users (email, name, created) VALUES ('test@gmail.com', 'Test User', NOW());

INSERT INTO region (name) VALUES
    ('Banskobystrický kraj'),
    ('Bratislavský kraj'),
    ('Košický kraj'),
    ('Nitriansky kraj'),
    ('Prešovský kraj'),
    ('Trenčiansky kraj'),
    ('Trnavský kraj'),
    ('Žilinský kraj');

INSERT INTO region (parent_id, name) VALUES
    (1, 'Banská Bystrica'),
    (1, 'Hnúšťa'),
    (1, 'Lučenec'),
    (1, 'Rimavská Sobota'),
    (1, 'Veľký Krtíš'),
    (1, 'Žarnovica'),
    (1, 'Zvolen'),
    (2, 'Bratislava'),
    (2, 'Bratislava - ostatné'),
    (2, 'Malacky'),
    (2, 'Pezinok'),
    (2, 'Senec'),
    (3, 'Košice'),
    (3, 'Michalovce'),
    (3, 'Rožňava'),
    (3, 'Spišská Nová Ves'),
    (3, 'Trebišov'),
    (4, 'Komárno'),
    (4, 'Levice'),
    (4, 'Nitra'),
    (4, 'Nové Zámky'),
    (4, 'Šaľa'),
    (4, 'Topoľčany'),
    (5, 'Bardejov'),
    (5, 'Humenné'),
    (5, 'Poprad'),
    (5, 'Prešov'),
    (5, 'Spišské Podhradie'),
    (6, 'Bojnice'),
    (6, 'Dubnica nad Váhom'),
    (6, 'Handlová'),
    (6, 'Ilava'),
    (6, 'Myjava'),
    (6, 'Nové Mesto nad Váhom'),
    (6, 'Partizánske'),
    (6, 'Považská Bystrica'),
    (6, 'Prievidza'),
    (6, 'Púchov'),
    (6, 'Trenčín'),
    (7, 'Dunajská Streda'),
    (7, 'Galanta'),
    (7, 'Hlohovec'),
    (7, 'Piešťany'),
    (7, 'Šamorín'),
    (7, 'Senica'),
    (7, 'Trnava'),
    (7, 'Vrbové'),
    (8, 'Čadca'),
    (8, 'Liptovský Mikuláš'),
    (8, 'Martin'),
    (8, 'Námestovo'),
    (8, 'Žilina');