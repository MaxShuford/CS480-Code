use crate::error::AppResult;
use crate::error::Error::*;
use crate::structs::*;
use mysql::prelude::*;
use mysql::*;

//function to log the user in
pub fn login(input: User) -> AppResult<i64> {
    let url = "mysql://root:root@localhost:3306/mooglegaps";
    let pool = Pool::new(url).expect("Failed to create database pool");
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    //get user id and password from db
    let row: Option<(i64, String)> = conn.exec_first(
        "SELECT user_id, `password` 
        FROM `user` 
        WHERE username = :usrnm;",
        params! {
            "usrnm" => &input.username
        },
    )?;

    //unpack option
    if let Some((user_id, db_pw)) = row {
        //compare passwords
        if db_pw == input.pw_hash {
            //return user id
            return Ok(user_id);
        }
    }

    //if something errors
    Err(LoginFailed {
        username: input.username,
    })
}

//function to create account
pub fn create_account(conn: &mut PooledConn, input: User) -> AppResult<i64> {
    //query database for account
    let account: Option<i64> = conn.exec_first(
        "SELECT user_id FROM `user` WHERE username = :usrnm;",
        params! {
            "usrnm" => &input.username
        },
    )?;

    //if the account exists, we throw an error
    if account.is_some() {
        return Err(UserExists {
            username: input.username,
        });
    }

    //since account doesnt exist, create user
    conn.exec_drop(
        "INSERT INTO `user` (username, `password`) 
        VALUES (:usrnm, :pw);",
        params! {
            "usrnm" => &input.username,
            "pw" => &input.pw_hash
        },
    )?;

    //get user id
    let id = conn.last_insert_id() as i64;
    //return user id
    Ok(id)
}

//function to change password
pub fn change_pass(conn: &mut PooledConn, input: ChangePassword) -> AppResult<i32> {
    //get password from db
    let db_pw: Option<String> = conn.exec_first(
        "SELECT `password` 
        FROM `user` 
        WHERE user_id = :uid;",
        params! {
            "uid" => &input.uuid
        },
    )?;

    //unpack option
    if let Some(current_pw) = db_pw {
        //compare passwords
        if current_pw == input.old_pw {
            //set new password
            conn.exec_drop(
                "UPDATE `user` 
                SET `password` = :new_pass 
                WHERE user_id = :uid;",
                params! {
                    "new_pass" => &input.new_pw,
                    "uid" => &input.uuid
                },
            )?;

            //return success
            return Ok(1);
        }
    }

    //return error if user not found or incorrect password
    Err(IncorrectPassword)
}

