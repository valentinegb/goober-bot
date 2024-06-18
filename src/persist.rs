// Goober Bot, Discord bot
// Copyright (C) 2024  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::bail;
use serde::{de::DeserializeOwned, Serialize};
use shuttle_persist_msgpack::PersistError;

use crate::{Context, Error};

/// Loads a value from [`shuttle_persist_msgpack`] (if the value exists), or
/// saves the default of the value (if the value does not exist).
pub(crate) fn load_or_save_default<T>(ctx: Context<'_>, key: &str) -> Result<T, Error>
where
    T: DeserializeOwned + Serialize + Default,
{
    let data = ctx.data();

    Ok(match data.persist.load(key) {
        Ok(t) => t,
        Err(err) => match err {
            PersistError::Open(ref io_err) => match io_err.kind() {
                std::io::ErrorKind::NotFound => {
                    data.persist.save(key, T::default())?;

                    data.persist.load(key)?
                }
                _ => bail!(err),
            },
            _ => bail!(err),
        },
    })
}
