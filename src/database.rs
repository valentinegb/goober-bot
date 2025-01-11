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

use serde::{de::DeserializeOwned, Serialize};

use crate::Context;

/// Reads a value from [`shuttle_shared_db::SerdeJsonOperator`] (if the value
/// exists), or writes the default of the value (if the value does not exist).
pub(crate) async fn read_or_write_default<T>(
    ctx: Context<'_>,
    key: &str,
) -> Result<T, opendal::Error>
where
    T: DeserializeOwned + Serialize + Default,
{
    let data = ctx.data();

    match data.op.read_serialized(key).await {
        Ok(t) => Ok(t),
        Err(err) => match err.kind() {
            opendal::ErrorKind::NotFound => {
                data.op.write_serialized(key, &T::default()).await?;

                Ok(data.op.read_serialized(key).await?)
            }
            _ => Err(err),
        },
    }
}
