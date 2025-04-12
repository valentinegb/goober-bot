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

#![allow(clippy::suspicious_doc_comments)]

use paste::paste;

// Hey,
// https://cdn.discordapp.com/emojis/<id>.webp?size=48&quality=lossless
// ;)

// in debug builds, uses emojis from Goober Bot Dev app
// in release builds, uses emojis from Goober Bot app

/// Creates an emoji `&str` constant for debug builds and another for release
/// builds.
///
/// # Examples
///
/// ```
/// emoji!("emojiName", "1234567890987654321" /* debug */, "1234567890987654321" /* release */);
/// emoji!("animatedEmoji", "1234567890987654321" /* debug */, "1234567890987654321" /* release */, true /* gif */);
/// ```
macro_rules! emoji {
    ($name:literal, $debug_id:literal, $release_id:literal, $format:literal, $prefix:literal) => {
        paste! {
            #[allow(clippy::suspicious_doc_comments)]
            #[doc = concat!("![](https://cdn.discordapp.com/emojis/", $debug_id, ".", $format, "?quality=lossless)")]
            #[cfg(debug_assertions)]
            pub const [<$name:snake:upper>]: &str = concat!("<", $prefix, ":", $name, ":", $debug_id, ">");
            #[doc = concat!("![](https://cdn.discordapp.com/emojis/", $release_id, ".", $format, "?quality=lossless)")]
            #[cfg(not(debug_assertions))]
            pub const [<$name:snake:upper>]: &str = concat!("<", $prefix, ":", $name, ":", $release_id, ">");
        }
    };
    ($name:literal, $debug_id:literal, $release_id:literal$(, false)?) => {
        emoji!($name, $debug_id, $release_id, "webp", "");
    };
    ($name:literal, $debug_id:literal, $release_id:literal, true) => {
        emoji!($name, $debug_id, $release_id, "gif", "a");
    };
}

#[rustfmt::skip]
emoji!("explosion", "1330040514405470311", "1330044953132400703", true);
emoji!("floof", "1263605435785810104", "1263609061539315722");
emoji!("floofAngry", "1263605462927016077", "1263609077661962331");
emoji!("floofBlep", "1263605485488308295", "1263609094791495724");
emoji!("floofCat", "1263605506593915053", "1263609111581560862");
emoji!("floofCool", "1263605526160474112", "1263609129683910761");
emoji!("floofCry", "1263605545852600393", "1263609147824410684");
emoji!("floofDrool", "1263605564035039323", "1263609166875066438");
emoji!("floofHappy", "1263605580380110890", "1263609184415383613");
emoji!("floofHeart", "1263605598524539001", "1263609201431675002");
#[rustfmt::skip]
emoji!("floofInnocent", "1263605617034006619", "1263609220725608519");
emoji!("floofLoad", "1263605636411949118", "1263609237762871336");
#[rustfmt::skip]
emoji!("floofLoadAnimated", "1263605189995266058", "1263609041179906059", true);
emoji!("floofLol", "1263605657886654495", "1263609255647510668");
emoji!("floofLurk", "1263605681420894258", "1263609272818729082");
#[rustfmt::skip]
emoji!("floofMischief", "1263605706733650041", "1263609299838697552");
emoji!("floofMug", "1263605736517271634", "1263609319555993792");
emoji!("floofNervous", "1263605768700301386", "1263609339013501008");
emoji!("floofNom", "1263605793710800897", "1263609382801903666");
emoji!("floofOwo", "1263605821338816583", "1263609400732418089");
emoji!("floofPat", "1263605857300643951", "1263609418214543371");
emoji!("floofPeek", "1263605875906449570", "1263609437726179479");
emoji!("floofPlead", "1263605895930052668", "1263609456760062072");
emoji!("floofSad", "1263605923188965428", "1263609478440288317");
emoji!("floofScared", "1263605944839831614", "1263609529820647544");
emoji!("floofSmug", "1263605963877912577", "1263609552356773971");
emoji!("floofTeehee", "1263605984115560478", "1263609577815933061");
emoji!("floofTired", "1263606003082199131", "1263609597382496308");
emoji!("floofWhat", "1263606024892321945", "1263609615036190821");
emoji!("floofWoozy", "1263606042840010752", "1263609632333762592");
emoji!("iAmTheLaw", "1360451308770955275", "1360450936056840313");
