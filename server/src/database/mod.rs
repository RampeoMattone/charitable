// mariadb/mysql interface for the bot.
use mysql::prelude::*;
use mysql::*;

pub struct Server {
    id: u64,
    mod_channel: u64,
    pub_channel: u64,
    //available: bool,
}

impl Server {
    pub fn new(id: u64, mod_channel: u64, pub_channel: u64) -> Server {
        Server {
            id,
            mod_channel,
            pub_channel,
        }
    }
    pub fn id(&self) -> u64 {self.id}
    pub fn mod_channel(&self) -> u64 {self.mod_channel}
    pub fn pub_channel(&self) -> u64 {self.pub_channel}
    //pub fn available(&self) -> bool {self.available}
}

pub struct User<'a> {
    id: u64,
    username: &'a str,
}

impl<'a> User<'a> {
    pub fn new(id: u64, username: &'a str) -> User<'a> {
        User { id, username }
    }
    pub fn id(&self) -> u64 {self.id}
    pub fn username(&self) -> &str {self.username}
}

pub struct Database {
    conn: PooledConn,
}

impl Database {
    pub fn connect(url: &str) -> Database {
        let pool = Pool::new(url).unwrap();
        Database {
            conn: pool.get_conn().unwrap(),
        }
    }
    // aggiunge o modifica (se duplicato) un record utente
    pub fn user_sync(&mut self, user: User) -> Result<()> {
        self.conn.exec_drop(
            r"INSERT INTO user (id, username)
                        VALUES (:id, :username)
                        ON DUPLICATE KEY UPDATE
                        username = :username,
                        timestamp = current_timestamp();",
            params! {
            "id" => user.id,
            "username" => user.username,
            },
        )?;
        Ok(())
    }
    // come user_sync ma con supporto a vettori
    pub fn user_batch_sync(&mut self, vec: Vec<User>) -> Result<()> {
        self.conn.exec_batch(
            r"INSERT INTO user (id, username)
                        VALUES (:id, :username)
                        ON DUPLICATE KEY UPDATE
                        username = :username,
                        timestamp = current_timestamp()
                        ;",
            vec.iter().map(|user| {
                params! {
                "id" => user.id,
                "username" => user.username,
                }
            }),
        )?;
        Ok(())
    }
    // aggiunge o modifica (se duplicato) un record server
    pub fn server_sync(&mut self, server: Server) -> Result<()> {
        self.conn.exec_drop(
            r"INSERT INTO server (id, mod_channel, pub_channel)
            VALUES (:id, :mod_channel, :pub_channel)
            ON DUPLICATE KEY UPDATE
            mod_channel = :mod_channel,
            pub_channel = :pub_channel;",
            params! {
            "id" => server.id,
            "mod_channel" => server.mod_channel,
            "pub_channel" => server.pub_channel,
            },
        )?;
        Ok(())
    }
    // agiunge o aggiorna una relazione tra utente e server.
    pub fn user_server_state_sync(&mut self, user_id: u64, server_id: u64, enabled: bool) -> Result<()> {
        self.conn.exec_drop(
            r"INSERT INTO user_to_server (user_id, server_id, sequence)
                        VALUES (:user_id, :server_id,
                            (SELECT count(if(thx_tipo.user_id = :user_id, 1, NULL))
                            FROM user_to_server AS thx_tipo))
                        ON DUPLICATE KEY UPDATE
                            enabled = :enabled;",
            params! (user_id, server_id, enabled),
        )?;
        Ok(())
    }
    // come la precedente ma viene utilizzata per sincronizzare un gruppo di utenti attivi.
    // se non si fa parte del gruppo si viene segnati come bloccati
    pub fn user_server_state_batch_sync(&mut self, active: Vec<(u64, u64)>) -> Result<()> {
        self.conn.query_drop(r"DROP TABLE IF EXISTS temp_user_to_server;
        CREATE TEMPORARY TABLE temp_user_to_server (
            user_id BIGINT UNSIGNED NOT NULL,
            server_id BIGINT UNSIGNED NOT NULL,
            constraint user_to_server_id_combo_pk  primary key (user_id, server_id)
            );")?;
        self.conn.exec_batch(r"INSERT INTO
        temp_user_to_server (user_id, server_id)
        VALUES (:user_id, :server_id);", active.iter().map(|p| params! {
            "user_id" => p.0,
            "server_id" => p.1,
        }))?;
        self.conn.query_drop("UPDATE user_to_server \
            SET enabled = IF((user_id, server_id) NOT IN (SELECT user_id, server_id FROM temp_user_to_server), false, true) \
            WHERE TRUE; \
            INSERT INTO user_to_server (user_id, server_id) \
            SELECT user_id, server_id FROM temp_user_to_server \
            WHERE (user_id, server_id) NOT IN (SELECT user_id, server_id FROM user_to_server);")?;
        Ok(())
    }
    // se il server diventa unreachable o la sua config è invalidata viene chiamata la funzione
    pub fn server_available_sync(&mut self, server_id: u64, available: bool) -> Result<()> {
        self.conn.exec_drop(
            r"UPDATE server
                            SET available = :available
                            WHERE id = :server_id;",
            params! (available, server_id),
        )?;
        Ok(())
    }
    // aggiunge un messaggio in input. WARNING: errors out if duplicate entry
    pub fn msg_in_add(&mut self, id: u64, author: u64, message: String) -> Result<()> {
        self.conn.exec_drop(
            r"INSERT INTO inbox (id, author, message)
                        VALUES (:id, :author, :message)",
            params! (id, author, message),
        )?;
        Ok(())
    }
    // aggiunge o modifica un messaggio in out.
    pub fn msg_out_sync(&mut self,
                        id: u64,
                        inbox_id: u64,
                        server_id: u64,
                        mod_id: u64,
                        pub_id: Option<u64>,
                        rejected: bool) -> Result<()> {
        self.conn.exec_drop(
            r"INSERT INTO outbox (id, inbox_id, server_id, mod_id, pub_id, rejected)
                        VALUES (:id, :inbox_id, :server_id, :mod_id, :pub_id, :rejected)
                        ON DUPLICATE KEY UPDATE
                            pub_id = :pub_id,
                            rejected = :rejected;",
            params! (id, inbox_id, server_id, mod_id, pub_id, rejected),
        )?;
        Ok(())
    }
    // trova l'ultimo messaggio mandato da un utente
    pub fn user_last_msg(&mut self, author: u64) -> Result<Option<u64>> {
        self.conn.exec_first(r"SELECT id
                                    FROM inbox
                                    WHERE author = :author
                                    ORDER BY timestamp DESC LIMIT 1;",
                             params!(author))
    }
    // restituisce i server con cui l'utente è relazionato, assieme allo stato della relazione
    pub fn user_server_state_get(&mut self, user_id: u64) -> Result<Vec<(u64, bool)>> {
        self.conn.exec("SELECT server_id, \
                                    IF(enabled, \
                                        IF((SELECT available FROM server WHERE id = server_id), 1, 0), \
                                        0) \
                                FROM user_to_server \
                                WHERE user_id = :user_id \
                                ORDER BY id;",
                       params!(user_id))
    }
}