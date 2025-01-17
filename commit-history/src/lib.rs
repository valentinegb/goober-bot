// Goober Bot, Discord bot
// Copyright (C) 2025  Valentine Briese
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

use git2::Repository;
use proc_macro::TokenStream;

#[proc_macro]
pub fn commit_history(_item: TokenStream) -> TokenStream {
    let repo = Repository::open("./").unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    let mut string = String::new();

    revwalk.push_head().unwrap();

    for (i, oid) in revwalk.take(10).enumerate() {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        if i == 0 {
            string += &format!("The last change was <t:{}:R>.\n", commit.time().seconds());
        }

        string += &format!(
            "\n[{}](https://github.com/valentinegb/goober-bot/commit/{}): {}",
            &oid.to_string()[..7],
            oid,
            commit.message().unwrap().lines().next().unwrap(),
        );
    }

    format!("{string:#?}").parse().unwrap()
}
