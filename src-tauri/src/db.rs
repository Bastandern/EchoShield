use rusqlite::{Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use tauri::api::path::app_data_dir;
use tauri::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct AudioFile {
    pub id: String,
    pub user_id: String,
    pub filename: String,
    pub created_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(config: &Config) -> Result<Self> {
        // 获取应用数据目录
        let app_data_dir = app_data_dir(config).expect("Failed to get app data dir");
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
        
        let db_path = app_data_dir.join("echoshield.db");
        let conn = Connection::open(db_path)?;

        // 创建用户表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // 创建音频文件表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audio_files (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn register_user(&self, username: &str, password: &str) -> Result<User> {
        // 检查用户名是否已存在
        if self.get_user_by_username(username)?.is_some() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
        
        let now = Utc::now().to_rfc3339();
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash,
            created_at: now.clone(),
            updated_at: now,
        };

        self.conn.execute(
            "INSERT INTO users (id, username, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &user.id,
                &user.username,
                &user.password_hash,
                &user.created_at,
                &user.updated_at,
            ),
        )?;

        Ok(user)
    }

    pub fn login_user(&self, username: &str, password: &str) -> Result<Option<User>> {
        if let Some(user) = self.get_user_by_username(username)? {
            let valid = verify(password.as_bytes(), &user.password_hash)
                .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

            if valid {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password_hash, created_at, updated_at 
             FROM users WHERE username = ?1"
        )?;

        let user = stmt.query_row([username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        });

        match user {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn save_audio_file(&self, user_id: &str, filename: &str) -> Result<AudioFile> {
        let now = Utc::now().to_rfc3339();
        let audio_file = AudioFile {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            filename: filename.to_string(),
            created_at: now,
        };

        self.conn.execute(
            "INSERT INTO audio_files (id, user_id, filename, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            (
                &audio_file.id,
                &audio_file.user_id,
                &audio_file.filename,
                &audio_file.created_at,
            ),
        )?;

        Ok(audio_file)
    }

    pub fn get_user_audio_files(&self, user_id: &str) -> Result<Vec<AudioFile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, filename, created_at 
             FROM audio_files 
             WHERE user_id = ?1
             ORDER BY created_at DESC"
        )?;

        let audio_files = stmt.query_map([user_id], |row| {
            Ok(AudioFile {
                id: row.get(0)?,
                user_id: row.get(1)?,
                filename: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        let mut result = Vec::new();
        for audio_file in audio_files {
            result.push(audio_file?);
        }

        Ok(result)
    }

    pub fn check_audio_file_access(&self, user_id: &str, filename: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) 
             FROM audio_files 
             WHERE user_id = ?1 AND filename = ?2"
        )?;

        let count: i64 = stmt.query_row([user_id, filename], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn rename_audio_file(&self, user_id: &str, old_filename: &str, new_filename: &str) -> Result<bool> {
        // 首先检查文件是否存在且属于该用户
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE user_id = ?1 AND filename = ?2",
            [user_id, old_filename],
            |row| row.get(0)
        )?;

        if count == 0 {
            return Ok(false);
        }

        // 检查新文件名是否已存在
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE user_id = ?1 AND filename = ?2",
            [user_id, new_filename],
            |row| row.get(0)
        )?;

        if count > 0 {
            return Ok(false);
        }

        // 更新文件名
        self.conn.execute(
            "UPDATE audio_files SET filename = ?1 WHERE user_id = ?2 AND filename = ?3",
            [new_filename, user_id, old_filename]
        )?;

        Ok(true)
    }

    pub fn delete_audio_file(&self, user_id: &str, filename: &str) -> Result<bool> {
        // 首先检查文件是否存在且属于该用户
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE user_id = ?1 AND filename = ?2",
            [user_id, filename],
            |row| row.get(0)
        )?;

        if count == 0 {
            return Ok(false);
        }

        // 删除文件记录
        self.conn.execute(
            "DELETE FROM audio_files WHERE user_id = ?1 AND filename = ?2",
            [user_id, filename]
        )?;

        Ok(true)
    }
} 