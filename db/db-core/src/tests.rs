/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! Test utilities
use crate::prelude::*;

/// test all database functions
pub async fn database_works<'a, T: MCDatabase>(db: &T, p: &Register<'a>) {
    assert!(db.ping().await, "ping test");
    if db.username_exists(p.username).await.unwrap() {
        db.delete_user(p.username).await.unwrap();
        assert!(
            !db.username_exists(p.username).await.unwrap(),
            "user is deleted so username shouldn't exsit"
        );
    }
    db.register(p).await.unwrap();

    assert_eq!(
        db.get_password(&Login::Username(p.username)).await.unwrap(),
        p.hash,
        "user password matches"
    );

    assert_eq!(
        db.get_password(&Login::Email(p.email.as_ref().unwrap()))
            .await
            .unwrap(),
        p.hash,
        "user password matches"
    );

    assert!(
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user is registered so email should exsit"
    );
    assert!(
        db.username_exists(p.username).await.unwrap(),
        "user is registered so username should exsit"
    );
    db.delete_user(p.username).await.unwrap();
    assert!(
        !db.username_exists(p.username).await.unwrap(),
        "user is deleted so username shouldn't exsit"
    );

    // register with email = None
    let mut p2 = p.clone();
    p2.email = None;
    db.register(&p2).await.unwrap();
    assert!(
        db.username_exists(p2.username).await.unwrap(),
        "user is registered so username should exsit"
    );
    assert!(
        !db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user registration with email is deleted; so email shouldn't exsit"
    );

    let update_email = UpdateEmail {
        username: p.username,
        new_email: p.email.as_ref().unwrap(),
    };

    db.update_email(&update_email).await.unwrap();
    println!(
        "null user email: {}",
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap()
    );
    assert!(
        db.email_exists(p.email.as_ref().unwrap()).await.unwrap(),
        "user was with empty email but email is set; so email should exsit"
    );
}
